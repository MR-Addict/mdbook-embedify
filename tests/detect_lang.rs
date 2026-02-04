use mdbook_embedify::detect_lang;

fn detect_lang(path: String) -> String {
    detect_lang::detect_lang(path, None)
}

#[cfg(test)]
mod basic_language_detection_tests {
    use super::*;

    #[test]
    fn test_detect_rust_language() {
        assert_eq!(detect_lang("src/main.rs".to_string()), "rust");
        assert_eq!(detect_lang("lib.rs".to_string()), "rust");
        assert_eq!(detect_lang("/path/to/file.rs".to_string()), "rust");
    }

    #[test]
    fn test_detect_javascript_language() {
        assert_eq!(detect_lang("script.js".to_string()), "javascript");
        assert_eq!(detect_lang("component.jsx".to_string()), "javascript");
        assert_eq!(detect_lang("src/utils.js".to_string()), "javascript");
        assert_eq!(
            detect_lang("/Users/user/project/main.js".to_string()),
            "javascript"
        );
    }

    #[test]
    fn test_detect_python_language() {
        assert_eq!(detect_lang("script.py".to_string()), "python");
        assert_eq!(detect_lang("main.py".to_string()), "python");
        assert_eq!(detect_lang("src/utils.py".to_string()), "python");
        assert_eq!(detect_lang("/path/to/script.py".to_string()), "python");
    }

    #[test]
    fn test_detect_html_language() {
        assert_eq!(detect_lang("index.html".to_string()), "html");
        assert_eq!(detect_lang("template.htm".to_string()), "html");
        assert_eq!(detect_lang("page.xhtml".to_string()), "html");
        assert_eq!(detect_lang("/var/www/index.html".to_string()), "html");
    }

    #[test]
    fn test_detect_css_language() {
        assert_eq!(detect_lang("styles.css".to_string()), "css");
        assert_eq!(detect_lang("main.css".to_string()), "css");
        assert_eq!(detect_lang("src/components/button.css".to_string()), "css");
    }

    #[test]
    fn test_detect_json_language() {
        assert_eq!(detect_lang("package.json".to_string()), "json");
        assert_eq!(detect_lang("config.json".to_string()), "json");
        assert_eq!(detect_lang("data.geojson".to_string()), "json");
        assert_eq!(detect_lang("/etc/config.json".to_string()), "json");
    }

    #[test]
    fn test_detect_markdown_language() {
        assert_eq!(detect_lang("README.md".to_string()), "markdown");
        assert_eq!(detect_lang("docs.md".to_string()), "markdown");
        assert_eq!(detect_lang("CHANGELOG.markdown".to_string()), "markdown");
        assert_eq!(detect_lang("file.mdown".to_string()), "markdown");
    }

    #[test]
    fn test_detect_yaml_language() {
        assert_eq!(detect_lang("config.yml".to_string()), "yaml");
        assert_eq!(detect_lang("docker-compose.yaml".to_string()), "yaml");
        assert_eq!(detect_lang(".github/workflows/ci.yml".to_string()), "yaml");
    }

    #[test]
    fn test_detect_toml_language() {
        assert_eq!(detect_lang("Cargo.toml".to_string()), "toml");
        assert_eq!(detect_lang("pyproject.toml".to_string()), "toml");
        assert_eq!(detect_lang("config.toml".to_string()), "toml");
    }

    #[test]
    fn test_detect_xml_language() {
        assert_eq!(detect_lang("config.xml".to_string()), "xml");
        assert_eq!(detect_lang("pom.xml".to_string()), "xml");
        assert_eq!(detect_lang("build.xml".to_string()), "xml");
    }
}

#[cfg(test)]
mod compiled_languages_tests {
    use super::*;

    #[test]
    fn test_detect_c_language() {
        assert_eq!(detect_lang("main.c".to_string()), "c");
        assert_eq!(detect_lang("header.h".to_string()), "c");
    }

