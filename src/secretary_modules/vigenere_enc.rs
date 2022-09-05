use super::{EncryptTool, Tool, View};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct VigenereEnc {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: String,
}

impl Default for VigenereEnc {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: String::from(""),
        }
    }
}

impl Tool for VigenereEnc {
    fn name(&self) -> &'static str {
        "ﲘ Vigenere"
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

impl View for VigenereEnc {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {
        let Self {
            plaintext,
            ciphertext,
            key,
        } = self;
        let plaintext_edit = egui::TextEdit::multiline(plaintext)
            .hint_text("Write your plaintext here")
            .show(ui);
        let popup_id_plaintext = ui.make_persistent_id("Error_popup_plaintext");
        let key_edit = egui::TextEdit::singleline(key)
            .hint_text("Write your key here")
            .show(ui);
        let popup_id_key = ui.make_persistent_id("Error_popup_key");
        ui.horizontal_top(|ui| {
            ui.add_enabled(false, egui::TextEdit::multiline(ciphertext).hint_text("Here will appear you ciphertext"));
            ui.vertical_centered_justified(|ui| {
                if ui.add(egui::Button::new("Copy")).clicked() {
                    ui.output().copied_text = ciphertext.to_string();
                }
                if ui.add(egui::Button::new("Copy Key")).clicked() {
                    ui.output().copied_text = key.to_string();
                }
            });
        });
        if (plaintext_edit.response.changed() || key_edit.response.changed()) && (!key.is_empty() && !plaintext.is_empty()) {
            let plaintext_is_valid = self.valid_plaintext();
            let key_is_valid = self.valid_key();
            if plaintext_is_valid && key_is_valid{
                ui.memory().close_popup();
                self.update_ciphertext();
            } else if !plaintext_is_valid{
                ui.memory().open_popup(popup_id_plaintext);
            } else {
                ui.memory().open_popup(popup_id_key);
            }
        }
        egui::popup::popup_below_widget(ui, popup_id_plaintext, &plaintext_edit.response, |ui| {
            ui.code("Unvalid plaintext, plaintext must be lowercase alphabetic");
        });
        egui::popup::popup_below_widget(ui, popup_id_key, &key_edit.response, |ui| {
            ui.code("Unvalid key, key must be lowercase single word");
        });
    }
}

impl EncryptTool for VigenereEnc {
    fn valid_plaintext(&self) -> bool {
        self.plaintext
            .chars()
            .all(|c| c.is_ascii_whitespace() | c.is_ascii_lowercase())
    }
    fn update_ciphertext(&mut self) -> () {
        let plaintext_whiteless: String = self
            .plaintext
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect();
        let key_vector: Vec<u32> = self
            .key
            .chars()
            .map(|c| c as u32 - 96)
            .collect::<Vec<u32>>();
        let mut ciphertext_build = String::from("");
        for c in plaintext_whiteless.char_indices() {
            let position = (c.1 as u32 + &key_vector[c.0 % key_vector.len()] - 97) % 26 + 65;
            ciphertext_build.push(char::from_u32(position).unwrap());
        }
        self.ciphertext = ciphertext_build;
    }
}

impl VigenereEnc {
    fn valid_key(&self) -> bool {
        self.key
            .chars()
            .all(|c| c.is_ascii_lowercase())
    }
}
