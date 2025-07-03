use crate::parser;

use hyperpolyglot;
use mdbook::preprocess::PreprocessorContext;
use std::fs;

fn parse_include_range(input: String, max_line: usize) -> Result<(usize, usize), String> {
    let trimmed = input.trim();

    if trimmed.is_empty() || trimmed == "-" {
        return Ok((1, max_line));
    }

    let parts: Vec<&str> = trimmed.split('-').collect();

    if parts.len() != 2 {
        return Ok((1, max_line));
    }

    let start = parts[0].trim();
    let end = parts[1].trim();

    let start_line = if start.is_empty() {
        1
    } else {
        match start.parse::<usize>() {
            Ok(n) if n >= 1 && n <= max_line => n,
            _ => return Err("Invalid or out-of-order start line number".to_string()),
        }
    };

    let end_line = if end.is_empty() {
        max_line
    } else {
        match end.parse::<usize>() {
            Ok(n) if n >= start_line && n <= max_line => n,
            _ => return Err("Invalid or out-of-order end line number".to_string()),
        }
    };

    Ok((start_line, end_line))
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
    let range = match range_option.map(|option| parse_include_range(option.value.clone(), max_line))
    {
        Some(Ok(range)) => range,
        Some(Err(err)) => return Err(err),
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
    let lang_detected = hyperpolyglot::detect(std::path::Path::new(&file_path))
        .ok()
        .flatten();

    let language = match lang_option {
        Some(option) => option.value,
        _ => lang_detected.map_or("plaintext".to_string(), |detection| {
            detection.language().to_lowercase()
        }),
    };

    // check if the language is markdown, if so, wrapped by backticks
    if lang_detected.map_or(false, |detection| detection.language() == "Markdown") {
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
