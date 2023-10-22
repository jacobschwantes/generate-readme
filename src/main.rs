use ui::message;

mod template;
mod ui;

fn main() {
    let mut template_mode = ui::select(
        &vec!["Standard".to_string(), "Custom".to_string()],
        "Choose a template category".to_string(),
        0,
    );

    let standard_templates = template::load_standard_templates();

    let templates = match template_mode.as_str() {
        "Custom" => match template::load_custom_templates() {
            Ok(templates) => templates,
            Err(err) => {
                message(Err(format!("{}", err)));
                template_mode = "Standard".to_string();
                standard_templates
            }
        },
        _ => standard_templates,
    };

    let templates_names_only: Vec<String> = templates
        .iter()
        .map(|template| template.name.trim_end_matches(".md").to_string())
        .collect();
    let template_selection = ui::select(
        &templates_names_only,
        format!("Choose a template from {}", template_mode).to_string(),
        0,
    );
    let template = templates
        .iter()
        .find(|&template| template.name.starts_with(&template_selection))
        .unwrap();
    let section_blocks = &template.content;
    let section_names_only = section_blocks
        .iter()
        .map(|block| block.name.clone())
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
