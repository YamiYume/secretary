use crate::secretary::Status;
use dirs::home_dir;
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct PlaintextInput<Format> {
    plaintext: String,
    pub formated: Format,
    pub response: Option<egui::Response>,
    popup: Option<egui::Id>,
    pub status: Status,
}

impl PlaintextInput<String> {
    pub fn format(&mut self) {
        if self.plaintext.is_empty() {
            self.formated = String::default();
            self.status = Status::Idle;
            return;
        }
        self.formated = self
            .plaintext
            .trim()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        self.status = Status::Waiting;
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            self.response = Some(
                ui.add(
                    egui::TextEdit::multiline(&mut self.plaintext)
                        .hint_text(" Write your plaintext here"),
                ),
            );
            if self.formated.is_empty() {
                ui.code(" ");
            } else { 
            ui.code(&self.formated);
            }
        });
        self.popup_ui(ui);
    }
}

impl PlaintextInput<PathBuf> {
    fn format(&mut self) {
        self.formated = PathBuf::from(&self.plaintext);
        self.status = Status::Waiting;
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.horizontal(|ui| {
                self.response = Some(
                    ui.add(
                        egui::TextEdit::multiline(&mut self.plaintext)
                            .hint_text(" Write your plaintext here"),
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
                    self.plaintext = path.to_string_lossy().to_string();
                    self.formated = path;
                    self.status = Status::Valid;
                }
            }
        });
        self.popup_ui(ui)
    }
}

impl<Format> PlaintextInput<Format> {
    fn popup_ui(&mut self, ui: &mut egui::Ui) {
        self.popup = Some(ui.make_persistent_id("plaintext_error"));
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
