use crate::detect_lang;
use crate::parser;

use mdbook::preprocess::PreprocessorContext;
use std::fs;

fn wrap_content_in_code_block(content: &str, language: &str, lang_detected: &str) -> String {
    // check if the language is markdown, if so, wrapped by backticks
    if lang_detected == "markdown" {
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

        format!("{}{}\n{}\n{}", backticks, language, content, backticks)
    } else {
        format!("```{}\n{}\n```", language, content)
    }
}

fn parse_include_range(input: String, max_line: usize) -> (usize, usize) {
    let trimmed = input.trim();

    if trimmed.is_empty() || trimmed == "-" {
        return (1, max_line);
    }

    // Split by dash, but only accept exactly 2 parts
    let parts: Vec<&str> = trimmed.split('-').collect();

    if parts.len() != 2 {
        return (1, max_line);
    }

    let start = parts[0].trim();
    let end = parts[1].trim();

    let start_line = if start.is_empty() {
        1
    } else {
        match start.parse::<i32>() {
            Ok(n) if n >= 1 && n as usize <= max_line => n as usize,
            _ => 1, // Return 1 for invalid start line
        }
    };

    let end_line = if end.is_empty() {
        max_line
    } else {
        match end.parse::<i32>() {
            Ok(n) if n >= start_line as i32 && n as usize <= max_line => n as usize,
            _ => max_line, // Return max_line for invalid end line
        }
    };

    (start_line, end_line)
}

pub fn include_script(
    ctx: &PreprocessorContext,
    options: Vec<parser::EmbedAppOption>,
) -> Result<String, String> {
    // get the file path from the options
    let file_path = parser::get_option("file", options.clone());
    if file_path.is_none() {
        return Err("Option file is required".to_string());
    }
    let file_path = ctx.root.join(&file_path.unwrap().value);
    let file_path = file_path.to_string_lossy().to_string();

    // read the file content
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("Cannot find file {}", file_path));
    }

    let content = fs::read_to_string(&file_path).unwrap_or_else(|_| String::new());

    // get the range from the options
    let range_option = parser::get_option("range", options.clone());
    let max_line = content.lines().count();
    let range = match range_option {
        Some(option) => parse_include_range(option.value.clone(), max_line),
        _ => (1, max_line), // Default to the full range if no range is provided
    };

    // get the content from the range
    let start = range.0 - 1; // convert to 0-based index
    let end = range.1; // end is exclusive
    let lines: Vec<&str> = content.lines().collect();
    let content = lines[start..end].join("\n");

    // if include type is raw, return the content as is
    if let Some(option) = parser::get_option("type", options.clone()) {
        if option.value == "raw" {
            return Ok(content);
        }
    }

    // if include type is not raw, wrap the content in a code block
    let lang_option = parser::get_option("lang", options.clone());
    let lang_detected = detect_lang::detect_lang(file_path.clone());

    let language = match lang_option {
        Some(option) => option.value,
        _ => lang_detected.clone(),
    };

    Ok(wrap_content_in_code_block(
        &content,
        &language,
        &lang_detected,
    ))
}
