use pest::Parser;

#[derive(Parser)]
#[grammar = "assets/parser.pest"] // Ensure this file name is correct.
struct MacroParser;

#[derive(Clone)]
pub struct Placeholder {
    pub key: String,
    pub default: String,
    pub method: String,
}

pub struct EmbedAppOption {
    pub name: String,
    pub value: String,
}

pub struct EmbedApp {
    pub name: String,
    pub options: Vec<EmbedAppOption>,
}

pub fn parse_placeholder(input: &str) -> Option<Placeholder> {
    if let Ok(pairs) = MacroParser::parse(Rule::placeholder, input) {
        for pair in pairs {
            // Skip if the rule is not `placeholder`
            if pair.as_rule() != Rule::placeholder {
                continue;
            }

            let mut placeholder = Placeholder {
                key: String::new(),
                default: String::new(),
                method: String::new(),
            };

            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    // Placeholder method
                    Rule::placeholder_method => {
                        placeholder.method = inner_pair.as_str().to_string();
                    }

                    // Placeholder key and default value, only have one argument
                    Rule::placeholder_argument => {
                        for arg_pair in inner_pair.into_inner() {
                            match arg_pair.as_rule() {
                                // Placeholder key
                                Rule::placeholder_key => {
                                    placeholder.key = arg_pair.as_str().to_string();
                                }

                                // Placeholder default value
                                Rule::placeholder_default => {
                                    placeholder.default = arg_pair
                                        .as_str()
                                        .trim_matches('"')
                                        .trim_matches('\'')
                                        .to_string();
                                }

                                _ => {}
                            }
                        }
                    }

                    _ => {}
                }
            }

            return Some(placeholder);
        }
    }

    None
}

pub fn parse_app(input: &str) -> Option<EmbedApp> {
    if let Ok(pairs) = MacroParser::parse(Rule::embed, input) {
        for pair in pairs {
            // Skip if the rule is not `embed`
            if pair.as_rule() != Rule::embed {
                continue;
            }

            let mut app = EmbedApp {
                name: String::new(),
                options: Vec::new(),
            };

            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    // App name
                    Rule::embed_name => {
                        app.name = inner_pair.as_str().to_string();
                    }

                    // App options
                    Rule::embed_options => {
                        for option_pair in inner_pair.into_inner() {
                            match option_pair.as_rule() {
                                // Option name and value pairs
                                Rule::embed_option => {
                                    let mut option = EmbedAppOption {
                                        name: String::new(),
                                        value: String::new(),
                                    };

                                    for opt_pair in option_pair.into_inner() {
                                        match opt_pair.as_rule() {
                                            Rule::embed_option_name => {
                                                option.name = opt_pair.as_str().to_string();
                                            }

                                            Rule::embed_option_value => {
                                                option.value = opt_pair
                                                    .as_str()
                                                    .trim_matches('"')
                                                    .trim_matches('\'')
                                                    .to_string();
                                            }
                                            _ => {}
                                        }
                                    }

                                    app.options.push(option);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }

            return Some(app);
        }
    }

    None
}
