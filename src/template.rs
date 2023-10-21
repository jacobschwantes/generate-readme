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

    for line in content.lines() {
        if line.starts_with("##") {
            if section_name != "" {
                blocks.push(Block {
                    name: section_name.trim_start_matches("?").to_string(),
                    content: block_content.join("\n"),
                    required: !section_name.starts_with("?"),
                });
                block_content.clear();
                section_name = "";
            } else {
                section_name = trim_header_prefix(line)
            }
            block_content.push(line.replace("## ?", "## "))
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