    #[test]
    fn test_detect_cpp_language() {
        assert_eq!(detect_lang("main.cpp".to_string()), "cpp");
        assert_eq!(detect_lang("class.cxx".to_string()), "cpp");
        assert_eq!(detect_lang("utils.cc".to_string()), "cpp");
        assert_eq!(detect_lang("header.hpp".to_string()), "cpp");
    }

    #[test]
    fn test_detect_java_language() {
        assert_eq!(detect_lang("Main.java".to_string()), "java");
        assert_eq!(detect_lang("MyClass.java".to_string()), "java");
        assert_eq!(detect_lang("src/com/example/Main.java".to_string()), "java");
    }

    #[test]
    fn test_detect_csharp_language() {
        assert_eq!(detect_lang("Program.cs".to_string()), "csharp");
        assert_eq!(detect_lang("MyClass.cs".to_string()), "csharp");
        assert_eq!(detect_lang("src/Program.cs".to_string()), "csharp");
    }

    #[test]
    fn test_detect_go_language() {
        assert_eq!(detect_lang("main.go".to_string()), "go");
        assert_eq!(detect_lang("utils.go".to_string()), "go");
        assert_eq!(detect_lang("src/server.go".to_string()), "go");
    }

    #[test]
    fn test_detect_kotlin_language() {
        assert_eq!(detect_lang("Main.kt".to_string()), "kotlin");
        assert_eq!(detect_lang("Utils.kts".to_string()), "kotlin");
        assert_eq!(detect_lang("src/main/kotlin/Main.kt".to_string()), "kotlin");
    }
}

#[cfg(test)]
mod shell_and_scripting_tests {
    use super::*;

    #[test]
    fn test_detect_bash_language() {
        assert_eq!(detect_lang("script.sh".to_string()), "shell");
        assert_eq!(detect_lang("setup.zsh".to_string()), "shell");
        assert_eq!(detect_lang("bin/deploy.sh".to_string()), "shell");
    }

    #[test]
    fn test_detect_lua_language() {
        assert_eq!(detect_lang("script.lua".to_string()), "lua");
        assert_eq!(detect_lang("config.lua".to_string()), "lua");
        assert_eq!(detect_lang("init.lua".to_string()), "lua");
    }

    #[test]
    fn test_detect_php_language() {
        assert_eq!(detect_lang("index.php".to_string()), "php");
        assert_eq!(detect_lang("config.php".to_string()), "php");
        assert_eq!(detect_lang("src/Utils.php".to_string()), "php");
    }
}

#[cfg(test)]
mod edge_cases_and_fallback_tests {
    use super::*;

    #[test]
    fn test_no_extension_returns_plaintext() {
        assert_eq!(detect_lang("README".to_string()), "plaintext");
        assert_eq!(detect_lang("LICENSE".to_string()), "plaintext");
        assert_eq!(detect_lang("CHANGELOG".to_string()), "plaintext");
        assert_eq!(detect_lang("unknown_file".to_string()), "plaintext");
    }

    #[test]
    fn test_unknown_extension_returns_plaintext() {
        assert_eq!(detect_lang("file.unknown".to_string()), "plaintext");
        assert_eq!(detect_lang("data.xyz".to_string()), "plaintext");
        assert_eq!(detect_lang("temp.temp123".to_string()), "plaintext");
    }

    #[test]
    fn test_empty_string_returns_plaintext() {
        assert_eq!(detect_lang("".to_string()), "plaintext");
    }

    #[test]
    fn test_just_extension_returns_plaintext() {
        assert_eq!(detect_lang(".".to_string()), "plaintext");
        assert_eq!(detect_lang("..".to_string()), "plaintext");
    }

    #[test]
    fn test_hidden_files_with_extensions() {
        assert_eq!(detect_lang(".gitignore".to_string()), "plaintext");
        assert_eq!(detect_lang(".env".to_string()), "plaintext");
        assert_eq!(detect_lang(".hidden.js".to_string()), "javascript");
        assert_eq!(detect_lang(".config.json".to_string()), "json");
    }

