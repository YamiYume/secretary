use super::{DecryptTool, Tool, View};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct AfinDec {
    pub plaintext: String,
    pub ciphertext: String,
    pub key: [i32; 2],
}

impl Default for AfinDec {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: [1, 0],
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
        let Self {
            plaintext,
            ciphertext,
            key,
        } = self;
        let ciphertext_edit = egui::TextEdit::multiline(ciphertext)
            .hint_text("Write your ciphertext here")
            .show(ui);
        let popup_id_ciphertext = ui.make_persistent_id("Error_popup_ciphertext");
        let key_selector_0 = ui.add(egui::Slider::new(&mut key[0], 0..=26)
            .text("A key"));
        let key_selector_1 = ui.add(egui::Slider::new(&mut key[1], 0..=26)
            .text("B key"));
        let popup_id_key= ui.make_persistent_id("Error_popup_key");
        ui.horizontal_top(|ui| {
            ui.add_enabled(
                false,
                egui::TextEdit::multiline(plaintext)
                    .hint_text("Here will appear your plaintext")
            );
            ui.vertical_centered_justified(|ui| {
                if ui.add(egui::Button::new("Copy")).clicked() {
                    ui.output().copied_text = plaintext.to_string();
                }
                if ui.add(egui::Button::new("Copy Key")).clicked() {
                    ui.output().copied_text =
                        format!("{} {}", key[0].to_string(), key[1].to_string());
                }
            });
        });
        if (ciphertext_edit.response.changed()
            || key_selector_0.changed()
            || key_selector_1.changed())
            && !ciphertext.is_empty()
        {
            let key_is_valid = self.valid_key();
            let ciphertext_is_valid = self.valid_ciphertext();
            if ciphertext_is_valid && key_is_valid {
                ui.memory().close_popup();
                self.update_plaintext();
            } else if key_is_valid {
                ui.memory().open_popup(popup_id_ciphertext);
            } else {
                ui.memory().open_popup(popup_id_key);
            }
        }
        egui::popup::popup_below_widget(ui, popup_id_ciphertext, &ciphertext_edit.response, |ui| {
            ui.code("Unvalid ciphertext, ciphertext must be uppercase alphabetic single word");
        });
        egui::popup_below_widget(ui, popup_id_key, &key_selector_1, |ui| {
            ui.code("Unvalid A key, must be cooprime with 26");
        });
    }
}

impl DecryptTool for AfinDec {
    fn valid_ciphertext(&self) -> bool {
        self.ciphertext
            .chars()
            .all(|c| c.is_ascii_uppercase())
    }
    fn update_plaintext(&mut self) -> () {
        let mut plaintext_build = String::from("");
        for c in self.ciphertext.chars() {
            let position = ((c as i32 - self.key[1] - 65) * AfinDec::mod_inv(self.key[0], 26)).rem_euclid(26) + 97;
            plaintext_build.push(char::from_u32(position as u32).unwrap());
        }
        self.plaintext = plaintext_build;
    }
}

impl AfinDec {
    fn valid_key(&self) -> bool {
        if self.key[0] == 0 {
            return true;
        } else {
            return self.key[0] % 13 != 0 && self.key[0] % 2 != 0
        }
    }
    fn mod_inv(a: i32, module: i32) -> i32 {
        let mut mn = (module, a);
        let mut xy = (0, 1);
      
        while mn.1 != 0 {
            xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
            mn = (mn.1, mn.0 % mn.1);
        }
      
        while xy.0 < 0 {
            xy.0 += module;
        }
        xy.0
    }
}

