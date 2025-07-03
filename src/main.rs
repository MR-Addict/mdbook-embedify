mod assets;
mod cli;
mod embed;
mod parser;
mod utils;

use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::io;

#[macro_use]
extern crate pest_derive;

fn main() {
    let cli = cli::Cli::new();
    let embed = embed::Embed::new();

    // reply --supports command line argument
    cli.reply_supports(&embed);

    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin()).unwrap();
    let result = embed.run(&ctx, book).unwrap();

    // Write the result to stdout
    serde_json::to_writer(io::stdout(), &result).unwrap();
}

#[cfg(test)]
mod tests {
    use std::{
        path::Path,
        process::{Command, Stdio},
    };

    fn cd_to_crate_home() {
        let path = std::env::var("CARGO_MANIFEST_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap());

        std::env::set_current_dir(path).unwrap();
    }

    fn build() {
        cd_to_crate_home();

        let mut cmd = Command::new("cargo");

        cmd.arg("build").arg("-q");
        let output = cmd.output().unwrap();
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert_eq!(stdout, "");
        assert_eq!(stderr, "");
    }

    #[test]
    fn test_nothing() {
        build();
        let mut cmd = Command::new("./target/debug/mdbook-embedify");
        cmd.stdin(Stdio::inherit());
        let output = cmd.output().unwrap();
        assert_eq!(output.status.code().unwrap(), 1);
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stdout.contains("A mdbook embed preprocessor that embeds app to your book"));
        assert_eq!(stderr, "");
    }

    #[test]
    fn test_case_a() {
        build();
        std::env::set_current_dir("test_books/a").unwrap();
        let mut cmd = Command::new("mdbook");
        cmd.arg("build");
        cmd.stdin(Stdio::inherit());
        let output = cmd.output().unwrap();
        assert_eq!(output.status.code().unwrap(), 0);
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert_eq!(stdout, "");
        assert!(stderr.contains("Book building has started"));

        let cmp = dircmp::Comparison::default();

        let result = cmp
            .compare(Path::new("book"), Path::new("expected"))
            .expect("should be able to compare directories");
        println!("Result: {:?}", result);
        assert!(result.is_empty());
    }
}
