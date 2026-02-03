use mdbook_core::config::Config;
use minify::html::minify;
use pulldown_cmark;
use regex::Regex;

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
        .get::<bool>(format!("preprocessor.embedify.{}", key).as_str())
        .ok()
        .flatten()
        .unwrap_or(false)
}

pub fn get_config_string<'a>(config: &'a Config, key: &str, default: &'a str) -> String {
    config
        .get::<String>(format!("preprocessor.embedify.{}", key).as_str())
        .ok()
        .flatten()
        .unwrap_or_else(|| default.to_string())
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
        vec![("id", &id), ("theme", &theme), ("message", &message)],
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
        ("repo", repo.as_str()),
        ("repo-id", repo_id.as_str()),
        ("category", category.as_str()),
        ("category-id", category_id.as_str()),
        ("reactions-enabled", reactions_enabled.as_str()),
        ("theme", theme.as_str()),
        ("lang", lang.as_str()),
        ("loading", loading.as_str()),
    ];

    create_embed("giscus", options)
}

pub fn create_footer(config: &Config) -> String {
    // get the config
    let message = get_config_string(config, "footer.message", "");
    create_embed("footer", vec![("message", &message)])
}

pub fn create_scroll_to_top() -> String {
    create_embed("scroll-to-top", vec![])
}
