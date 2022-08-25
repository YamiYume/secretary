use super::{DecryptTool, Tool, View};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct CaesarDec {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: i32,
}

impl Default for CaesarDec {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: 0,
        }
    }
}

impl Tool for CaesarDec {
    fn name(&self) -> &'static str {
        "ﲙ Caesar"
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

impl View for CaesarDec {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {
        let Self {
            plaintext,
            ciphertext,
            key,
        } = self;
        let ciphertext_edit = egui::TextEdit::multiline(ciphertext)
            .hint_text("Write your ciphertext here")
            .show(ui);
        let popup_id = ui.make_persistent_id("Error_popup");
        let key_selector = ui.add(egui::Slider::new(key, 0..=26));
        ui.horizontal_top(|ui| {
            ui.add_enabled(
                false,
                egui::TextEdit::multiline(plaintext).hint_text("Here will appear your plaintext")
            );
            ui.vertical_centered_justified(|ui| {
                if ui.add(egui::Button::new("Copy")).clicked() {
                    ui.output().copied_text = plaintext.to_string();
                }
                if ui.add(egui::Button::new("Copy Key")).clicked() {
                    ui.output().copied_text = key.to_string();
                }
            });
        });
        if ciphertext_edit.response.changed() | key_selector.changed() {
            if self.valid_ciphertext() {
                ui.memory().close_popup();
                self.update_plaintext();
            } else {
                ui.memory().open_popup(popup_id);
            }
        }
        egui::popup::popup_below_widget(ui, popup_id, &ciphertext_edit.response, |ui| {
            ui.code("Unvalid ciphertext, must be uppercase alphabetic single word");
        });
    }
}

impl DecryptTool for CaesarDec {
    fn valid_ciphertext(&self) -> bool {
        self.ciphertext.chars().all(|c| c.is_ascii_uppercase())
    }
    fn update_plaintext(&mut self) -> () {
        let mut plaintext_build = String::from("");
        for c in self.ciphertext.chars() {
            let position: i32 = (c as i32 - self.key - 65).rem_euclid(26) + 97;
            plaintext_build.push(char::from_u32(position as u32).unwrap());
        }
        self.plaintext = plaintext_build;
    }
}