    #[test]
    fn test_case_insensitive_extensions() {
        assert_eq!(detect_lang("file.JS".to_string()), "javascript");
        assert_eq!(detect_lang("file.RS".to_string()), "rust");
        assert_eq!(detect_lang("file.HTML".to_string()), "html");
        assert_eq!(detect_lang("file.Py".to_string()), "python");
        assert_eq!(detect_lang("file.CSS".to_string()), "css");
    }

    #[test]
    fn test_multiple_dots_in_filename() {
        assert_eq!(detect_lang("my.backup.file.js".to_string()), "javascript");
        assert_eq!(detect_lang("config.local.json".to_string()), "json");
        assert_eq!(detect_lang("test.spec.ts".to_string()), "typescript");
        assert_eq!(detect_lang("component.test.js".to_string()), "javascript");
    }
}

#[cfg(test)]
mod complex_extension_tests {
    use super::*;

    #[test]
    fn test_typescript_extensions() {
        assert_eq!(detect_lang("app.ts".to_string()), "typescript");
        assert_eq!(detect_lang("component.tsx".to_string()), "typescript");
        assert_eq!(detect_lang("types.d.ts".to_string()), "typescript");
    }

    #[test]
    fn test_various_web_extensions() {
        assert_eq!(detect_lang("styles.scss".to_string()), "scss");
        assert_eq!(detect_lang("styles.sass".to_string()), "plaintext"); // sass not in language map
        assert_eq!(detect_lang("styles.less".to_string()), "less");
        assert_eq!(detect_lang("template.vue".to_string()), "plaintext"); // vue not in language map
    }
}

#[cfg(test)]
mod path_handling_tests {
    use super::*;

    #[test]
    fn test_absolute_paths() {
        assert_eq!(detect_lang("/home/user/main.rs".to_string()), "rust");
        assert_eq!(detect_lang("/var/www/html/index.html".to_string()), "html");
        assert_eq!(detect_lang("/etc/config/app.json".to_string()), "json");
    }

    #[test]
    fn test_relative_paths() {
        assert_eq!(detect_lang("./src/main.rs".to_string()), "rust");
        assert_eq!(detect_lang("../config.json".to_string()), "json");
        assert_eq!(detect_lang("../../utils.js".to_string()), "javascript");
    }

    #[test]
    fn test_windows_paths() {
        assert_eq!(detect_lang("C:\\Users\\user\\main.rs".to_string()), "rust");
        assert_eq!(detect_lang("D:\\web\\index.html".to_string()), "html");
        assert_eq!(detect_lang("..\\config\\app.json".to_string()), "json");
    }

    #[test]
    fn test_paths_with_spaces() {
        assert_eq!(detect_lang("my project/main file.rs".to_string()), "rust");
        assert_eq!(detect_lang("/web site/index.html".to_string()), "html");
        assert_eq!(detect_lang("app config.json".to_string()), "json");
    }

    #[test]
    fn test_paths_with_special_characters() {
        assert_eq!(detect_lang("project-name/main.rs".to_string()), "rust");
        assert_eq!(detect_lang("my_project/utils.js".to_string()), "javascript");
        assert_eq!(detect_lang("app@version/config.json".to_string()), "json");
    }
}

#[cfg(test)]
mod real_world_filenames_tests {
    use super::*;

    #[test]
    fn test_common_config_files() {
        assert_eq!(detect_lang("package.json".to_string()), "json");
        assert_eq!(detect_lang("tsconfig.json".to_string()), "json");
        assert_eq!(detect_lang("Cargo.toml".to_string()), "toml");
        assert_eq!(detect_lang("pyproject.toml".to_string()), "toml");
        assert_eq!(detect_lang("docker-compose.yml".to_string()), "yaml");
        assert_eq!(detect_lang(".eslintrc.js".to_string()), "javascript");
    }

    #[test]
    fn test_build_and_ci_files() {
        assert_eq!(detect_lang("webpack.config.js".to_string()), "javascript");
        assert_eq!(detect_lang("rollup.config.js".to_string()), "javascript");
        assert_eq!(detect_lang("vite.config.ts".to_string()), "typescript");
        assert_eq!(detect_lang("jest.config.js".to_string()), "javascript");
        assert_eq!(detect_lang(".github/workflows/ci.yml".to_string()), "yaml");
    }

