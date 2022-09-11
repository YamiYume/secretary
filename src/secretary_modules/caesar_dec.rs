use super::{Tool, View};
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
        let (ciphertext_edit, cipher_error) = super::ciphertext_input(&mut self.ciphertext, ui);
        let key_selector = ui.add(egui::Slider::new(&mut self.key, 1..=25));

        super::plaintext_output(&mut self.plaintext, &vec![self.key], ui);

        if (ciphertext_edit.changed() || key_selector.changed()) && !self.ciphertext.is_empty() {
            if super::valid_ciphertext(&self.ciphertext) {
                ui.memory().close_popup();
                self.update_plaintext();
            } else {
                ui.memory().open_popup(cipher_error);
            }
        }
        egui::popup_below_widget(ui, cipher_error, &ciphertext_edit, |ui| {
            ui.code("ciphertext must be single word uppercase")
        });
    }
}

impl CaesarDec {
    fn update_plaintext(&mut self) -> () {
        let mut new_plaintext = String::from("");
        for c in self.ciphertext.chars() {
            new_plaintext.push(CaesarDec::char_decipher(c, self.key));
        }
        self.plaintext = new_plaintext;
    }
    fn char_decipher(c: char, key: i32) -> char {
        char::from_u32(
            ((c as i32 - key - 65).rem_euclid(26) + 97) as u32
        ).unwrap()
    }
}
