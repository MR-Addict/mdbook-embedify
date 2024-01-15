use clap::{Arg, Command};
use mdbook::{preprocess::Preprocessor, Config};
use pulldown_cmark;
use regex::Regex;
use rust_embed::RustEmbed;
use std::process;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Assets;

fn parse_options(options_str: &str) -> Vec<(String, String)> {
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

fn render_template(app: &str, placeholders: &[(String, String)]) -> String {
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

    // replace the key with the value
    for (key, value) in placeholders {
        let mut replacement = value.clone();
        // if app is announcement-banner and key is message, render markdown content
        if app == "announcement-banner" && key == "message" {
            let mut html = String::new();
            let parser = pulldown_cmark::Parser::new(&replacement);
            pulldown_cmark::html::push_html(&mut html, parser);
            replacement = html.into();
        }

        // replace {% key %} with replacement
        let re = Regex::new(&format!(r"\{{% {}\ %\}}", key)).unwrap();
        let replaced_result = re.replace_all(&result, replacement.clone()).to_string();
        // replacement was successful
        if replaced_result != result {
            result = replaced_result
        } else {
            //replacement does not success, try replace {% key|default %} with replacement
            let re = Regex::new(&format!(r"\{{% {}\|[^%]* %\}}", key)).unwrap();
            result = re.replace_all(&result, replacement).to_string();
        }
    }

    // replace {% key|default %} with default value
    let re = Regex::new(r"\{\% ([^|]+)\|([^}]+) \%\}").unwrap();
    result = re.replace_all(&result, "$2").to_string();

    // return the result
    result.to_string()
}

pub fn render_embeds(content: String) -> String {
    let mut content = content;
    // this can make the process faster
    if !content.contains("{% embed ") && !content.ends_with(" %}") {
        return content;
    }

    // create a regex to match <!-- embed ignore begin -->...<!-- embed ignore end -->
    let re_ignore =
        Regex::new(r"(?s)<!-- embed ignore begin -->(.*)<!-- embed ignore end -->").unwrap();

    // replace the ignored content with a placeholder
    let mut ignored_sections: Vec<(String, String)> = Vec::new();
    for (i, caps) in re_ignore.captures_iter(&content.clone()).enumerate() {
        let placeholder = format!("EMBED_IGNORE_{}", i);
        let ignored = caps.get(0).map_or("", |m| m.as_str());

        ignored_sections.push((placeholder.clone(), ignored.to_string()));
        content = re_ignore.replace(&content, placeholder).to_string();
    }

    // replace the content
    let re_embed = Regex::new(r"\{% embed ([\w-]+)(.*) %\}").unwrap();
    content = re_embed
        .replace_all(&content, |caps: &regex::Captures| {
            // parse app and options str
            let app = caps.get(1).map_or("", |m| m.as_str());
            let options_str = caps.get(2).map_or("", |m| m.as_str());

            // get options and return the rendered template
            let options = parse_options(options_str);
            render_template(app, &options)
        })
        .to_string();

    // replace the placeholders with the ignored content
    if ignored_sections.len() > 0 {
        for (placeholder, ignored) in ignored_sections {
            content = content.replace(&placeholder, &ignored);
        }
    }

    content
}

pub fn get_config_string(config: &Config, key: &str, default: &str) -> String {
    config
        .get(format!("preprocessor.embedify.{}", key).as_str())
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

pub fn get_config_bool(config: &Config, key: &str, default: bool) -> bool {
    config
        .get(format!("preprocessor.embedify.{}", key).as_str())
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
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
