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
    let content = fs::read_to_string(file_path.clone()).unwrap_or_else(|_| {
        panic!("File {} not found for include", file_path);
    });

    let language = parser::get_option("lang", options);
    let language = if language.is_none() {
        // detect the language from the content
        detect_lang::from_path(&file_path).unwrap().id()
    } else {
        &language.unwrap().value
    };

    // format the content as a code block
    format!("```{}\n{}\n```", language, content.trim())
}
