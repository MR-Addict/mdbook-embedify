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
            if let Some(name) = lang["name"].as_str() {
                // Handle extensions
                if let Some(extensions) = lang["extensions"].as_array() {
                    for ext in extensions {
                        if let Some(extension) = ext.as_str() {
                            map.insert(extension.to_string(), name.to_string());
                        }
                    }
                }

                // Handle filenames
                if let Some(filenames) = lang["filenames"].as_array() {
                    for filename in filenames {
                        if let Some(filename_str) = filename.as_str() {
                            map.insert(filename_str.to_string(), name.to_string());
                        }
                    }
                }
            }
        }
        map
    };

    static ref LANGUAGE_PATTERNS: Vec<(regex::Regex, String)> = {
        let json_content = include_str!("assets/config/languages.json");
        let languages: Vec<serde_json::Value> =
            serde_json::from_str(json_content).expect("Failed to parse languages.json");

        let mut patterns = Vec::new();
        for lang in languages {
            if let Some(name) = lang["name"].as_str() {
                // Handle filename patterns
                if let Some(filename_patterns) = lang["filename_patterns"].as_array() {
                    for pattern in filename_patterns {
                        if let Some(pattern_str) = pattern.as_str() {
                            // Convert glob pattern to regex
                            let regex_pattern = glob_to_regex(pattern_str);
                            if let Ok(regex) = regex::Regex::new(&regex_pattern) {
                                patterns.push((regex, name.to_string()));
                            }
                        }
                    }
                }
            }
        }
        patterns
    };
}

/// Convert a glob pattern to a regex pattern
fn glob_to_regex(glob: &str) -> String {
    let mut regex = String::new();
    regex.push('^');

    for ch in glob.chars() {
        match ch {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            '.' => regex.push_str("\\."),
            '^' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '+' | '\\' => {
                regex.push('\\');
                regex.push(ch);
            }
            _ => regex.push(ch),
        }
    }

    regex.push('$');
    regex
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

            // Finally try filename patterns (for wildcard matching)
            for (pattern, language) in LANGUAGE_PATTERNS.iter() {
                if pattern.is_match(filename_str) {
                    return language.clone();
                }
            }
        }
    }

    // If no extension found or extension not in map, return "plaintext"
    "plaintext".to_string()
}
