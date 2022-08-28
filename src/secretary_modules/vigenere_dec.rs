use super::{DecryptTool, Tool, View};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct VigenereDec {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: String,
}

impl Default for VigenereDec {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: String::from(""),
        }
    }
}

impl Tool for VigenereDec {
    fn name(&self) -> &'static str {
        "ﲙ Vigenere"
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

impl View for VigenereDec {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {
        let Self {
            plaintext,
            ciphertext,
            key,
        } = self;
        let ciphertext_edit = egui::TextEdit::multiline(ciphertext)
            .hint_text("Write your ciphertext here")
            .show(ui);
        let popup_id_ciphertext = ui.make_persistent_id("Error_popup_ciphertext");
        let key_edit = egui::TextEdit::singleline(key)
            .hint_text("Write your key here")
            .show(ui);
        let popup_id_key = ui.make_persistent_id("Error_popup_key");
        ui.horizontal_top(|ui| {
            ui.add_enabled(false, egui::TextEdit::multiline(plaintext).hint_text("here"));
            ui.vertical_centered_justified(|ui| {
                if ui.add(egui::Button::new("Copy")).clicked() {
                    ui.output().copied_text = plaintext.to_string();
                }
                if ui.add(egui::Button::new("Copy Key")).clicked() {
                    ui.output().copied_text = key.to_string();
                }
            });
        });
        if (ciphertext_edit.response.changed() || key_edit.response.changed()) && (!key.is_empty() && !ciphertext.is_empty()) {
            let ciphertext_is_valid = self.valid_ciphertext();
            let key_is_valid = self.valid_key();
            if ciphertext_is_valid && key_is_valid{
                ui.memory().close_popup();
                self.update_plaintext();
            } else if !ciphertext_is_valid{
                ui.memory().open_popup(popup_id_ciphertext);
            } else {
                ui.memory().open_popup(popup_id_key);
            }
        }
        egui::popup::popup_below_widget(ui, popup_id_ciphertext, &ciphertext_edit.response, |ui| {
            ui.code("Unvalid ciphertext, ciphertext must be uppercase single word");
        });
        egui::popup::popup_below_widget(ui, popup_id_key, &key_edit.response, |ui| {
            ui.code("Unvalid key, key must be lowercase single word");
        });
    }
}

impl DecryptTool for VigenereDec {
    fn valid_ciphertext(&self) -> bool {
        self.ciphertext
            .chars()
            .all(|c| c.is_ascii_uppercase())
    }
    fn update_plaintext(&mut self) -> () {
        let key_vector: Vec<i32> = self
            .key
            .chars()
            .map(|c| c as i32 - 96)
            .collect::<Vec<i32>>();
        let mut plaintext_build = String::from("");
        for c in self.ciphertext.char_indices() {
            let position = (c.1 as i32 - &key_vector[c.0 % key_vector.len()] - 65).rem_euclid(26) + 97;
            plaintext_build.push(char::from_u32(position as u32).unwrap());
        }
        self.plaintext = plaintext_build;
    }
}

impl VigenereDec {
    fn valid_key(&self) -> bool {
        self.key
            .chars()
            .all(|c| c.is_ascii_lowercase())
    }
}
