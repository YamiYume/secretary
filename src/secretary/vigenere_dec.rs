use super::{Tool, View};
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
        let (ciphertext_edit, cipher_error) = super::ciphertext_input(&mut self.ciphertext, ui);
        let (key_edit, key_error) = super::key_input(&mut self.key, ui);

        super::plaintext_output(&mut self.plaintext, &vec![&self.key], ui);

        if (ciphertext_edit.changed() || key_edit.changed())
            && (!self.key.is_empty() && !self.ciphertext.is_empty())
        {
            let ciphertext_is_valid = super::valid_ciphertext(&self.ciphertext);
            let key_is_valid = self.valid_key();

            if ciphertext_is_valid && key_is_valid {
                ui.memory().close_popup();
                self.update_plaintext();
            } else if !ciphertext_is_valid {
                ui.memory().open_popup(cipher_error);
            } else {
                ui.memory().open_popup(key_error);
            }
        }
        super::error_popup(
            cipher_error,
            &ciphertext_edit,
            ui,
            "ciphertext  must be single word uppercase",
        );
        super::error_popup(key_error, &key_edit, ui, "key must be single word lowecase");
    }
}

impl VigenereDec {
    fn update_plaintext(&mut self) -> () {
        let key_vector: Vec<i32> = self.key.chars().map(|c| c as i32 - 96).collect();
        let mut new_plaintext = String::from("");
        for (i, c) in self.ciphertext.char_indices() {
            new_plaintext.push(VigenereDec::char_decipher(
                c,
                key_vector[i % key_vector.len()],
            ));
        }
        self.plaintext = new_plaintext;
    }

    fn char_decipher(c: char, key: i32) -> char {
        char::from_u32(
            ((c as i32 - key - 65).rem_euclid(26) + 97)
                .try_into()
                .unwrap(),
        )
        .unwrap()
    }

    fn valid_key(&self) -> bool {
        self.key.chars().all(|c| c.is_ascii_lowercase())
    }
}
