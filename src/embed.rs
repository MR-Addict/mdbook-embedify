use crate::assets::scripts::include;
use crate::parser;
use crate::utils;

use mdbook::{
    book::{Book, Chapter},
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::Regex;
use rust_embed::RustEmbed;

// Compile regex patterns once for reuse
lazy_static::lazy_static! {
    static ref RE_EMBED_MACRO: Regex = Regex::new(r"\{%\s*.*?\s*%\}").unwrap();
    static ref RE_IGNORE: Regex = Regex::new(r"(?si)<!-- embed ignore begin -->(.*)<!-- embed ignore end -->").unwrap();
}

#[derive(RustEmbed)]
#[folder = "src/assets/templates"]
struct Assets;

pub struct Embed;

impl Embed {
    pub fn new() -> Embed {
        Embed
    }
}

fn render_script_app(
    ctx: &PreprocessorContext,
    app: parser::EmbedApp,
) -> Result<Option<String>, String> {
    match app.name.as_str() {
        "include" => include::include_script(ctx, app.options).map(Some),
        _ => Ok(None),
    }
}

fn render_template_app(
    ctx: &PreprocessorContext,
    app: parser::EmbedApp,
) -> Result<Option<String>, String> {
    let mut template = String::new();
    let app_path = format!("{}.html", app.name);

    // check custom template first
    let templates_folder =
        utils::get_config_string(&ctx.config, "custom-templates-folder", "assets/templates");
    if !templates_folder.is_empty() {
        let joined_folder = ctx.root.join(templates_folder);
        let joined_folder = joined_folder.to_string_lossy().to_string();
        let template_path = format!("{}/{}", joined_folder, app_path);
        if std::path::Path::new(&template_path).exists() {
            template = std::fs::read_to_string(&template_path).unwrap_or_else(|_| String::new());
        }
    }

    // if custom template not found, use the default template
    if template.is_empty() && Assets::iter().any(|name| name == app_path) {
        // get the template from the embedded files
        let file = Assets::get(&app_path).unwrap();
        template = String::from_utf8(file.data.to_vec()).unwrap_or_else(|_| String::new());
    }

    // No template found
    if template.is_empty() {
        return Ok(None);
    }

    // Use a mutable flag to track if we need to exit early
    let mut should_exit = false;

    let result = RE_EMBED_MACRO
        .replace_all(&template, |caps: &regex::Captures| {
            if should_exit {
                return "".to_string(); // Short-circuit further replacements
            }

            let input = caps.get(0).map_or("", |m| m.as_str());
            let placeholder = parser::parse_placeholder(input);

            if placeholder.is_none() {
                return input.to_string();
            }

            // find the value in the options
            let placeholder = placeholder.unwrap();
            let found = app
                .options
                .iter()
                .find(|option| option.name == placeholder.key);

            // check if the option is required and not set
            if placeholder.default.is_empty() {
                if found.is_none() || found.unwrap().value.is_empty() {
                    should_exit = true;
                    return input.to_string();
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

    // If the flag is set, return error
    if should_exit {
        return Err("Missing required options".to_string());
    }

    Ok(Some(utils::minify_html(result)))
}

fn render_embeds(ctx: &PreprocessorContext, chapter: Chapter, content: String) -> String {
    let mut content = content;
    if chapter.is_draft_chapter() {
        return content; // Skip processing if the chapter is a draft
    }

    let chapter_path = chapter.path.unwrap().clone(); // Clone chapter path to avoid consuming it

    let mut ignored_sections: Vec<(String, String)> = Vec::new();
    for (i, caps) in RE_IGNORE.captures_iter(&content.clone()).enumerate() {
        let placeholder = format!("EMBED_IGNORE_{}", i);
        let ignored = caps.get(0).map_or("", |m| m.as_str());

        ignored_sections.push((placeholder.clone(), ignored.to_string()));
        content = RE_IGNORE.replace(&content, placeholder).to_string();
    }

    content = RE_EMBED_MACRO
        .replace_all(&content, |caps: &regex::Captures| {
            let input = caps.get(0).map_or("", |m| m.as_str());
            let app = parser::parse_app(input);
            if app.is_none() {
                return input.to_string();
            }
            let app = app.unwrap();

            // render template app first
            let mut rendered = render_template_app(ctx, app.clone());

            // when is ok, but not rendered, try to render script app
            if rendered.is_ok() && rendered.as_ref().unwrap().is_none() {
                rendered = render_script_app(ctx, app.clone());
            }

            // if failed, print the error and return the input
            if !rendered.is_ok() {
                let err = rendered.err().unwrap();
                eprintln!(
                    "(mdbook-embedify): Error while rendering app \"{}\" in {:?}. {}",
                    app.name, chapter_path, err
                );
                return input.to_string();
            }

            // if the app is not rendered, return the input
            if rendered.as_ref().unwrap().is_none() {
                return input.to_string();
            }

            // unwrap the result
            rendered.unwrap().unwrap()
        })
        .to_string();

    for (placeholder, ignored) in ignored_sections {
        content = content.replace(&placeholder, &ignored);
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
                chapter.content = render_embeds(ctx, chapter.clone(), content);
            }
        });

        // return the book
        Ok(book)
    }
}
