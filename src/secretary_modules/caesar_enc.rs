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
        "ﲘ Caesar"
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

        let (plaintext_edit, plain_error) = super::plaintext_input(&mut self.plaintext, ui);
        let key_selector = ui.add(egui::Slider::new(&mut self.key, 1..=25));

        super::ciphertext_output(&mut self.ciphertext, &vec![self.key], ui);

        if (plaintext_edit.changed() || key_selector.changed()) && !self.plaintext.is_empty() {
            if super::valid_plaintext(&self.plaintext) {
                ui.memory().close_popup();
                self.update_ciphertext();
            } else {
                ui.memory().open_popup(plain_error);
            }
        }
        egui::popup_below_widget(ui, plain_error, &plaintext_edit, |ui| {
            ui.code("plaintext must be lowercase only")
        });
    }
}

impl CaesarEnc {
    fn update_ciphertext(&mut self) -> () {
        let mut new_ciphertext = String::from("");
        for c in super::whiteless(&self.plaintext).chars() {
            new_ciphertext.push(CaesarEnc::char_cipher(c, self.key));
        }
        self.ciphertext = new_ciphertext;
    }

    fn char_cipher(c: char, key: u32) -> char {
        char::from_u32((c as u32 + key - 97) % 26 + 65).unwrap()
    }
}
