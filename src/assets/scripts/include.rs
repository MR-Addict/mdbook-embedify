use crate::parser;

use detect_lang;
use std::fs;

pub fn include_script(options: Vec<parser::EmbedAppOption>) -> Result<String, String> {
    // get the file path from the options
    let file_path = parser::get_option("file", options.clone());
    if file_path.is_none() {
        return Err("Option file is required".to_string());
    }
    let file_path = file_path.unwrap().value;

    // read the file content
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("Cannot find file {}", file_path));
    }

    let content = fs::read_to_string(&file_path).unwrap_or_else(|_| String::new());
    let content = content.trim().to_string();

    let lang_option = parser::get_option("lang", options);
    let lang_detected = detect_lang::from_path(&file_path);

    let language = match lang_option {
        Some(option) => option.value,
        _ => lang_detected.map_or("plaintext".to_string(), |lang| lang.id().to_string()),
    };

    // check if the language is markdown, if so, wrapped by backticks
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

        Ok(format!(
            "{}{}\n{}\n{}",
            backticks, language, content, backticks
        ))
    } else {
        Ok(format!("```{}\n{}\n```", language, content))
    }
}
