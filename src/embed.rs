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

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                // get chapter content
                let content = &chapter.content;

                // this can make the process faster
                if !content.contains("{% embed ") && !content.ends_with(" %}") {
                    return;
                }

                let re = Regex::new(r"\{% embed (\w+) (.*) %\}").unwrap();
                let caps = re.captures(&content).unwrap();

                // get app and options
                let app = caps.get(1).map_or("", |m| m.as_str());
                let options_str = caps.get(2).map_or("", |m| m.as_str());

                // render the template
                let template = utils::render_template(app, &utils::parse_options(options_str));

                // replace the content
                chapter.content = re.replace_all(&content as &str, template).to_string()
            }
        });

        Ok(book)
    }
}
