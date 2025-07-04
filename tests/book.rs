use dircmp::Comparison;
use std::path::Path;
use std::process::Command;

/// Test that verifies the mdbook-embedify preprocessor generates the expected output
/// by comparing the generated book directory with a pre-defined expected directory.
///
/// This test:
/// 1. Runs `mdbook build tests/test-book` to generate the book output
/// 2. Uses `dircmp` to compare the generated `book/` directory with `expected/`
/// 3. Fails if there are any differences in file structure or content
#[test]
fn test_basic_apps() {
    let test_book_path = "tests/test-book";
    let book_output_path = "tests/test-book/book";
    let expected_path = "tests/test-book/expected";

    // Ensure the test-book directory exists
    assert!(
        Path::new(test_book_path).exists(),
        "Test book directory not found: {}",
        test_book_path
    );

    // Ensure the expected directory exists
    assert!(
        Path::new(expected_path).exists(),
        "Expected directory not found: {}",
        expected_path
    );

    // Clean any existing book output
    if Path::new(book_output_path).exists() {
        std::fs::remove_dir_all(book_output_path)
            .expect("Failed to clean existing book output directory");
    }

    // Run mdbook build on the test book
    let output = Command::new("mdbook")
        .args(&["build", test_book_path])
        .output()
        .expect("Failed to execute mdbook build command");

    // Check if the build was successful
    if !output.status.success() {
        eprintln!(
            "mdbook build stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        eprintln!(
            "mdbook build stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!(
            "mdbook build failed with exit code: {:?}",
            output.status.code()
        );
    }

    // Ensure the book output directory was created
    assert!(
        Path::new(book_output_path).exists(),
        "Book output directory was not created: {}",
        book_output_path
    );

    // Compare the generated book with the expected output using dircmp
    let comparison = Comparison::default();
    let diff = comparison
        .compare(book_output_path, expected_path)
        .expect("Failed to compare directories");

    // Check if directories are identical - use the fields to determine if they match
    let has_differences = !diff.missing_right.is_empty()
        || !diff.missing_left.is_empty()
        || !diff.changed.is_empty()
        || !diff.different_type.is_empty();

    if has_differences {
        eprintln!("Directory comparison found differences:");

        // Print files only in the generated book (missing in expected)
        if !diff.missing_right.is_empty() {
            eprintln!("Files only in generated book (missing from expected):");
            for file in &diff.missing_right {
                eprintln!("  - {}", file.display());
            }
        }

        // Print files only in expected (missing in generated)
        if !diff.missing_left.is_empty() {
            eprintln!("Files only in expected (missing from generated):");
            for file in &diff.missing_left {
                eprintln!("  + {}", file.display());
            }
        }

        // Print files with different content
        if !diff.changed.is_empty() {
            eprintln!("Files with different content:");
            for file in &diff.changed {
                eprintln!("  ~ {}", file.display());
            }
        }

        // Print files with different types
        if !diff.different_type.is_empty() {
            eprintln!("Files with different types:");
            for file in &diff.different_type {
                eprintln!("  ! {}", file.display());
            }
        }

        panic!("Generated book does not match expected output");
    }

    println!("✓ Generated book matches expected output perfectly!");
}

#[cfg(test)]
mod cleanup {
    use super::*;

    /// Helper function to clean up test artifacts
    pub fn clean_test_artifacts() {
        let book_output_path = "tests/test-book/book";
        if Path::new(book_output_path).exists() {
            let _ = std::fs::remove_dir_all(book_output_path);
        }
    }

    #[test]
    fn cleanup_after_tests() {
        // This test runs last (alphabetically) to clean up
        clean_test_artifacts();
    }
}
