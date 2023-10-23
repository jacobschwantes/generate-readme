extern crate clipboard;
use crate::ui::{self, message};
use clipboard::{ClipboardContext, ClipboardProvider};
use include_dir::{include_dir, Dir};
use std::fs::{self};
use std::path::Path;

static TEMPLATES_DIR: Dir = include_dir!("templates");
pub struct Template {
    pub name: String,
    pub content: Vec<Block>,
    pub placeholders: Vec<String>,
}
pub struct Block {
    pub name: String,
    pub content: String,
    pub required: bool,
}

pub fn load_standard_templates() -> Vec<Template> {
    let mut templates: Vec<Template> = vec![];
    for entry in TEMPLATES_DIR.files() {
        let file_name = String::from(entry.path().to_str().unwrap());
        let content = entry.contents_utf8().unwrap().to_string();
        let parsed = parse_raw_template_string(content, file_name);
        templates.push(parsed.unwrap())
    }
    templates
}

pub fn load_custom_templates() -> Result<Vec<Template>, String> {
    // TODO: add support for setting/reading from custom templates directory
    let templates_dir = dirs::document_dir().unwrap().join("readme-templates");
    let mut templates: Vec<Template> = vec![];
    match fs::read_dir(templates_dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if is_markdown_file(entry.path().as_path()) {
                            let content = fs::read_to_string(entry.path())
                                .unwrap_or_else(|_| String::from(""));
                            let file_name = String::from(entry.file_name().to_str().unwrap());
                            match parse_raw_template_string(content, file_name) {
                                Ok(template) => templates.push(template),
                                Err(err) => message(Err(err.to_string())),
                            }
                        }
                    }
                    Err(err) => {
                        message(Err(err.to_string()));
                    }
                }
            }
        }
        Err(err) => return Err(err.to_string()),
    }
    match templates.is_empty() {
        true => Err("No templates found in ~/readme-templates".to_string()),
        false => Ok(templates),
    }
}

fn parse_raw_template_string(content: String, file_name: String) -> Result<Template, String> {
    let mut blocks: Vec<Block> = vec![];
    let mut block_content: Vec<String> = vec![];
    let mut section_name = "";
    let mut placeholders: Vec<String> = vec![];
    let lines: Vec<&str> = content.lines().collect();
    let last_index = lines.len() - 1;
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("##") {
            if section_name != "" {
                blocks.push(Block {
                    name: section_name.trim_start_matches("?").to_string(),
                    content: block_content.join("\n"),
                    required: !section_name.starts_with("?"),
                });
                block_content.clear();
                section_name = trim_header_prefix(line);
            } else {
                section_name = trim_header_prefix(line)
            }
            block_content.push(line.replace("## ?", "## "))
        } else if i == last_index {
            block_content.push(line.to_string());
            blocks.push(Block {
                name: section_name.trim_start_matches("?").to_string(),
                content: block_content.join("\n"),
                required: !section_name.starts_with("?"),
            });
        } else if line.starts_with("Placeholders") {
            let collected: Vec<String> = line
                .trim_start_matches("Placeholders: ")
                .split(",")
                .map(String::from)
                .collect();
            placeholders = collected
        } else {
            block_content.push(line.to_string());
        }
    }
    Ok(Template {
        name: file_name,
        content: blocks,
        placeholders,
    })
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map_or(false, |ext| ext == "md" || ext == "markdown")
}

fn trim_header_prefix(line: &str) -> &str {
    let mut trimmed = line;
    while trimmed.starts_with('#') {
        trimmed = &trimmed[1..];
    }
    trimmed.trim_start()
}

pub fn fill_placeholders(content: String, placeholders: Vec<String>) -> String {
    let mut replaced = content;
    for placeholder in placeholders {
        let formated = placeholder
            .replace("[", "")
            .replace("]", "")
            .replace("_", " ")
            .to_string();
        let user_input = ui::input(&format!("Enter a {}", formated), &formated);
        replaced = replaced.replace(&placeholder, &user_input)
    }
    replaced
}

