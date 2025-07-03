use lazy_static::lazy_static;
use mdbook::Config;
use minify::html::minify;
use pulldown_cmark;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

lazy_static! {
    static ref LANGUAGE_MAP: HashMap<String, String> = {
        let json_content = include_str!("assets/config/languages.json");
        let languages: Vec<serde_json::Value> =
            serde_json::from_str(json_content).expect("Failed to parse languages.json");

        let mut map = HashMap::new();
        for lang in languages {
            if let (Some(name), Some(extensions)) =
                (lang["name"].as_str(), lang["extensions"].as_array())
            {
                for ext in extensions {
                    if let Some(extension) = ext.as_str() {
                        map.insert(extension.to_string(), name.to_string());
                    }
                }
            }
        }
        map
    };
}

pub fn detect_lang(path: String) -> String {
    let path_obj = Path::new(&path);

    // Get the filename
    if let Some(filename) = path_obj.file_name() {
        if let Some(filename_str) = filename.to_str() {
            // Check for multi-dot extensions by looking for patterns like .html.hl, .tar.gz, etc.
            // We'll try progressively longer extensions starting from the first dot
            if let Some(first_dot_pos) = filename_str.find('.') {
                let mut current_pos = first_dot_pos;

                // Try each possible extension from longest to shortest
                while current_pos < filename_str.len() {
                    let potential_ext = &filename_str[current_pos..].to_lowercase();

                    // Look up the extension in our language map
                    if let Some(language) = LANGUAGE_MAP.get(potential_ext) {
                        return language.clone();
                    }

                    // Move to the next dot for a shorter extension
                    if let Some(next_dot) = filename_str[current_pos + 1..].find('.') {
                        current_pos = current_pos + 1 + next_dot;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    // If no extension found or extension not in map, return "plaintext"
    "plaintext".to_string()
}

pub fn remove_script_and_style_comments(content: String) -> String {
    let script_tag_re = Regex::new(r"(?s)<script.*?>.*?</script>").unwrap();
    let style_tag_re = Regex::new(r"(?s)<style.*?>.*?</style>").unwrap();

    let block_comment_re = Regex::new(r"(?s)/\*.*?\*/").unwrap();
    let line_comment_re = Regex::new(r"(?m)//.*?$").unwrap(); // only for <script>

    // Remove comments in <script>
    let content = script_tag_re
        .replace_all(&content, |caps: &regex::Captures| {
            let script_block = &caps[0];
            if let Some(body_start) = script_block.find('>') {
                if let Some(body_end) = script_block.rfind("</script>") {
                    let head = &script_block[..=body_start];
                    let body = &script_block[body_start + 1..body_end];
                    let tail = &script_block[body_end..];

                    let body = block_comment_re.replace_all(body, "");
                    let body = line_comment_re.replace_all(&body, "");

                    format!("{head}{}{tail}", body.trim())
                } else {
                    script_block.to_string()
                }
            } else {
                script_block.to_string()
            }
        })
        .to_string();

    // Remove comments in <style>
    let content = style_tag_re
        .replace_all(&content, |caps: &regex::Captures| {
            let style_block = &caps[0];
            if let Some(body_start) = style_block.find('>') {
                if let Some(body_end) = style_block.rfind("</style>") {
                    let head = &style_block[..=body_start];
                    let body = &style_block[body_start + 1..body_end];
                    let tail = &style_block[body_end..];

                    let body = block_comment_re.replace_all(body, "");

                    format!("{head}{}{tail}", body.trim())
                } else {
                    style_block.to_string()
                }
            } else {
                style_block.to_string()
            }
        })
        .to_string();

    content
}

pub fn minify_html(content: String) -> String {
    let result = remove_script_and_style_comments(content);
    minify(&result)
}

pub fn render_to_markdown(content: String) -> String {
    let mut html = String::new();
    let parser = pulldown_cmark::Parser::new(&content);
    pulldown_cmark::html::push_html(&mut html, parser);
    minify_html(html.trim().into())
}

pub fn get_config_bool(config: &Config, key: &str) -> bool {
    config
        .get(format!("preprocessor.embedify.{}", key).as_str())
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

pub fn get_config_string<'a>(config: &'a Config, key: &str, default: &'a str) -> &'a str {
    config
        .get(format!("preprocessor.embedify.{}", key).as_str())
        .and_then(|v| v.as_str())
        .unwrap_or(default)
}

pub fn create_embed(name: &str, options: Vec<(&str, &str)>) -> String {
    let mut option_str = String::new();
    for (key, value) in options {
        option_str.push_str(&format!(r#"{}="{}""#, key, value));
    }
    format!("{{% embed {} {} %}}", name, option_str)
}

pub fn create_announcement_banner(config: &Config) -> String {
    // get the config
    let id = get_config_string(config, "announcement-banner.id", "");
    let theme = get_config_string(config, "announcement-banner.theme", "default");
    let message = get_config_string(config, "announcement-banner.message", "");

    create_embed(
        "announcement-banner",
        vec![("id", id), ("theme", theme), ("message", message)],
    )
}

pub fn create_giscus(config: &Config) -> String {
    // get the config
    let repo = get_config_string(config, "giscus.repo", "");
    let repo_id = get_config_string(config, "giscus.repo-id", "");
    let category = get_config_string(config, "giscus.category", "");
    let category_id = get_config_string(config, "giscus.category-id", "");
    let reactions_enabled = get_config_string(config, "giscus.reactions-enabled", "1");
    let theme = get_config_string(config, "giscus.theme", "book");
    let lang = get_config_string(config, "giscus.lang", "en");
    let loading = get_config_string(config, "giscus.loading", "lazy");

    let options = vec![
        ("repo", repo),
        ("repo-id", repo_id),
        ("category", category),
        ("category-id", category_id),
        ("reactions-enabled", reactions_enabled),
        ("theme", theme),
        ("lang", lang),
        ("loading", loading),
    ];

    create_embed("giscus", options)
}

pub fn create_footer(config: &Config) -> String {
    // get the config
    let message = get_config_string(config, "footer.message", "");
    create_embed("footer", vec![("message", message)])
}

pub fn create_scroll_to_top() -> String {
    create_embed("scroll-to-top", vec![])
}
