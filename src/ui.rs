use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use dialoguer::console::Style;

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

pub fn confirm(prompt: &String) -> bool {
    dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

pub fn message(message: Result<String, String>) {
    match message {
        Ok(message) =>  println!(
            "{}",
            Style::new()
                .for_stderr()
                .green()
                .apply_to(format!("✔ {}", message))
        
        ),
        Err(message) => println!(
            "{}",
            Style::new()
                .for_stderr()
                .red()
                .apply_to(format!("✘ {}", message))
        
        ),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let options = vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
        ];
        let prompt = "Choose an option".to_string();
        let default = 1;

        let result = select(&options, prompt, default);

        assert_eq!(result, "Option 2");
    }
}
