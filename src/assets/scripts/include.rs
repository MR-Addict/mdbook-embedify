use crate::parser;

use detect_lang;
use std::fs;

pub fn include_script(options: Vec<parser::EmbedAppOption>) -> Option<String> {
    // get the file path from the options
    let file_path = parser::get_option("file", options.clone());
    if file_path.is_none() {
        return None;
    }
    let file_path = file_path.unwrap().value;

    // read the file content
    let content = fs::read_to_string(file_path.clone()).unwrap_or_else(|_| String::new());
    let content = if content.is_empty() {
        return None;
    } else {
        content.trim().to_string()
    };

    let lang_option = parser::get_option("lang", options);
    let lang_detected = detect_lang::from_path(&file_path);

    let language = match lang_option {
        Some(option) => option.value,
        _ => lang_detected.map_or("plaintext".to_string(), |lang| lang.id().to_string()),
    };

    // check if the language is markdown, if so, wrapped by ```` instead of ```
    if lang_detected == Some(detect_lang::Language("Markdown", "markdown")) {
        // Count the maximum number of consecutive backticks in the content
        let max_backticks = content
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.starts_with("```") {
                    Some(trimmed.chars().take_while(|&c| c == '`').count())
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(2) // Default to 2 if no backticks found
            + 1; // Add 1 to ensure we have enough backticks

        let backticks = "`".repeat(max_backticks);

        Some(format!(
            "{}{}\n{}\n{}",
            backticks, language, content, backticks
        ))
    } else {
        Some(format!("```{}\n{}\n```", language, content))
    }
}
