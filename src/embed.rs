use crate::utils;
use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
    Config,
};
use regex::Regex;

pub struct Embed;

impl Embed {
    pub fn new() -> Embed {
        Embed
    }
}

fn render_general_embeds(content: String) -> String {
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
    let re_embed = Regex::new(r"\{% embed ([\w-]+)(.*) %\}").unwrap();
    content = re_embed
        .replace_all(&content, |caps: &regex::Captures| {
            // parse app and options str
            let app = caps.get(1).map_or("", |m| m.as_str());
            let options_str = caps.get(2).map_or("", |m| m.as_str());

            // get options and return the rendered template
            let options = utils::parse_options(options_str);
            utils::render_template(app, &options)
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

fn render_announcement_banner(config: &Config) -> String {
    // get the config
    let id = utils::get_config_string(config, "announcement-banner.id", "");
    let theme = utils::get_config_string(config, "announcement-banner.theme", "default");
    let message = utils::get_config_string(config, "announcement-banner.message", "");

    // render the template
    let options = vec![
        ("id".to_string(), id),
        ("message".to_string(), message),
        ("theme".to_string(), theme),
    ];
    utils::render_template("announcement-banner", &options)
}

impl Preprocessor for Embed {
    fn name(&self) -> &str {
        "mdbook-embedify"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config = &ctx.config;

        let scroll_to_top_enabled = utils::get_config_bool(config, "scroll-to-top.enable", false);
        let announcement_banner_enabled =
            utils::get_config_bool(config, "announcement-banner.enable", false);

        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                // render every single embeds
                if chapter.content.contains("{% embed ") && chapter.content.contains(" %}") {
                    chapter.content = render_general_embeds(chapter.content.clone());
                }
                // render the global scroll to top button
                if scroll_to_top_enabled {
                    let template = utils::render_template("scroll-to-top", &Vec::new());
                    chapter.content.push_str(&template);
                }
                // render the global announcement banner
                if announcement_banner_enabled {
                    let template = render_announcement_banner(config);
                    chapter.content.push_str(&template);
                }
            }
        });

        // return the book
        Ok(book)
    }
}
