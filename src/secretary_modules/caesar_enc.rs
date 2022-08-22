use super::{Tool, View};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct CaesarEnc {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: u32,
}

impl Default for CaesarEnc {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: 0,
        }
    }
}

impl Tool for CaesarEnc {
    fn name(&self) -> &'static str {
        " Caesar"
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

impl View for CaesarEnc {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {
        let Self {
            plaintext,
            ciphertext,
            key,
        } = self;
        let plaintext_edit = egui::TextEdit::multiline(plaintext)
            .hint_text("Write your plaintext here")
            .show(ui);
        let key_selector = ui.add(egui::Slider::new(key, 0..=26));
        ui.add_enabled(false, egui::TextEdit::multiline(ciphertext));
        if plaintext_edit.response.changed() | key_selector.changed() {
            if self.valid_plaintext() {
                self.update_ciphertext()
            }
        }
    }
}

impl CaesarEnc {
    fn update_ciphertext(&mut self) -> () {
        let plaintext_whiteless: String = self.plaintext
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect();
        let mut ciphertext_build = String::from("");
        for c in plaintext_whiteless.chars() {
            let position = (c as u32 + self.key - 61) % 26 + 40;
            ciphertext_build.push(char::from_u32(position).unwrap());
        }
        self.ciphertext = ciphertext_build;
    }
    fn valid_plaintext(&self) -> bool {
        let mut is_valid: bool = true;
        is_valid &= self.plaintext.is_ascii();
        is_valid &= self.plaintext
            .chars()
            .all(|c| c.is_alphabetic());
        is_valid
    }
}
