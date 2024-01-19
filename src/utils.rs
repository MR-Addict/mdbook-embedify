use clap::{Arg, Command};
use mdbook::{preprocess::Preprocessor, Config};
use pulldown_cmark;
use regex::Regex;
use rust_embed::RustEmbed;
use std::process;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Assets;

pub fn get_config_bool(config: &Config, key: &str) -> bool {
    config
        .get(format!("preprocessor.embedify.{}", key).as_str())
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

pub fn get_config_string(config: &Config, key: &str, default: &str) -> String {
    config
        .get(format!("preprocessor.embedify.{}", key).as_str())
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

pub fn parse_options(options_str: &str) -> Vec<(String, String)> {
    // wrap the options in a <p> tag
    let options_str = format!("<p {}/>", options_str);
    // parse the options and warn if there is an syntax error
    let doc = roxmltree::Document::parse(&options_str)
        .expect("Failed to parse options, please check your syntax");
    let mut options = Vec::new();
    for attr in doc.root_element().attributes() {
        options.push((attr.name().to_owned(), attr.value().to_owned()));
    }
    options
}

pub fn render_markdown_processor(content: String) -> String {
    let mut html = String::new();
    let parser = pulldown_cmark::Parser::new(&content);
    pulldown_cmark::html::push_html(&mut html, parser);
    html.into()
}

pub fn render_template(app: &str, placeholders: &[(String, String)]) -> String {
    let path = format!("{}.html", app);
    // check if app is supported
    if !Assets::iter().any(|name| name == path) {
        panic!("App {} is not supported", app);
    }

    // get the template from the embedded files
    let template = Assets::get(&path).unwrap();
    // get template as string
    let template = std::str::from_utf8(template.data.as_ref()).unwrap();
    let mut result = template.to_string();

    // create a processors vec
    let mut processors: Vec<(String, fn(String) -> String)> = Vec::new();
    processors.push(("markdown".to_string(), render_markdown_processor));

    // replace the key with the value
    for (key, value) in placeholders {
        // iterate over the methods
        for (name, processor) in &processors {
            let pattern = format!(r"\{{% {}\({}\) %\}}", name, key);
            let re = Regex::new(&pattern).unwrap();
            if re.is_match(&result) {
                // replace {% processor(key) %} with processor rendered content
                let rendered = processor(value.clone());
                result = re.replace_all(&result, rendered).to_string();
            } else {
                // replace {% key %} or {% key|default %} with value
                let pattern = format!(r"\{{% {}(\|[^}}]*)? %\}}", key);
                let re = Regex::new(&pattern).unwrap();
                result = re.replace_all(&result, value).to_string();
            }
        }
    }

    // replace {% key|default %} with default value
    let re = Regex::new(r"\{\% ([^|]+)\|([^}]+) \%\}").unwrap();
    result = re.replace_all(&result, "$2").to_string();

    // return the result
    result.to_string()
}

pub fn reply_supports(pre: &dyn Preprocessor) {
    // Handle support for the --supports command line argument
    let matches = Command::new("mdbook-embedify")
        .about("A mdbook embed preprocessor that embeds app to your book")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = sub_args
            .get_one::<String>("renderer")
            .expect("Required argument");

        // Signal whether the renderer is supported by exiting with 1 or 0.
        if pre.supports_renderer(renderer) {
            process::exit(0);
        } else {
            process::exit(1);
        }
    }
}
