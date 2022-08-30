use super::{EncryptTool, Tool, View};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct AfinEnc {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: [u32; 2],
}

impl Default for AfinEnc {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: [1, 0],
        }
    }
}

impl Tool for AfinEnc {
    fn name(&self) -> &'static str {
        "ﲘ Afin"
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

impl View for AfinEnc {
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
        let key_selector_0 = ui.add(egui::Slider::new(&mut key[0], 0..=26)
            .text("A key"));
        let key_selector_1 = ui.add(egui::Slider::new(&mut key[1], 0..=26)
            .text("B key"));
        let popup_id_key= ui.make_persistent_id("Error_popup_key");
        ui.horizontal_top(|ui| {
            ui.add_enabled(
                false,
                egui::TextEdit::multiline(ciphertext).hint_text("here"),
            );
            ui.vertical_centered_justified(|ui| {
                if ui.add(egui::Button::new("Copy")).clicked() {
                    ui.output().copied_text = ciphertext.to_string();
                }
                if ui.add(egui::Button::new("Copy Key")).clicked() {
                    ui.output().copied_text =
                        format!("{} {}", key[0].to_string(), key[1].to_string());
                }
            });
        });
        if (plaintext_edit.response.changed()
            || key_selector_0.changed()
            || key_selector_1.changed())
            && !plaintext.is_empty()
        {
            let key_is_valid = self.valid_key();
            let plaintext_is_valid = self.valid_plaintext();
            if plaintext_is_valid && key_is_valid {
                ui.memory().close_popup();
                self.update_ciphertext();
            } else if key_is_valid {
                ui.memory().open_popup(popup_id_plaintext);
            } else {
                ui.memory().open_popup(popup_id_key);
            }
        }
        egui::popup::popup_below_widget(ui, popup_id_plaintext, &plaintext_edit.response, |ui| {
            ui.code("Unvalid plaintext, plaintext must be lowercase alphabetic");
        });
        egui::popup_below_widget(ui, popup_id_key, &key_selector_1, |ui| {
            ui.code("Unvalid A key, must be cooprime with 26");
        });
    }
}

impl EncryptTool for AfinEnc {
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
        let mut ciphertext_build = String::from("");
        for c in plaintext_whiteless.chars() {
            let position = ((c as u32 - 97) * self.key[0] + self.key[1]) % 26 + 65;
            ciphertext_build.push(char::from_u32(position).unwrap());
        }
        self.ciphertext = ciphertext_build;
    }
}

impl AfinEnc {
    fn valid_key(&self) -> bool {
        if self.key[0] == 0 {
            return true;
        } else {
            return !(26 % self.key[0] == 0)
        }
    }
}

