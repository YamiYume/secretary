use super::{Tool, View, EncryptTool};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

struct VigenereEnc {
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
        " Vigenere"
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
        let popup_id = ui.make_persistent_id("Error_popup");
        let key_edit = egui::TextEdit::singleline(key)
            .hint_text("Write your key here")
            .show(ui);
        ui.horizontal(|ui| {
            ui.add_enabled(false, egui::TextEdit::multiline(ciphertext));
            ui.vertical_centered_justified(|ui| {
                if ui.add(egui::Button::new("Copy")).clicked() {
                    ui.output().copied_text = ciphertext.to_string();
                }
                if ui.add(egui::Button::new("Copy Key")).clicked() {
                    ui.output().copied_text = key.to_string();
                }
            });
        });
        if plaintext_edit.response.changed() | key_edit.response.changed() {
            if self.valid_plaintest() {
                ui.memory().close_popup();
                self.update_ciphertext();
            } else {
                ui.memory().open_popup(popup_id);
            }
        }
    }
}

