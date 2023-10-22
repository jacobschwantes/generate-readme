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

pub fn load_custom_templates() -> Vec<Template> {
    let templates_dir = dirs::document_dir().unwrap().join("readme-templates");
    let mut templates: Vec<Template> = vec![];
    if let Ok(entries) = fs::read_dir(templates_dir) {
        for entry in entries {
            match entry {
                Ok(entry) => {
                    if is_markdown_file(entry.path().as_path()) {
                        let content =
                            fs::read_to_string(entry.path()).unwrap_or_else(|_| String::from(""));
                        let file_name = String::from(entry.file_name().to_str().unwrap());
                        match parse_raw_template_string(content, file_name) {
                            Ok(template) => templates.push(template),
                            Err(err) => println!("We ran into an error: {:?}", err),
                        }
                    }
                }
                Err(err) => {
                    println!("We ran into an error: {:?}", err);
                }
            }
        }
    } else {
        println!("couldnt read templates")
    }
    templates
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
