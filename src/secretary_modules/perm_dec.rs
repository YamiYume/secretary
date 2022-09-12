use super::{Tool, View};
use std::collections::HashSet;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct PermDec {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: String,
}

impl Default for PermDec {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: String::from(""),
        }
    }
}

impl Tool for PermDec {
    fn name(&self) -> &'static str {
        "ﲙ Permutation"
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

impl View for PermDec {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {
        let (ciphertext_edit, cipher_error) = super::ciphertext_input(&mut self.ciphertext, ui);
        let (key_edit, key_error) = super::key_input(&mut self.key, ui);
        
        super::plaintext_output(&mut self.plaintext, &vec![&self.key], ui);

        if (ciphertext_edit.changed() || key_edit.changed()) 
            && !self.ciphertext.is_empty() && !self.key.is_empty()  {

            let ciphertext_is_valid = PermDec::valid_ciphertext(&self.ciphertext);
            let key_is_valid = self.valid_key();

            if key_is_valid && ciphertext_is_valid {
                ui.memory().close_popup();
                self.update_plaintext();
            } else if key_is_valid {
                ui.memory().open_popup(cipher_error);
            } else {
                ui.memory().open_popup(key_error);
            }
        }
        egui::popup_below_widget(ui, cipher_error, &ciphertext_edit, |ui| {
            ui.code("Unvalid ciphertext, must be uppercase fixed lenght words");
        });
        egui::popup_below_widget(ui, key_error, &key_edit, |ui| {
            ui.code("Unvalid key, must be digits single word");
        });
    }
}

impl PermDec {
    fn update_plaintext(&mut self) -> () {
        let mut new_plaintext = String::from("");
        let key_vec: Vec<u32> = self.key
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        let mut accumulator: Vec<char> = Vec::new();
        for c in super::whiteless(&self.ciphertext).chars() {
            accumulator.push(c);
            if accumulator.len() == key_vec.len() {
                new_plaintext = format!("{}{}", new_plaintext, PermDec::cipher_slice(accumulator, &key_vec));
                accumulator = Vec::new();
            }
        }
        if accumulator.len() != 0 {
            accumulator.append(&mut vec!['A'; key_vec.len() - accumulator.len()]);
            new_plaintext = format!(
                "{}{}",
                new_plaintext,
                PermDec::cipher_slice(accumulator, &key_vec)
            );
        }
        self.plaintext = new_plaintext;
    }

    fn cipher_slice(slice: Vec<char>, key: &Vec<u32>) -> String {
        let mut new_slice = vec!['a'; slice.len()];
        for (i, c) in key.iter().zip(slice) {
            new_slice[*i as usize - 1] = c.to_ascii_lowercase();
        }
        new_slice.iter().collect::<String>()
    }


    fn valid_key(&self) -> bool {
        self.key.len() < 10
        && self.key
            .chars()
            .all(|c| c.is_ascii_digit())
        && self.key.len() == self.key.chars().collect::<HashSet<char>>().len()
        && !self.key.contains("0")
        && self.key.chars().all(|c| c.to_digit(10).unwrap() <= self.key.len() as u32)
    }

    fn valid_ciphertext(ciphertext :&String) -> bool {
        let mut answer = ciphertext
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_whitespace());
        let split: Vec<&str> = ciphertext.split(" ").collect();
        let theorical_len = (ciphertext.len() - split.len() + 1) / split.len();
        for slice in ciphertext.split_whitespace() {
            answer &= slice.len() == theorical_len;
            if !answer {
                break
            }
        }
        answer
    }
}
