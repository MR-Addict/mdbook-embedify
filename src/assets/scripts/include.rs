use crate::parser;

use detect_lang;
use std::fs;

pub fn include_script(options: Vec<parser::EmbedAppOption>) -> String {
    // get the file path from the options
    let file_path = parser::get_option("file", options.clone());
    if file_path.is_none() {
        panic!("File path is required for include script");
    }
    let file_path = file_path.unwrap().value;

    // read the file content
    let content = fs::read_to_string(file_path.clone()).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", file_path, err);
        String::new()
    });
    let content = if content.is_empty() {
        "[Empty content]".to_string()
    } else {
        content
    };

    let lang_option = parser::get_option("lang", options);
    let lang_detected = detect_lang::from_path(&file_path);

    let language = match lang_option {
        Some(option) => option.value,
        _ => lang_detected.map_or("plaintext".to_string(), |lang| lang.id().to_string()),
    };

    // check if the language is markdown, if so, wrapped by ```` instead of ```
    if lang_detected == Some(detect_lang::Language("Markdown", "markdown")) {
        format!("````{}\n{}\n````", language, content)
    } else {
        format!("```{}\n{}\n```", language, content)
    }
}
