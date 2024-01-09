use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::Regex;
use roxmltree;

pub struct Embed;

impl Embed {
    pub fn new() -> Embed {
        Embed
    }
}

fn parse_text(text: &str) -> String {
    // this can make the process faster
    if !text.contains("{% embed ") && !text.ends_with(" %}") {
        return text.to_owned();
    }

    let re = Regex::new(r"\{% embed (\w+) (.*) %\}").unwrap();

    re.replace_all(text, |caps: &regex::Captures| {
        let app = caps.get(1).map_or("", |m| m.as_str());
        let options_str = caps.get(2).map_or("", |m| m.as_str());
        let options_str = format!("<p {}/>", options_str);
        let doc = roxmltree::Document::parse(&options_str).unwrap();

        if app == "youtube" {
            let id = doc.root_element().attribute("id").unwrap();
            return format!("<iframe src=\"https://www.youtube.com/embed/{}\" allowfullscreen allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share\" style=\"width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 0.375rem\"></iframe>", id);
        } else if app == "codepen" {
            let slug = doc.root_element().attribute("slug").unwrap();
            let username = doc.root_element().attribute("username").unwrap();

            let height = match doc.root_element().attribute("height") {
                Some(height) => height,
                None => "600"
            };

            let theme = match doc.root_element().attribute("theme") {
                Some(height) => height,
                None => "dark"
            };

            return format!("<iframe height=\"{}\" title=\"Codepen\" src=\"https://codepen.io/{}/embed/{}?default-tab=result&theme-id={}\" allowtransparency allowfullscreen style=\"width: 100%; border: none; border-radius: 0.375rem\"></iframe>", height, username, slug, theme);
        } else if app =="gist" {
            let id = doc.root_element().attribute("id").unwrap();
            return format!("<script src=\"https://gist.github.com/{}.js\"></script>", id);
        } else {
            return text.to_owned();
        }
    }).to_string()
}

impl Preprocessor for Embed {
    fn name(&self) -> &str {
        return "mdbook-embedify";
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                chapter.content = parse_text(&chapter.content);
            }
        });

        return Ok(book);
    }
}
