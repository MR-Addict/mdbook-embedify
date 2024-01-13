use crate::utils;
use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
};

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
        let config = &ctx.config;

        let scroll_to_top_enabled = utils::get_config_bool(config, "scroll-to-top.enable", false);

        let announcement_banner_enabled =
            utils::get_config_bool(config, "announcement-banner.enable", false);
        let announcement_banner_name =
            utils::get_config_string(config, "announcement-banner.name", "");
        let announcement_banner_message =
            utils::get_config_string(config, "announcement-banner.message", "");
        let announcement_banner_style =
            utils::get_config_string(config, "announcement-banner.style", "default");

        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                if scroll_to_top_enabled {
                    chapter.content.push_str("\n{% embed scroll-to-top %}\n");
                }
                if announcement_banner_enabled {
                    chapter.content.push_str(&format!(
                        "\n{{% embed announcement-banner name=\"{}\" message=\"{}\" style=\"{}\" %}}\n",
                        announcement_banner_name, announcement_banner_message, announcement_banner_style
                    ));
                }
                chapter.content = utils::render_embeds(chapter.content.clone());
            }
        });

        // return the book
        Ok(book)
    }
}
