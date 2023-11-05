mod template;
mod ui;
use std::path::PathBuf;

use clap::{arg, builder::styling, Command};
fn main() {
    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
        .error(styling::AnsiColor::Red.on_default() | styling::Effects::BOLD);
    
    let cmd = Command::new("generate-readme")
        .version("0.1.1")
        .about("Create templated README.md files from the terminal.")
        .styles(styles)
        .args([arg!(-t --"template-dir" <DIR> "Sets a custom directory for templates")])
        .get_matches();

    let templates_dir = match cmd.get_one::<String>("template-dir").map(String::as_str) {
        Some(t_dir) => PathBuf::from(t_dir),
        _ => dirs::document_dir().unwrap().join("readme-templates"),
    };
    let mut template_mode = ui::select(vec!["Standard", "Custom"], "Choose a template category", 0);

    let standard_templates = template::load_standard_templates();

    let templates = match template_mode.as_str() {
        "Custom" => match template::load_custom_templates(templates_dir) {
            Ok(templates) => templates,
            Err(err) => {
                ui::message(Err(err));
                template_mode = "Standard".to_string();
                standard_templates
            }
        },
        _ => standard_templates,
    };

    let templates_names_only: Vec<&str> = templates
        .iter()
        .map(|template| template.name.trim_end_matches(".md"))
        .collect();

    let template_selection = ui::select(
        templates_names_only,
        format!("Choose a template from {}", template_mode).as_str(),
        0,
    );

    let template = templates
        .iter()
        .find(|&template| template.name.starts_with(&template_selection))
        .unwrap();

    let section_blocks = &template.content;

    let section_names_only = section_blocks
        .iter()
        .map(|block| block.name.as_str())
        .collect();

    let section_defaults = section_blocks
        .iter()
        .map(|block| block.required.clone())
        .collect();

    let section_selection = ui::multi_select(
        &section_names_only,
        "Choose which sections to include (toggle using spacebar)",
        section_defaults,
    );
    let collected_with_chosen_sections = section_selection
        .iter()
        .map(|&index| section_blocks[index].content.clone())
        .collect::<Vec<_>>()
        .join("\n");

    let filled = template::fill_placeholders(
        collected_with_chosen_sections,
        template.placeholders.clone(),
    );

    template::write_file(filled);
}
