mod template;
mod ui;

fn main() {

    let template_selection = ui::get_template_selection();

    match template_selection.as_str() {
        "Custom" => {
            ui::fill_content("custom".to_string());
        }
        _ => ui::fill_content("standard".to_string()),
    }
    
}