    #[test]
    fn test_documentation_files() {
        assert_eq!(detect_lang("README.md".to_string()), "markdown");
        assert_eq!(detect_lang("CHANGELOG.md".to_string()), "markdown");
        assert_eq!(detect_lang("docs/api.md".to_string()), "markdown");
        assert_eq!(detect_lang("book.toml".to_string()), "toml");
    }

    #[test]
    fn test_test_files() {
        assert_eq!(detect_lang("main_test.go".to_string()), "go");
        assert_eq!(detect_lang("utils.test.js".to_string()), "javascript");
        assert_eq!(detect_lang("component.spec.ts".to_string()), "typescript");
        assert_eq!(detect_lang("test_utils.py".to_string()), "python");
    }

    #[test]
    fn test_data_files() {
        assert_eq!(detect_lang("data.csv".to_string()), "csv");
        assert_eq!(detect_lang("config.ini".to_string()), "ini");
        assert_eq!(detect_lang("database.sql".to_string()), "sql");
        assert_eq!(detect_lang("query.graphql".to_string()), "graphql");
    }
}

#[cfg(test)]
mod filename_based_detection_tests {
    use super::*;

    #[test]
    fn test_makefile_detection() {
        assert_eq!(detect_lang("Makefile".to_string()), "makefile");
        assert_eq!(detect_lang("makefile".to_string()), "makefile");
        assert_eq!(detect_lang("GNUmakefile".to_string()), "makefile");
        assert_eq!(detect_lang("Makefile.am".to_string()), "makefile");
        assert_eq!(detect_lang("Makefile.in".to_string()), "makefile");
    }

    #[test]
    fn test_dockerfile_detection() {
        assert_eq!(detect_lang("Dockerfile".to_string()), "dockerfile");
        assert_eq!(detect_lang("dockerfile".to_string()), "dockerfile");
        assert_eq!(detect_lang("Containerfile".to_string()), "dockerfile");
    }

    #[test]
    fn test_ruby_specific_files() {
        assert_eq!(detect_lang("Rakefile".to_string()), "ruby");
        assert_eq!(detect_lang("Gemfile".to_string()), "ruby");
        assert_eq!(detect_lang("Guardfile".to_string()), "ruby");
        assert_eq!(detect_lang("Capfile".to_string()), "ruby");
        assert_eq!(detect_lang("Vagrantfile".to_string()), "ruby");
    }

    #[test]
    fn test_python_specific_files() {
        assert_eq!(detect_lang("SConstruct".to_string()), "python");
        assert_eq!(detect_lang("SConscript".to_string()), "python");
    }

    #[test]
    fn test_shell_config_files() {
        assert_eq!(detect_lang(".bashrc".to_string()), "shell");
        assert_eq!(detect_lang(".zshrc".to_string()), "shell");
        assert_eq!(detect_lang(".profile".to_string()), "shell");
        assert_eq!(detect_lang(".bash_profile".to_string()), "shell");
        assert_eq!(detect_lang(".zprofile".to_string()), "shell");
    }

    #[test]
    fn test_filename_takes_precedence_over_extension() {
        // Exact filename match takes precedence
        assert_eq!(detect_lang("Makefile".to_string()), "makefile"); // exact filename match

        // Pattern match takes precedence over unknown extension
        assert_eq!(detect_lang("Makefile.bak".to_string()), "makefile"); // matches Makefile.* pattern

        // But known extension takes precedence over pattern
        assert_eq!(detect_lang("Makefile.js".to_string()), "javascript"); // .js extension takes precedence
    }
}

#[cfg(test)]
mod wildcard_pattern_tests {
    use super::*;

