use pulldown_cmark;
use regex::Regex;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Assets;

pub fn render_markdown_processor(content: String) -> String {
    let mut html = String::new();
    let parser = pulldown_cmark::Parser::new(&content);
    pulldown_cmark::html::push_html(&mut html, parser);
    html.into()
}

pub fn parse_options(options_str: &str) -> Vec<(String, String)> {
    // options parse regex
    let re = Regex::new(r#"([\w-]+)="([^"]*)""#).unwrap();

    // optoins turple vector
    let mut options = Vec::new();

    // parse options
    for cap in re.captures_iter(options_str.trim()) {
        options.push((cap[1].to_string(), cap[2].to_string()));
    }

    // return options
    options
}

pub fn render_template(app: &str, placeholders: &[(String, String)]) -> String {
    // app path
    let path = format!("{}.html", app);

    // check if and app is supported
    if !Assets::iter().any(|name| name == path) {
        panic!("App {} is not supported", app);
    }

    // get the template from the embedded files
    let file = Assets::get(&path).unwrap();
    let template = std::str::from_utf8(file.data.as_ref()).unwrap();

    // create a processors vec
    let processors: Vec<(String, fn(String) -> String)> = vec![
        ("raw".to_string(), |content| content),
        ("markdown".to_string(), render_markdown_processor),
    ];

    // render placeholders
    let mut result = template.to_string();
    for (key, value) in placeholders {
        // iterate over the methods
        for (name, processor) in &processors {
            let pattern = format!(r"\{{% {}\({}(?:=([^)]+))?\) %\}}", name, key);
            let re = Regex::new(&pattern).unwrap();
            if re.is_match(&result) {
                // replace {% processor(key=default) %} with processor rendered content
                let rendered = processor(value.clone());
                result = re.replace_all(&result, rendered).to_string();
            }
        }
    }

    // replace {% processor(key=default) %} with default value
    let re = Regex::new(r"\{\% (\w+)\((\w+)=([^)]+)\) \%\}").unwrap();
    result = re.replace_all(&result, "$3").to_string();

    // return the result
    result.to_string()
}
