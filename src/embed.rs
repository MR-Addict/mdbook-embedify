use crate::utils;
use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::Regex;

pub struct Embed;

impl Embed {
    pub fn new() -> Embed {
        Embed
    }
}

impl Preprocessor for Embed {
    fn name(&self) -> &str {
        "mdbook-embedify"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // remove mdbook- from the name
        let name = self.name().replace("mdbook-", "");
        // get embedify config options
        let config = ctx.config.get_preprocessor(name).unwrap();

        let mut scroll_to_top = false;
        // if scroll-to-top set, get the value
        if let Some(raw_scroll_to_top) = config.get("scroll-to-top") {
            scroll_to_top = raw_scroll_to_top.as_bool().unwrap_or(false);
        }

        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                // copy chapter content
                let mut content = chapter.content.clone();

                // if scroll-to-top is enabled, add {% embed scrolltotop %}
                if scroll_to_top {
                    content.push_str("\n{% embed scroll-to-top %}");
                }

                // this can make the process faster
                if !scroll_to_top && !content.contains("{% embed ") && !content.ends_with(" %}") {
                    return;
                }

                // create a regex to match <!-- embed ignore begin -->...<!-- embed ignore end -->
                let re_ignore =
                    Regex::new(r"(?s)<!-- embed ignore begin -->(.*)<!-- embed ignore end -->")
                        .unwrap();

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

                chapter.content = content;
            }
        });

        Ok(book)
    }
}
