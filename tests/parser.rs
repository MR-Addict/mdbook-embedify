use mdbook_embedify::parser::{get_option, parse_app, parse_placeholder, EmbedAppOption};

#[cfg(test)]
mod placeholder_tests {
    use super::*;

    #[test]
    fn test_parse_placeholder_with_key_only() {
        let input = "{% title %}";
        let result = parse_placeholder(input).unwrap();

        assert_eq!(result.key, "title");
        assert_eq!(result.default, "");
        assert_eq!(result.method, "");
    }

    #[test]
    fn test_parse_placeholder_with_key_and_default() {
        let input = "{% title=\"Default Title\" %}";
        let result = parse_placeholder(input).unwrap();

        assert_eq!(result.key, "title");
        assert_eq!(result.default, "Default Title");
        assert_eq!(result.method, "");
    }

    #[test]
    fn test_parse_placeholder_with_single_quotes() {
        let input = "{% title='Default Title' %}";
        let result = parse_placeholder(input).unwrap();

        assert_eq!(result.key, "title");
        assert_eq!(result.default, "Default Title");
        assert_eq!(result.method, "");
    }

    #[test]
    fn test_parse_placeholder_with_method() {
        let input = "{% upper(title) %}";
        let result = parse_placeholder(input).unwrap();

        assert_eq!(result.key, "title");
        assert_eq!(result.default, "");
        assert_eq!(result.method, "upper");
    }

    #[test]
    fn test_parse_placeholder_with_method_and_default() {
        let input = "{% upper(title=\"Hello World\") %}";
        let result = parse_placeholder(input).unwrap();

        assert_eq!(result.key, "title");
        assert_eq!(result.default, "Hello World");
        assert_eq!(result.method, "upper");
    }

    #[test]
    fn test_parse_placeholder_with_underscore_and_dash() {
        let input = "{% my_key-name %}";
        let result = parse_placeholder(input).unwrap();

        assert_eq!(result.key, "my_key-name");
        assert_eq!(result.default, "");
        assert_eq!(result.method, "");
    }

