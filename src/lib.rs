extern crate clipboard;
use dialoguer::console::Style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use std::fs::{self};
use std::path::Path;
use clipboard::{ClipboardContext, ClipboardProvider};
use include_dir::{include_dir, Dir};
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


fn load_standard_templates() -> Vec<Template> {
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
                    let content = fs::read_to_string(entry.path()).unwrap_or_else(|_| String::from(""));
                    let file_name = String::from(entry.file_name().to_str().unwrap());
                    match parse_raw_template_string(content, file_name) {
                    Ok(template) => templates.push(template),
                    Err(err) => println!("We ran into an error: {:?}", err),}
                }},
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

pub fn fill_content(mode: String) {
    let templates = match mode.as_str() {
        "standard" => load_standard_templates(),
        _ => load_custom_templates()
    };
    let names_only: Vec<String> = templates
        .iter()
        .map(|template| template.name.trim_end_matches(".md").to_string())
        .collect();

    let selection = select(&names_only, format!("Choose a template from {}", mode).to_string(), 0);
    let selected_template = templates.iter().find(|&template| template.name.starts_with(&selection)).unwrap();
    let blocks = &selected_template.content;
    let names_only = blocks.iter().map(|block| block.name.clone()).collect();
    let defaults = blocks.iter().map(|block| block.required.clone()).collect();
    let section_selection = multi_select(&names_only, "Choose which sections to include (toggle using spacebar)", defaults);
    let output = section_selection
        .iter()
        .map(|&index| blocks[index].content.clone())
        .collect::<Vec<_>>()
        .join("\n");
    let mut replaced = output;

    for placeholder in &selected_template.placeholders {
        println!("{}", placeholder);
        let formated = placeholder.replace("[", "").replace("]", "").replace("_", " ").to_string();
        let user_input = input(&format!("Enter a value for {}", formated), &formated);
        replaced = replaced.replace(placeholder, &user_input)
    }
    write_file(replaced);

   
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

// get template option, this is the first thing we prompt user
pub fn get_template_selection() -> String {
    let modes = vec!["Standard".to_string(), "Custom".to_string()];
    select(&modes, "Choose a template category".to_string(), 0)

}


pub fn select(options: &Vec<String>, prompt: String, default: usize) -> String {
   let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .items(&options)
        .interact()
        .unwrap();
    options[selection].clone()
}
pub fn input(prompt: &String, default: &String) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()
        .unwrap()
}
pub fn multi_select(options: &Vec<String>, prompt: &str, defaults: Vec<bool>) -> Vec<usize> {
    MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&options)
        .defaults(&defaults[..])
        .interact()
        .unwrap()
}

pub fn write_file(content: String) {
    let options: Vec<String> = vec![
        "Save to current directory".to_string(),
        "Copy to clipboard".to_string(),
        "Output to console".to_string(),
    ];

    let selection = select(
        &options,
        "What would you like to do with the generated README?".to_string(),
        0,
    );
    
    match selection.as_str() {
        "Output to console" => {
            println!("{}", content)
        }
        "Copy to clipboard" => {
            let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
            clipboard.set_contents(content.to_owned()).unwrap();
            println!(
                "{}",
                Style::new()
                    .for_stderr()
                    .green()
                    .apply_to("✔ README copied to clipboard successfully")
            )
        }
        "Save to current directory" => {fs::write("README.md", content).expect("Unable to write to README.md");
        println!(
            "{}",
            Style::new()
                .for_stderr()
                .green()
                .apply_to("✔ README generated successfully\n  Please take a look at it and make changes if necessary")
        
        )}
        _ => println!("unmatched"),
    }
    
}
