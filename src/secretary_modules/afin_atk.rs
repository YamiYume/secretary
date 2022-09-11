use super::{Tool, View};
use egui_extras::{TableBuilder, Size};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct AfinAtk {
    pub ciphertext: String,
}

impl Default for AfinAtk {
    fn default() -> Self {
        Self {
            ciphertext: String::from("")
        }
    }
}

impl Tool for AfinAtk {
    fn name(&self) -> &'static str {
        "理 Afin"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> () {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl View for AfinAtk {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {
        let (ciphertext_edit, cipher_error) = super::ciphertext_input(&mut self.ciphertext, ui);
        if super::valid_ciphertext(&self.ciphertext) {
            if !self.ciphertext.is_empty() {
                ui.memory().close_popup();
                let decrypted = self.cryptoanalisys();
                TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Min))
                    .column(Size::relative(0.5))
                    .column(Size::relative(0.2))
                    .column(Size::relative(0.2))
                    .column(Size::relative(0.1))
                    .resizable(true)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("plaintext");
                        });
                        header.col(|ui| {
                            ui.heading("keyA");
                        });
                        header.col(|ui| {
                            ui.heading("keyB");
                        });
                        header.col(|ui| {
                            ui.heading("");
                        });
                    })
                    .body(|mut body| {
                        for (i, [plaintext, key_1, key_2]) in decrypted.iter().enumerate() {
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(plaintext);
                                });
                                row.col(|ui| {
                                    ui.label(key_1);
                                });
                                row.col(|ui| {
                                    ui.label(key_2);
                                });
                                row.col(|ui| {
                                   if ui.add(egui::Button::new("copy")).clicked() {
                                        ui.output().copied_text = format!(
                                            "{} {} {}",
                                            decrypted[i][0],
                                            decrypted[i][1],
                                            decrypted[i][2]
                                        );
                                    }
                                });
                            });
                        }
                    });
            }
        } else {
            ui.memory().open_popup(cipher_error);
        }
        egui::popup_below_widget(ui, cipher_error, &ciphertext_edit, |ui| {
            ui.code("Unvalid ciphertext, must be single word uppercase")
        });
    }
}

impl AfinAtk {
    fn cryptoanalisys(&self) -> Vec<[String; 3]> {
        let mut posibilities = Vec::new();
        for key_1 in (0..=26).filter(|x| (x % 13 != 0) && (x % 2 != 0)) {
            for key_2 in 0..=26 {
                let plaintext = decipher(&self.ciphertext, key_1, key_2);
                posibilities.push([plaintext, key_1.to_string(), key_2.to_string()])
            }
        }
        posibilities
    }
}

fn decipher(text: &str, key_1: i32, key_2: i32) -> String {
    let mut plaintext = String::from("");
    for c in text.chars() {
        let position = ((c as i32 - key_2 - 65) * super::mod_inv(key_1, 26)).rem_euclid(26) + 97;
        plaintext.push(char::from_u32(position as u32).unwrap());
    }
    plaintext
}