    #[test]
    fn test_parse_placeholder_invalid_syntax() {
        let input = "{% %}";
        let result = parse_placeholder(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_placeholder_missing_brackets() {
        let input = "title";
        let result = parse_placeholder(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_placeholder_empty_string() {
        let input = "";
        let result = parse_placeholder(input);
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod embed_app_tests {
    use super::*;

    #[test]
    fn test_parse_app_simple() {
        let input = "{% embed youtube %}";
        let result = parse_app(input).unwrap();

        assert_eq!(result.name, "youtube");
        assert_eq!(result.options.len(), 0);
    }

    #[test]
    fn test_parse_app_with_single_option() {
        let input = "{% embed youtube id=\"dQw4w9WgXcQ\" %}";
        let result = parse_app(input).unwrap();

        assert_eq!(result.name, "youtube");
        assert_eq!(result.options.len(), 1);
        assert_eq!(result.options[0].name, "id");
        assert_eq!(result.options[0].value, "dQw4w9WgXcQ");
    }

    #[test]
    fn test_parse_app_with_multiple_options() {
        let input = "{% embed youtube id=\"dQw4w9WgXcQ\" width=800 height=600 %}";
        let result = parse_app(input).unwrap();

        assert_eq!(result.name, "youtube");
        assert_eq!(result.options.len(), 3);

        let id_option = result.options.iter().find(|opt| opt.name == "id").unwrap();
        assert_eq!(id_option.value, "dQw4w9WgXcQ");

        let width_option = result
            .options
            .iter()
            .find(|opt| opt.name == "width")
            .unwrap();
        assert_eq!(width_option.value, "800");

        let height_option = result
            .options
            .iter()
            .find(|opt| opt.name == "height")
            .unwrap();
        assert_eq!(height_option.value, "600");
    }

    #[test]
    fn test_parse_app_with_single_quotes() {
        let input = "{% embed codepen pen='abcdef' theme='dark' %}";
        let result = parse_app(input).unwrap();

        assert_eq!(result.name, "codepen");
        assert_eq!(result.options.len(), 2);

        let pen_option = result.options.iter().find(|opt| opt.name == "pen").unwrap();
        assert_eq!(pen_option.value, "abcdef");

        let theme_option = result
            .options
            .iter()
            .find(|opt| opt.name == "theme")
            .unwrap();
        assert_eq!(theme_option.value, "dark");
    }

    #[test]
    fn test_parse_app_with_mixed_quotes() {
        let input = "{% embed stackblitz project=\"angular-starter\" theme='dark' %}";
        let result = parse_app(input).unwrap();

        assert_eq!(result.name, "stackblitz");
        assert_eq!(result.options.len(), 2);

        let project_option = result
            .options
            .iter()
            .find(|opt| opt.name == "project")
            .unwrap();
        assert_eq!(project_option.value, "angular-starter");

        let theme_option = result
            .options
            .iter()
            .find(|opt| opt.name == "theme")
            .unwrap();
        assert_eq!(theme_option.value, "dark");
    }

    #[test]
    fn test_parse_app_with_dashes_and_underscores() {
        let input = "{% embed my-app_name option_1=value-1 option-2=\"value_2\" %}";
        let result = parse_app(input).unwrap();

        assert_eq!(result.name, "my-app_name");
        assert_eq!(result.options.len(), 2);

        let option1 = result
            .options
            .iter()
            .find(|opt| opt.name == "option_1")
            .unwrap();
        assert_eq!(option1.value, "value-1");

        let option2 = result
            .options
            .iter()
            .find(|opt| opt.name == "option-2")
            .unwrap();
        assert_eq!(option2.value, "value_2");
    }

    #[test]
    fn test_parse_app_invalid_syntax() {
        let input = "{% embed %}";
        let result = parse_app(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_app_missing_embed_keyword() {
        let input = "{% youtube id=\"test\" %}";
        let result = parse_app(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_app_empty_string() {
        let input = "";
        let result = parse_app(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_app_malformed_brackets() {
        let input = "{ embed youtube %}";
        let result = parse_app(input);
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod get_option_tests {
    use super::*;

    #[test]
    fn test_get_option_found() {
        let options = vec![
            EmbedAppOption {
                name: "id".to_string(),
                value: "test123".to_string(),
            },
            EmbedAppOption {
                name: "width".to_string(),
                value: "800".to_string(),
            },
            EmbedAppOption {
                name: "height".to_string(),
                value: "600".to_string(),
            },
        ];

        let result = get_option("width", options).unwrap();
        assert_eq!(result.name, "width");
        assert_eq!(result.value, "800");
    }

    #[test]
    fn test_get_option_not_found() {
        let options = vec![
            EmbedAppOption {
                name: "id".to_string(),
                value: "test123".to_string(),
            },
            EmbedAppOption {
                name: "width".to_string(),
                value: "800".to_string(),
            },
        ];

        let result = get_option("height", options);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_option_empty_list() {
        let options = vec![];
        let result = get_option("any", options);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_option_case_sensitive() {
        let options = vec![EmbedAppOption {
            name: "Width".to_string(),
            value: "800".to_string(),
        }];

        let result = get_option("width", options);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_option_exact_match() {
        let options = vec![
            EmbedAppOption {
                name: "width".to_string(),
                value: "800".to_string(),
            },
            EmbedAppOption {
                name: "width-max".to_string(),
                value: "1200".to_string(),
            },
        ];

        let result = get_option("width", options).unwrap();
        assert_eq!(result.name, "width");
        assert_eq!(result.value, "800");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complex_embed_parsing() {
        let input = "{% embed youtube id=\"dQw4w9WgXcQ\" width=800 height=600 autoplay=false title=\"Never Gonna Give You Up\" %}";
        let result = parse_app(input).unwrap();

        assert_eq!(result.name, "youtube");
        assert_eq!(result.options.len(), 5);

        let id = get_option("id", result.options.clone()).unwrap();
        assert_eq!(id.value, "dQw4w9WgXcQ");

        let width = get_option("width", result.options.clone()).unwrap();
        assert_eq!(width.value, "800");

        let height = get_option("height", result.options.clone()).unwrap();
        assert_eq!(height.value, "600");

        let autoplay = get_option("autoplay", result.options.clone()).unwrap();
        assert_eq!(autoplay.value, "false");

        let title = get_option("title", result.options.clone()).unwrap();
        assert_eq!(title.value, "Never Gonna Give You Up");
    }

    #[test]
    fn test_complex_placeholder_parsing() {
        let input = "{% upper(page_title=\"My Awesome Blog Post\") %}";
        let result = parse_placeholder(input).unwrap();

        assert_eq!(result.method, "upper");
        assert_eq!(result.key, "page_title");
        assert_eq!(result.default, "My Awesome Blog Post");
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "{%   embed    youtube   id=\"test\"   width=800   %}";
        let result = parse_app(input).unwrap();

        assert_eq!(result.name, "youtube");
        assert_eq!(result.options.len(), 2);

        let id = get_option("id", result.options.clone()).unwrap();
        assert_eq!(id.value, "test");

        let width = get_option("width", result.options.clone()).unwrap();
        assert_eq!(width.value, "800");
    }
}
