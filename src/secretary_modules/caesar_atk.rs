use super::{Tool, View, AttackTool};
use egui_extras::{TableBuilder, Size};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct CaesarAtk {
    pub ciphertext: String,
}

impl Default for CaesarAtk {
    fn default() -> Self {
        Self {
            ciphertext: String::from("")
        }
    }
}

impl Tool for CaesarAtk {
    fn name(&self) -> &'static str {
        "理 Caesar"
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

impl View for CaesarAtk {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {
        let ciphertext_edit = ui.add(
            egui::TextEdit::multiline(&mut self.ciphertext)
                .hint_text("Write your ciphertext here")
        );
        let popup_id = ui.make_persistent_id("ciphertext_error");
        if self.valid_ciphertext() {
            if !self.ciphertext.is_empty() {
                ui.memory().close_popup();
                let decrypted = self.cryptoanalisys();
                TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Min))
                    .column(Size::relative(0.8))
                    .column(Size::relative(0.1))
                    .column(Size::relative(0.1))
                    .resizable(true)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("plaintext");
                        });
                        header.col(|ui| {
                            ui.heading("key");
                        });
                        header.col(|ui| {
                            ui.heading("");
                        });
                    })
                    .body(|mut body| {
                        for (i, [plaintext, key]) in decrypted.iter().enumerate() {
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(plaintext);
                                });
                                row.col(|ui| {
                                    ui.label(key);
                                });
                                row.col(|ui| {
                                   if ui.add(egui::Button::new("copy")).clicked() {
                                        ui.output().copied_text = format!(
                                            "{} {}",
                                            decrypted[i][0],
                                            decrypted[i][1]
                                        );
                                    }
                                });
                            });
                        }
                    });
            }
        } else {
            ui.memory().open_popup(popup_id);
        }
        egui::popup_below_widget(ui, popup_id, &ciphertext_edit, |ui| {
            ui.code("Unvalid ciphertext, must be single word uppercase")
        });
    }
}

impl AttackTool for CaesarAtk {
    fn ciphertext(&self) -> &str {
        &self.ciphertext
    }
}

impl CaesarAtk {
    fn cryptoanalisys(&self) -> Vec<[String; 2]> {
        let mut posibilities = Vec::new();
        for key in 0..=26 {
            let plaintext = decipher(self.ciphertext(), key);
            posibilities.push([plaintext, key.to_string()])
        }
        posibilities
    }
}

fn decipher(text: &str, key: i32) -> String {
    let mut plaintext = String::from("");
    for c in text.chars() {
        let position: i32 = (c as i32 - key - 65).rem_euclid(26) + 97;
        plaintext.push(char::from_u32(position as u32).unwrap());
    }
    plaintext
}
