use egui;
use rand::prelude::*;
use rand::seq::SliceRandom;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Password Generator by Veythrix",
        options,
        Box::new(|_cc| Ok(Box::new(PasswordGeneratorApp::default()))),
    );
}

struct PasswordGeneratorApp {
    password: String,
    length: usize,
    use_lowercase: bool,
    use_uppercase: bool,
    use_numbers: bool,
    use_symbols: bool,
    history: Vec<String>,
    password_strength: String,
}

impl Default for PasswordGeneratorApp {
    fn default() -> Self {
        Self {
            password: String::new(),
            length: 12,
            use_lowercase: true,
            use_uppercase: true,
            use_numbers: true,
            use_symbols: true,
            history: Vec::new(),
            password_strength: String::new(),
        }
    }
}

impl PasswordGeneratorApp {
    fn generate_password(&mut self) {
        let mut rng = rand::thread_rng();
        let mut characters = Vec::new();

        if self.use_lowercase {
            characters.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
        }
        if self.use_uppercase {
            characters.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.use_numbers {
            characters.extend_from_slice(b"0123456789");
        }
        if self.use_symbols {
            characters.extend_from_slice(b"!@#$%^&*()_+-=[]{}|;:,.<>?");
        }

        if characters.is_empty() {
            self.password = "Choose something".to_string();
            return;
        }

        self.password.clear();
        for _ in 0..self.length {
            if let Some(&c) = characters.choose(&mut rng) {
                self.password.push(c as char);
            }
        }

        self.history.push(self.password.clone());
        self.update_password_strength();
    }

    fn update_password_strength(&mut self) {
        let mut strength = 0;
        if self.use_lowercase {
            strength += 1;
        }
        if self.use_uppercase {
            strength += 1;
        }
        if self.use_numbers {
            strength += 1;
        }
        if self.use_symbols {
            strength += 1;
        }

        self.password_strength = match strength {
            1 => "Weak".to_string(),
            2 => "Medium".to_string(),
            3 => "Strong".to_string(),
            4 => "Very Strong".to_string(),
            _ => "Unknown".to_string(),
        };
    }
}

impl eframe::App for PasswordGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Password Generator by Veythrix");
	    ui.label("GitHub/CodeBerg");
            ui.hyperlink("https://github.com/Veythrix");
            ui.hyperlink("https://codeberg.org/Veythrix");
	    ui.label("Source Code");
            ui.hyperlink("https://codeberg.org/Veythrix/password-generator");
            ui.hyperlink("https://github.com/Veythrix/password-generator");
            ui.separator();

            ui.label("Password Length:");
            ui.add(egui::Slider::new(&mut self.length, 4..=32));

            ui.checkbox(&mut self.use_lowercase, "Include Lowercase Letters");
            ui.checkbox(&mut self.use_uppercase, "Include Uppercase Letters");
            ui.checkbox(&mut self.use_numbers, "Include Numbers");
            ui.checkbox(&mut self.use_symbols, "Include Symbols");

            if ui.button("Generate Password").clicked() {
                self.generate_password();
            }

            ui.separator();

            ui.label("Generated Password:");
            ui.text_edit_singleline(&mut self.password);

            ui.label(format!("Password Strength: {}", self.password_strength));

            if ui.button("Copy Password").clicked() {
                ctx.copy_text(self.password.clone());
            }

            ui.separator();

            ui.label("Password History:");
            for (i, password) in self.history.iter().enumerate() {
                ui.label(format!("{}: {}", i + 1, password));
            }
        });
    }
}
