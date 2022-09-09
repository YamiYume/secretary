use super::{EncryptTool, Tool, View};
use std::collections::{HashMap, HashSet};
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
        let Self {
            plaintext,
            ciphertext,
            key,
        } = self;
        let plaintext_edit = ui.add(
            egui::TextEdit::multiline(plaintext)
                .hint_text("Write your plaintext here")
        );
        let plaintext_popup_id = ui.make_persistent_id("Error_popup_plaintext");
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

impl EncryptTool for PermEnc {
    fn update_ciphertext(&mut self) -> () {
        let plaintext_whiteless: String = self
            .plaintext
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect();
        let mut ciphertext_build = String::from("");
        let perm_map = self.perm_map();
        for c in plaintext_whiteless.chars() {
            ciphertext_build.push(perm_map[&c]);
        }
        self.ciphertext = ciphertext_build;
    }
    fn valid_plaintext(&self) -> bool {
        self.plaintext
            .chars()
            .all(|c| c.is_ascii_lowercase() | c.is_ascii_whitespace())
    }
}

impl PermEnc {
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
