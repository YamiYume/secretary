use super::{Tool, View};
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
        let (plaintext_edit, plain_error) = super::plaintext_input(&mut self.plaintext, ui);
        let (key_edit, key_error) = super::key_input(&mut self.key, ui);

        super::ciphertext_output(&mut self.ciphertext, vec![self.key], ui);

        if (plaintext_edit.changed() || key_edit.changed()) 
        && (!self.key.is_empty() && !self.plaintext.is_empty()) {

            let plaintext_is_valid = super::valid_plaintext(&self.plaintext);
            let key_is_valid = self.valid_key();

            if plaintext_is_valid && key_is_valid{
                ui.memory().close_popup();
                self.update_ciphertext();
            } else if !plaintext_is_valid{
                ui.memory().open_popup(plain_error);
            } else {
                ui.memory().open_popup(key_error);
            }
        }
        super::error_popup(plain_error, &plaintext_edit, ui, "plaintext must be lowecase only");
        super::error_popup(key_error, &key_edit, ui, "key must be single word lowecase");
    }
}

impl VigenereEnc {
    fn update_ciphertext(&mut self) -> () {
        let key_vector: Vec<u32> = self
            .key
            .chars()
            .map(|c| c as u8 - 96)
            .collect();
        let mut new_ciphertext = String::from("");
        for (i, c) in super::whiteless(&self.plaintext).char_indices() {
            new_ciphertext.push(
                self.char_cipher(c, key_vector[i % key_vector.len()])
            );
        }
        self.ciphertext = new_ciphertext;
    }

    fn char_cipher(c: char, key: u8) -> char {
        char::from_u32((c as u32 + key - 97) % 26 + 65).unwrap()
    }

    fn valid_key(&self) -> bool {
        self.key
            .chars()
            .all(|c| c.is_ascii_lowercase())
    }
}
