use dialoguer::console::Style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};

pub fn select(options: Vec<&str>, prompt: &str, default: usize) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .items(&options)
        .max_length(10)
        .interact()
        .unwrap();
    options[selection].to_string()
}

pub fn input(prompt: &str, default: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()
        .unwrap()
}

pub fn multi_select(options: &Vec<&str>, prompt: &str, defaults: Vec<bool>) -> Vec<usize> {
    MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&options)
        .defaults(&defaults[..])
        .interact()
        .unwrap()
}

pub fn confirm(prompt: &str) -> bool {
    dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

pub fn message(message: Result<&str, String>) {
    match message {
        Ok(message) => println!(
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