use super::{ciphertext_input, mod_inv, plaintext_output, valid_ciphertext, Tool, View};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct AfinDec {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: Vec<i32>,
}

impl Default for AfinDec {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: vec![1, 1],
        }
    }
}

impl Tool for AfinDec {
    fn name(&self) -> &'static str {
        "ﲙ Afin"
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

impl View for AfinDec {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {
        let (ciphertext_edit, cipher_error) = ciphertext_input(&mut self.ciphertext, ui);
        let key_selector_a = ui.add(egui::Slider::new(&mut self.key[0], 1..=25).text("A key"));
        let key_selector_b = ui.add(egui::Slider::new(&mut self.key[1], 1..=25).text("B key"));
        let key_error = ui.make_persistent_id("key_error");

        plaintext_output(&mut self.plaintext, &self.key, ui);

        if (ciphertext_edit.changed() || key_selector_a.changed() || key_selector_b.changed())
            && !self.ciphertext.is_empty()
        {
            let key_is_valid = self.valid_key();
            let ciphertext_is_valid = valid_ciphertext(&self.ciphertext);
            if ciphertext_is_valid && key_is_valid {
                ui.memory().close_popup();
                self.update_plaintext();
            } else if key_is_valid {
                ui.memory().open_popup(cipher_error);
            } else {
                ui.memory().open_popup(key_error);
            }
        }
        egui::popup_below_widget(ui, cipher_error, &ciphertext_edit, |ui| {
            ui.code("ciphertext must be uppercase single word")
        });
        egui::popup_below_widget(ui, key_error, &key_selector_a, |ui| {
            ui.code("key A must be cooprime with 26")
        });
    }
}

impl AfinDec {
    fn update_plaintext(&mut self) -> () {
        let mut new_plaintext = String::from("");
        for c in self.ciphertext.chars() {
            new_plaintext.push(AfinDec::char_decipher(c, &self.key));
        }
        self.plaintext = new_plaintext;
    }

    fn char_decipher(c: char, key: &Vec<i32>) -> char {
        char::from_u32(
            (((c as i32 - key[1] - 65) * mod_inv(key[0], 26)).rem_euclid(26) + 97) as u32,
        )
        .unwrap()
    }

    fn valid_key(&self) -> bool {
        if self.key[0] == 0 {
            return true;
        } else {
            return self.key[0] % 13 != 0 && self.key[0] % 2 != 0;
        }
    }
}