    #[test]
    fn test_dockerfile_wildcard_patterns() {
        // Test Dockerfile* pattern
        assert_eq!(detect_lang("Dockerfile".to_string()), "dockerfile");
        assert_eq!(detect_lang("Dockerfile.base".to_string()), "dockerfile");
        assert_eq!(detect_lang("Dockerfile.prod".to_string()), "dockerfile");
        assert_eq!(detect_lang("Dockerfile.dev".to_string()), "dockerfile");
        assert_eq!(detect_lang("Dockerfile.test".to_string()), "dockerfile");

        // Test dockerfile* pattern
        assert_eq!(detect_lang("dockerfile".to_string()), "dockerfile");
        assert_eq!(detect_lang("dockerfile.base".to_string()), "dockerfile");
        assert_eq!(detect_lang("dockerfile.prod".to_string()), "dockerfile");

        // Test *.dockerfile pattern
        assert_eq!(detect_lang("base.dockerfile".to_string()), "dockerfile");
        assert_eq!(detect_lang("prod.dockerfile".to_string()), "dockerfile");
        assert_eq!(detect_lang("app.dockerfile".to_string()), "dockerfile");
        assert_eq!(detect_lang("web.dockerfile".to_string()), "dockerfile");
    }

    #[test]
    fn test_wildcard_patterns_case_sensitivity() {
        // Should match case-sensitive patterns
        assert_eq!(detect_lang("Dockerfile.base".to_string()), "dockerfile");
        assert_eq!(detect_lang("dockerfile.base".to_string()), "dockerfile");

        // Should not match different case for the base name
        assert_eq!(detect_lang("DOCKERFILE.base".to_string()), "plaintext");
        assert_eq!(detect_lang("DockerFile.base".to_string()), "plaintext");
    }

    #[test]
    fn test_wildcard_patterns_precedence() {
        // Exact filename match should take precedence over patterns
        assert_eq!(detect_lang("Dockerfile".to_string()), "dockerfile");
        assert_eq!(detect_lang("dockerfile".to_string()), "dockerfile");
        assert_eq!(detect_lang("Containerfile".to_string()), "dockerfile");

        // Pattern match should work for non-exact matches
        assert_eq!(detect_lang("Dockerfile.custom".to_string()), "dockerfile");
        assert_eq!(detect_lang("my.dockerfile".to_string()), "dockerfile");
    }

    #[test]
    fn test_extension_vs_pattern_precedence() {
        // Extension should still take precedence over patterns if it's a valid extension
        // For example, if we had a file called "Dockerfile.js", it should be detected as javascript
        assert_eq!(detect_lang("Dockerfile.js".to_string()), "javascript");
        assert_eq!(detect_lang("dockerfile.py".to_string()), "python");

        // But if the extension is not recognized, pattern matching should work
        assert_eq!(detect_lang("Dockerfile.custom".to_string()), "dockerfile");
        assert_eq!(detect_lang("dockerfile.myext".to_string()), "dockerfile");
    }

    #[test]
    fn test_no_false_positives() {
        // Files that start with dockerfile but don't match patterns should not be detected
        assert_eq!(detect_lang("dockerfiles".to_string()), "plaintext");
        assert_eq!(detect_lang("Dockerfiles".to_string()), "plaintext");
        assert_eq!(detect_lang("dockerfile_backup".to_string()), "plaintext");
        assert_eq!(detect_lang("Dockerfile_backup".to_string()), "plaintext");

        // Files that end with dockerfile but don't match patterns should not be detected
        assert_eq!(detect_lang("notadockerfile".to_string()), "plaintext");
        assert_eq!(detect_lang("backup_dockerfile".to_string()), "plaintext");

        // Files that contain dockerfile but don't match patterns
        assert_eq!(detect_lang("my_dockerfile_backup".to_string()), "plaintext");
    }

