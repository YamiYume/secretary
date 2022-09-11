use super::{Tool, View};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct AfinEnc {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: Vec<u32>,
}

impl Default for AfinEnc {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: vec![1, 1],
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
        let (plaintext_edit, plain_error) = super::plaintext_input(&mut self.plaintext, ui);
        let key_selector_0 = ui.add(egui::Slider::new(&mut self.key[0], 1..=25)
            .text("A key"));
        let key_selector_1 = ui.add(egui::Slider::new(&mut self.key[1], 1..=25)
            .text("B key"));
        let key_error= ui.make_persistent_id("key_error");
        
        super::ciphertext_output(&mut self.ciphertext, &self.key, ui);
                
        if (plaintext_edit.changed() || key_selector_0.changed() || key_selector_1.changed()) 
            && !self.plaintext.is_empty() {
            let key_is_valid = self.valid_key();
            let plaintext_is_valid = super::valid_plaintext(&self.plaintext);

            if plaintext_is_valid && key_is_valid {
                ui.memory().close_popup();
                self.update_ciphertext();
            } else if key_is_valid {
                ui.memory().open_popup(plain_error);
            } else {
                ui.memory().open_popup(key_error);
            }
        }
        egui::popup_below_widget(ui, plain_error, &plaintext_edit, |ui| {
            ui.code("plaintext must be lowecase only");
        });
        egui::popup_below_widget(ui, key_error, &key_selector_0, |ui| {
            ui.code("Key A must be cooprime with 26");
        });
    }
}

impl AfinEnc {
    fn update_ciphertext(&mut self) -> () {
        let mut new_ciphertext = String::from("");
        for c in super::whiteless(&self.plaintext).chars() {
            new_ciphertext.push(AfinEnc::char_cipher(c, &self.key));
        }
        self.ciphertext = new_ciphertext;
    }

    fn char_cipher(c: char, key: &Vec<u32>) -> char {
        char::from_u32(
            ((c as u32 - 97) * key[0] + key[1]) % 26 + 65
        ).unwrap()
    }

    fn valid_key(&self) -> bool {
        return self.key[0] % 13 != 0 && self.key[0] % 2 != 0
    }
}

