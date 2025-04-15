use crate::assets::scripts::include;
use crate::parser;
use crate::utils;

use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::Regex;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/assets/templates"]
struct Assets;

pub struct Embed;

impl Embed {
    pub fn new() -> Embed {
        Embed
    }
}

fn render_script_app(app: parser::EmbedApp) -> Option<String> {
    match app.name.as_str() {
        "include" => Some(include::include_script(app.options)),
        _ => None,
    }
}

fn render_template_app(app: parser::EmbedApp) -> Option<String> {
    let app_path = format!("{}.html", app.name);

    // check if and app is supported
    if !Assets::iter().any(|name| name == app_path) {
        return None;
    }

    // get the template from the embedded files
    let file = Assets::get(&app_path).unwrap();
    let template = std::str::from_utf8(file.data.as_ref()).unwrap();

    // render placeholders
    let re = Regex::new(r"\{%\s*.*?\s*%\}").unwrap();

    let result = re
        .replace_all(&template.to_string(), |caps: &regex::Captures| {
            let input = caps.get(0).map_or("", |m| m.as_str());
            let placeholder = parser::parse_placeholder(input);

            if placeholder.is_none() {
                return input.to_string();
            }

            let placeholder = placeholder.unwrap();

            // find the value in the options
            let found = app
                .options
                .iter()
                .find(|option| option.name == placeholder.key);

            // when default value is not set, and the option value is not set, panic
            if placeholder.default.is_empty() {
                if found.is_none() || found.unwrap().value.is_empty() {
                    panic!(
                        "Option {} is required for app {}",
                        placeholder.key, app.name
                    );
                }
            }

            // when the option value is set, use it, otherwise use the default value
            let mut value = if found.is_some() && !found.unwrap().value.is_empty() {
                found.unwrap().value.clone()
            } else {
                placeholder.default.clone()
            };

            // render the value with the method
            if placeholder.method == "markdown" {
                value = utils::render_to_markdown(value.clone());
            }

            value
        })
        .to_string();

    Some(result)
}

fn render_embeds(content: String) -> String {
    let mut content = content;

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
    let re_embed = Regex::new(r"\{%\s*.*?\s*%\}").unwrap();
    content = re_embed
        .replace_all(&content, |caps: &regex::Captures| {
            let input = caps.get(0).map_or("", |m| m.as_str());
            let app = parser::parse_app(input);
            if app.is_none() {
                return input.to_string();
            }
            let app = app.unwrap();

            // render template app
            let mut rendered = render_template_app(app.clone());
            // when the template app is not supported, render script app
            if rendered.is_none() {
                rendered = render_script_app(app.clone());
            }

            // when the app is not supported, return the input
            if rendered.is_none() {
                panic!("Error while rendering app {}", app.name);
            }

            // return and format the rendered content
            let rendered = rendered.unwrap();
            format!("<!-- mdbook-embedify [{}]  -->\n{}", app.name, rendered)
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

impl Preprocessor for Embed {
    fn name(&self) -> &str {
        "mdbook-embedify"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config = &ctx.config;

        let footer = utils::get_config_bool(config, "footer.enable");
        let giscus = utils::get_config_bool(config, "giscus.enable");
        let scroll_to_top = utils::get_config_bool(config, "scroll-to-top.enable");
        let announcement_banner = utils::get_config_bool(config, "announcement-banner.enable");

        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                let mut content = chapter.content.clone();
                // create the global scroll to top button
                if scroll_to_top {
                    content.push_str(&utils::create_scroll_to_top());
                }
                // create the global announcement banner
                if announcement_banner {
                    content.push_str(&utils::create_announcement_banner(config));
                }
                // create the global giscus comments
                if giscus {
                    content.push_str(&utils::create_giscus(config));
                }
                // create the global footer
                if footer {
                    content.push_str(&utils::create_footer(config));
                }
                // render the embeds in the content
                chapter.content = render_embeds(content);
            }
        });

        // return the book
        Ok(book)
    }
}
