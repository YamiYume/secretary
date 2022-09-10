use super::{DecryptTool, Tool, View};
use std::collections::{HashMap, HashSet};
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
        "ﲙ  Permutation"
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
        let Self {
            plaintext,
            ciphertext,
            key,
        } = self;
        let ciphertext_edit = ui.add(
            egui::TextEdit::multiline(ciphertext)
                .hint_text("Write your ciphertext here")
        );
        let ciphertext_popup_id = ui.make_persistent_id("Error_popup_ciphertext");
        let key_edit = ui.add(
            egui::TextEdit::multiline(key)
                .hint_text("Write yor key here")
        );
        let key_popup_id = ui.make_persistent_id("Error_popup_key");
        ui.horizontal_top(|ui| {
            ui.add_enabled(
                false,
                egui::TextEdit::multiline(ciphertext).hint_text("Here will appear your ciphertext")
            );
            ui.vertical_centered_justified(|ui| {
                if ui.add(egui::Button::new("Copy")).clicked() {
                    ui.output().copied_text = ciphertext.to_string();
                }
                if ui.add(egui::Button::new("Copy Key")).clicked() {
                    ui.output().copied_text = key.to_string();
                }
            });
        });
        if (plaintext_edit.changed() || key_edit.changed()) 
            && !self.plaintext.is_empty() && !self.key.is_empty()  {
            let plaintext_is_valid = self.valid_plaintext();
            let key_is_valid = self.valid_key();
            if key_is_valid && plaintext_is_valid {
                ui.memory().close_popup();
                self.update_ciphertext();
            } else if key_is_valid {
                ui.memory().open_popup(plaintext_popup_id);
            } else {
                ui.memory().open_popup(key_popup_id);
            }
        }
        egui::popup_below_widget(ui, plaintext_popup_id, &plaintext_edit, |ui| {
            ui.code("Unvalid plaintext, must be lowercase alphabetic");
        });
        egui::popup_below_widget(ui, key_popup_id, &key_edit, |ui| {
            ui.code("Unvalid key, must be lowercase single word");
        });
    }
}

impl DecryptTool for PermDec {
    fn update_plaintext(&mut self) -> () {
        let mut plaintext_build = String::from("");
        let perm_map = self.perm_map();
        for c in self.ciphertext.chars() {
            plaintext_build.push(perm_map[&c]);
        }
        self.ciphertext = plaintext_build;
    }
    fn valid_ciphertext(&self) -> bool {
        self.ciphertext
            .chars()
            .all(|c| c.is_ascii_uppercase())
    }
}

impl PermDec {
    fn valid_key(&self) -> bool {
        self.key.len() == 26
        && self.key.chars().all(|c| c.is_lowercase())
        && self.key.len() == HashSet::<char>::from_iter(self.key.chars()).len()
    }
    fn perm_map(&self) -> HashMap<char, char> {
        let mut permutation = HashMap::new();
        for (i, c) in (0..26).zip(self.key.chars()) {
            permutation.insert(char::from_u32(i + 97).unwrap(), c.to_ascii_uppercase());
        }
        permutation
    }
}
