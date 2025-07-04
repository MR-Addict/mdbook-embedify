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
            // First, try to match the full filename (for cases like Makefile, Dockerfile, etc.)
            if let Some(language) = LANGUAGE_MAP.get(filename_str) {
                return language.clone();
            }

            // Then try the file extension (everything after the last dot)
            if let Some(last_dot_pos) = filename_str.rfind('.') {
                let extension = &filename_str[last_dot_pos..].to_lowercase();
                if let Some(language) = LANGUAGE_MAP.get(extension) {
                    return language.clone();
                }
            }
        }
    }

    // If no extension found or extension not in map, return "plaintext"
    "plaintext".to_string()
}
