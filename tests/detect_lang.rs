use mdbook_embedify::detect_lang::detect_lang;

#[cfg(test)]
mod basic_language_detection_tests {
    use super::*;

    #[test]
    fn test_detect_rust_language() {
        assert_eq!(detect_lang("src/main.rs".to_string()), "rust");
        assert_eq!(detect_lang("lib.rs".to_string()), "rust");
        assert_eq!(detect_lang("config.rs.in".to_string()), "rust");
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
        assert_eq!(detect_lang("header.h".to_string()), "cpp");
        assert_eq!(detect_lang("config.h.in".to_string()), "c");
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
        assert_eq!(detect_lang("Makefile".to_string()), "plaintext");
        assert_eq!(detect_lang("LICENSE".to_string()), "plaintext");
        assert_eq!(detect_lang("Dockerfile".to_string()), "plaintext");
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
    fn test_compound_extensions() {
        // Test extensions with multiple parts
        assert_eq!(detect_lang("archive.tar.gz".to_string()), "plaintext");
        assert_eq!(detect_lang("config.rs.in".to_string()), "rust");
    }

    #[test]
    fn test_longest_extension_match() {
        // The function should try longer extensions first
        // For example, if both .html and .html.hl are defined, it should prefer the longer one
        assert_eq!(detect_lang("file.html".to_string()), "html");
        assert_eq!(detect_lang("file.html.hl".to_string()), "html");
    }

    #[test]
    fn test_typescript_extensions() {
        // TypeScript extensions conflict with XML
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
        assert_eq!(detect_lang("data.csv".to_string()), "plaintext"); // CSV not in language map
        assert_eq!(detect_lang("config.ini".to_string()), "ini");
        assert_eq!(detect_lang("database.sql".to_string()), "sql");
        assert_eq!(detect_lang("query.graphql".to_string()), "plaintext"); // GraphQL not in language map
    }
}
