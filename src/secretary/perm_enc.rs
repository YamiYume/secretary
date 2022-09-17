use super::{Tool, View};
use std::collections::HashSet;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct PermEnc {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: String,
}

impl Default for PermEnc {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: String::from(""),
        }
    }
}

impl Tool for PermEnc {
    fn name(&self) -> &'static str {
        "ﲘ Permutation"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> () {
        egui::Window::new(self.name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl View for PermEnc {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {
        let (plaintext_edit, plain_error) = super::plaintext_input(&mut self.plaintext, ui);
        let (key_edit, key_error) = super::key_input(&mut self.key, ui);

        super::ciphertext_output(&mut self.ciphertext, &vec![&self.key], ui);

        if (plaintext_edit.changed() || key_edit.changed())
            && !self.plaintext.is_empty()
            && !self.key.is_empty()
        {
            let plaintext_is_valid = super::valid_plaintext(&self.plaintext);
            let key_is_valid = self.valid_key();

            if key_is_valid && plaintext_is_valid {
                ui.memory().close_popup();
                self.update_ciphertext();
            } else if key_is_valid {
                ui.memory().open_popup(plain_error);
            } else {
                ui.memory().open_popup(key_error);
            }
        }
        egui::popup_below_widget(ui, plain_error, &plaintext_edit, |ui| {
            ui.code("Unvalid plaintext, must be lowercase alphabetic");
        });
        egui::popup_below_widget(ui, key_error, &key_edit, |ui| {
            ui.code("Unvalid key, must be digits single word");
        });
    }
}

impl PermEnc {
    fn update_ciphertext(&mut self) -> () {
        let mut new_ciphertext = String::from("");
        let key_vec: Vec<u32> = self.key.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut accumulator: Vec<char> = Vec::new();
        for c in super::whiteless(&self.plaintext).chars() {
            accumulator.push(c);
            if accumulator.len() == key_vec.len() {
                new_ciphertext = format!(
                    "{} {}",
                    new_ciphertext,
                    PermEnc::cipher_slice(accumulator, &key_vec)
                );
                accumulator = Vec::new();
            }
        }
        if accumulator.len() != 0 {
            accumulator.append(&mut vec!['a'; key_vec.len() - accumulator.len()]);
            new_ciphertext = format!(
                "{} {}",
                new_ciphertext,
                PermEnc::cipher_slice(accumulator, &key_vec)
            );
        }
        self.ciphertext = new_ciphertext.trim().to_string();
    }

    fn cipher_slice(slice: Vec<char>, key: &Vec<u32>) -> String {
        let mut new_slice = Vec::new();
        for i in key {
            new_slice.push(slice[*i as usize - 1].to_ascii_uppercase());
        }
        new_slice.iter().collect::<String>()
    }

    fn valid_key(&self) -> bool {
        self.key.len() < 10
            && self.key.chars().all(|c| c.is_ascii_digit())
            && self.key.len() == self.key.chars().collect::<HashSet<char>>().len()
            && !self.key.contains("0")
            && self
                .key
                .chars()
                .all(|c| c.to_digit(10).unwrap() <= self.key.len() as u32)
    }
}
