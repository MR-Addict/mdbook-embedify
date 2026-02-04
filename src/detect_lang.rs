use lazy_static::lazy_static;
use mdbook_core::config::Config;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Deserialize, Clone)]
struct LanguageConfig {
    name: Option<String>,
    extensions: Option<Vec<String>>,
    filenames: Option<Vec<String>>,
}

lazy_static! {
    static ref STATIC_LANGUAGES: Vec<LanguageConfig> = {
        let yaml_content = include_str!("assets/config/languages.yaml");
        serde_yaml::from_str(yaml_content).expect("Failed to parse languages.yaml")
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

fn get_language_overrides(config: Option<&Config>) -> HashMap<String, LanguageConfig> {
    if let Some(config) = config {
        if let Ok(Some(languages)) =
            config.get::<HashMap<String, LanguageConfig>>("preprocessor.embedify.include.languages")
        {
            return languages;
        }
    }
    HashMap::new()
}

fn match_exact(filename: &str, config: &LanguageConfig) -> bool {
    if let Some(filenames) = &config.filenames {
        for pattern in filenames {
            if !pattern.contains('*') && !pattern.contains('?') {
                if pattern == filename {
                    return true;
                }
            }
        }
    }
    false
}

fn match_extension(filename: &str, config: &LanguageConfig) -> bool {
    if let Some(extensions) = &config.extensions {
        if let Some(last_dot_pos) = filename.rfind('.') {
            let extension = &filename[last_dot_pos..].to_lowercase();
            for ext in extensions {
                if ext.to_lowercase() == *extension {
                    return true;
                }
            }
        }
    }
    false
}

fn match_pattern(filename: &str, config: &LanguageConfig) -> bool {
    if let Some(filenames) = &config.filenames {
        for pattern in filenames {
            if pattern.contains('*') || pattern.contains('?') {
                // Wildcard match
                let regex_pattern = glob_to_regex(pattern);
                if let Ok(regex) = regex::Regex::new(&regex_pattern) {
                    if regex.is_match(filename) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn detect_lang(path: String, config: Option<&Config>) -> String {
    let path_obj = Path::new(&path);
    let filename = match path_obj.file_name().and_then(|f| f.to_str()) {
        Some(f) => f,
        None => return "plaintext".to_string(),
    };

    let overrides = get_language_overrides(config);
    let mut languages_check_list: Vec<(&str, &LanguageConfig)> =
        Vec::with_capacity(STATIC_LANGUAGES.len() + overrides.len());
    let mut processed_overrides = HashSet::new();

    // 1. Add static languages (checking for overrides)
    for config in STATIC_LANGUAGES.iter() {
        if let Some(name) = &config.name {
            if let Some(override_config) = overrides.get(name) {
                languages_check_list.push((name, override_config));
                processed_overrides.insert(name);
            } else {
                languages_check_list.push((name, config));
            }
        }
    }

    // 2. Add remaining overrides
    for (name, config) in &overrides {
        if !processed_overrides.contains(name) {
            languages_check_list.push((name, config));
        }
    }

    // Pass 1: Check Exact Filenames
    for (name, config) in &languages_check_list {
        if match_exact(filename, config) {
            return name.to_string();
        }
    }

    // Pass 2: Check Extensions
    for (name, config) in &languages_check_list {
        if match_extension(filename, config) {
            return name.to_string();
        }
    }

    // Pass 3: Check Patterns
    for (name, config) in &languages_check_list {
        if match_pattern(filename, config) {
            return name.to_string();
        }
    }

    "plaintext".to_string()
}
