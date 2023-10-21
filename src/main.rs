use generate_readme::{fill_content};


fn main() {
    // let templates = get_templates();

    let template_selection = generate_readme::get_template_selection();

    match template_selection.as_str() {
        "Standard" => {
            fill_content("standard".to_string());
        }
        "Custom" => fill_content("custom".to_string()),
        _ => println!("other"),
    }
}
