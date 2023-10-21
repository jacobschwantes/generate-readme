extern crate clipboard;
use crate::template;
use dialoguer::console::Style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use std::fs::{self};
use clipboard::{ClipboardContext, ClipboardProvider};

pub fn fill_content(mode: String) {
    let templates = match mode.as_str() {
        "standard" => template::load_standard_templates(),
        _ => template::load_custom_templates()
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