use lazy_static::lazy_static;
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