fn is_valid_filename(filename: &str) -> bool {
    if !filename.is_ascii() {
        return false;
    }
    let invalid_chars = r#"<>:\"/\|?*"#;
    if filename
        .chars()
        .any(|c| invalid_chars.contains(c) || c.is_control())
    {
        return false;
    }
    true
}

pub fn write_file(content: String) {
    let selection = ui::select(
        &vec![
            "Save to current directory".to_string(),
            "Copy to clipboard".to_string(),
            "Output to console".to_string(),
        ],
        "What would you like to do with the generated README?".to_string(),
        0,
    );

    match selection.as_str() {
        "Copy to clipboard" => {
            let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
            clipboard.set_contents(content.to_owned()).unwrap();
            ui::message(Ok("README copied to clipboard successfully".to_string()))
        }
        "Save to current directory" => match Path::new("README.md").exists() {
            // file exists already, prompt user to overwrite
            true => match ui::confirm(&"README.md already exists. Overwrite?".to_string()) {
                // overwrite
                true => match fs::write("README.md", content) {
                    Ok(_) => ui::message(Ok("README generated successfully".to_string())),
                    Err(err) => ui::message(Err(err.to_string())),
                },
                // don't overwrite
                false => {
                    // prompt user to save to a different file name
                    match ui::confirm(
                        &"Would you like to save to a different file name?".to_string(),
                    ) {
                        true => {
                            // prompt user for new file name
                            let get_file_name = || {
                                let new_file_name = ui::input(
                                    &"Enter a new file name".to_string(),
                                    &"README".to_string(),
                                ) + ".md";
                                let is_valid = !Path::new(&new_file_name).exists()
                                    && is_valid_filename(&new_file_name);

                                return (is_valid, new_file_name);
                            };
                            // recursively prompt user for new file name until valid
                            match get_file_name() {
                                (true, new_file_name) => match fs::write(new_file_name, content) {
                                    Ok(_) => {
                                        ui::message(Ok("README generated successfully".to_string()))
                                    }
                                    Err(err) => ui::message(Err(err.to_string())),
                                },
                                (false, _) => {
                                    ui::message(Err(
                                        "Invalid name or file already exists".to_string()
                                    ));
                                    write_file(content);
                                }
                            }
                        }
                        // return to main menu
                        false => write_file(content),
                    }
                }
            },
            // file doesn't exist, write to README.md
            false => match fs::write("README.md", content) {
                Ok(_) => ui::message(Ok("README generated successfully".to_string())),
                Err(err) => ui::message(Err(err.to_string())),
            },
        },
        _ => println!("{}", content),
    }
}

// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_standard_templates() {
        let templates = load_standard_templates();
        assert!(!templates.is_empty());
    }

    #[test]
    fn test_parse_raw_template_string() {
        let content = String::from("## ?Section 1\nContent 1\n## Section 2\nContent 2");
        let file_name = String::from("test.md");

        let template = parse_raw_template_string(content, file_name).unwrap();
        println!("{:?}", template.content[1].content);
        assert_eq!(template.name, "test.md");
        assert_eq!(template.placeholders, Vec::<String>::new());
        assert_eq!(template.content.len(), 2);
        assert_eq!(template.content[0].name, "Section 1");
        assert_eq!(template.content[0].content, "## Section 1\nContent 1");
        assert_eq!(template.content[0].required, false);
        assert_eq!(template.content[1].name, "Section 2");
        assert_eq!(template.content[1].content, "## Section 2\nContent 2");
        assert_eq!(template.content[1].required, true);
    }

    #[test]
    fn test_is_markdown_file() {
        let path = Path::new("test.md");
        assert!(is_markdown_file(path));
        let path = Path::new("test.markdown");
        assert!(is_markdown_file(path));
        let path = Path::new("test.txt");
        assert!(!is_markdown_file(path));
    }

    #[test]
    fn test_trim_header_prefix() {
        let line = "## ?Section 1";
        assert_eq!(trim_header_prefix(line), "?Section 1");
        let line = "### Section 2";
        assert_eq!(trim_header_prefix(line), "Section 2");
    }
}