    #[test]
    fn test_manual_demonstration() {
        // Demonstrate the wildcard functionality working
        println!("=== Dockerfile Wildcard Pattern Tests ===");

        // Exact filename matches (highest precedence)
        let result1 = detect_lang("Dockerfile".to_string());
        println!("Dockerfile -> {}", result1);
        assert_eq!(result1, "dockerfile");

        let result2 = detect_lang("dockerfile".to_string());
        println!("dockerfile -> {}", result2);
        assert_eq!(result2, "dockerfile");

        // Pattern matches for Dockerfile.*
        let result3 = detect_lang("Dockerfile.base".to_string());
        println!("Dockerfile.base -> {}", result3);
        assert_eq!(result3, "dockerfile");

        let result4 = detect_lang("Dockerfile.production".to_string());
        println!("Dockerfile.production -> {}", result4);
        assert_eq!(result4, "dockerfile");

        // Pattern matches for dockerfile.*
        let result5 = detect_lang("dockerfile.dev".to_string());
        println!("dockerfile.dev -> {}", result5);
        assert_eq!(result5, "dockerfile");

        // Pattern matches for *.dockerfile
        let result6 = detect_lang("web.dockerfile".to_string());
        println!("web.dockerfile -> {}", result6);
        assert_eq!(result6, "dockerfile");

        let result7 = detect_lang("api.dockerfile".to_string());
        println!("api.dockerfile -> {}", result7);
        assert_eq!(result7, "dockerfile");

        // Extensions take precedence over patterns
        let result8 = detect_lang("Dockerfile.js".to_string());
        println!("Dockerfile.js -> {}", result8);
        assert_eq!(result8, "javascript");

        let result9 = detect_lang("dockerfile.py".to_string());
        println!("dockerfile.py -> {}", result9);
        assert_eq!(result9, "python");

        println!("=== All wildcard tests passed! ===");
    }

    #[test]
    fn test_additional_wildcard_patterns() {
        // Test Makefile patterns
        assert_eq!(detect_lang("Makefile".to_string()), "makefile");
        assert_eq!(detect_lang("Makefile.local".to_string()), "makefile");
        assert_eq!(detect_lang("Makefile.dev".to_string()), "makefile");
        assert_eq!(detect_lang("makefile.prod".to_string()), "makefile");

        // Test shell patterns
        assert_eq!(detect_lang(".bashrc".to_string()), "shell");
        assert_eq!(detect_lang("my.bashrc".to_string()), "shell");
        assert_eq!(detect_lang("custom.zshrc".to_string()), "shell");
        assert_eq!(detect_lang("local.profile".to_string()), "shell");

        // These should not match
        assert_eq!(detect_lang("bashrc".to_string()), "plaintext"); // no dot prefix
        assert_eq!(detect_lang("notmakefile".to_string()), "plaintext");
    }
}

#[cfg(test)]
mod config_detection_tests {
    use mdbook_embedify::detect_lang;
    use mdbook_core::config::Config;

    #[test]
    fn test_config_overrides() {
         // Construct context
         let mut config = Config::default();
         // Set include.languages.dockerfile.extensions = [".mydev"]
         config.set("preprocessor.embedify.include.languages.dockerfile.extensions", vec![".mydev"]).unwrap();
           
         assert_eq!(detect_lang::detect_lang("test.mydev".to_string(), Some(&config)), "dockerfile");
    }

    #[test]
    fn test_config_filenames() {
         let mut config = Config::default();
         config.set("preprocessor.embedify.include.languages.makefile.filenames", vec!["MyMake"]).unwrap();
           
         assert_eq!(detect_lang::detect_lang("MyMake".to_string(), Some(&config)), "makefile");
    }

    #[test]
    fn test_config_patterns() {
         let mut config = Config::default();
         // Test wildcard pattern configuration
         config.set("preprocessor.embedify.include.languages.python.filenames", vec!["config_*.script"]).unwrap();
           
         assert_eq!(detect_lang::detect_lang("config_test.script".to_string(), Some(&config)), "python");
    }

    #[test]
    fn test_add_new_language() {
         let mut config = Config::default();
         // Add a completely new language
         config.set("preprocessor.embedify.include.languages.mylang.extensions", vec![".mlg"]).unwrap();
           
         assert_eq!(detect_lang::detect_lang("test.mlg".to_string(), Some(&config)), "mylang");
    }
}
