use mdbook::Config;
use pulldown_cmark;

pub fn render_to_markdown(content: String) -> String {
    let mut html = String::new();
    let parser = pulldown_cmark::Parser::new(&content);
    pulldown_cmark::html::push_html(&mut html, parser);
    html.trim().into()
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
