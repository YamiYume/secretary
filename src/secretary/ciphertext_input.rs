use crate::secretary::Status;
use dirs::home_dir;
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct CiphertextInput<Format> {
    ciphertext: String,
    formated: Format,
    response: Option<egui::Response>,
    popup: Option<egui::Id>,
    status: Status,
}

impl CiphertextInput<String> {
    fn format(&mut self) {
        self.formated = self
            .ciphertext
            .trim()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase())
            .collect();
        self.status = Status::Waiting;
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            self.response = Some(
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.ciphertext)
                        .hint_text(" Write your ciphertext here"),
                ),
            );
            ui.code(&self.formated);
        });
        self.popup_ui(ui);
    }
}

impl CiphertextInput<PathBuf> {
    fn format(&mut self) {
        self.formated = PathBuf::from(&self.ciphertext);
        self.status = Status::Waiting;
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical_centered_justified(|ui| {
                self.response = Some(
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.ciphertext)
                            .hint_text(" Write your ciphertext here"),
                    ),
                );
                ui.code(self.formated.to_string_lossy());
            });
            if ui.add(egui::Button::new(" Search")).clicked() {
                let file_path = FileDialog::new()
                    .add_filter("Image", &["jpg", "png"])
                    .set_directory(home_dir().unwrap_or(PathBuf::from("/")))
                    .pick_file();
                if let Some(path) = file_path {
                    self.ciphertext = path.to_string_lossy().to_string();
                    self.formated = path;
                    self.status = Status::Valid;
                }
            }
        });
        self.popup_ui(ui)
    }
}

impl<Format> CiphertextInput<Format> {
    fn popup_ui(&mut self, ui: &mut egui::Ui) {
        self.popup = Some(ui.make_persistent_id("ciphertext_error"));
        egui::popup_below_widget(&ui, self.popup.unwrap(), self.response.as_ref().unwrap(), |ui| {
            if let Status::Invalid(error) = self.status {
                ui.code(error);
            }
        });
        if let (Status::Invalid(_), Some(popup)) = (&self.status, self.popup) {
            ui.memory().open_popup(popup);
        } else if let Some(popup) = self.popup {
            if ui.memory().is_popup_open(popup) {
                ui.memory().toggle_popup(popup);
            }
        }
    }
}
