use crate::secretary::Status;

#[derive(Debug, Default)]
pub struct KeyInput<Format> {
    key: String,
    pub formated: Vec<Format>,
    pub response: Option<egui::Response>,
    popup: Option<egui::Id>,
    pub status: Status,
}

impl KeyInput<u32> {
    pub fn format(&mut self) {
        if self.key.is_empty() {
            self.formated = vec![];
            self.status = Status::Idle;
            return;
        }
        let filtered_key: String = self
            .key
            .trim()
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
            .collect();
        self.formated = filtered_key
            .split_whitespace()
            .map(|s| u32::from_str_radix(s, 10).unwrap())
            .collect();
        self.status = Status::Waiting;
    }
}

impl KeyInput<char> {
    pub fn format(&mut self) {
        if self.key.is_empty() {
            self.formated = vec![];
            self.status = Status::Idle;
            return;
        }
        self.formated = self
            .key
            .trim()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect();
        self.status = Status::Waiting;
    }
}

impl<Format: std::fmt::Debug> KeyInput<Format> {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.vertical_centered_justified(|ui| { 
            ui.horizontal(|ui| {
                self.response = Some(ui.add(
                    egui::TextEdit::singleline(&mut self.key).hint_text(" Write Your key here"),
                ));
                if ui.add(egui::Button::new(" Copy")).clicked() {
                    ui.output().copied_text = String::from(self.key.as_str());
                }
            });
        ui.code(format!("{:?}", &self.formated));
        });
        self.popup_ui(ui)
    }

    fn popup_ui(&mut self, ui: &mut egui::Ui) {
        self.popup = Some(ui.make_persistent_id("key_error"));
        egui::popup_below_widget(&ui, self.popup.unwrap(), self.response.as_ref().unwrap(), |ui| {
            if let Status::Invalid(error) = self.status {
                ui.code(error);
            }
        });
        if let (Status::Invalid(_), Some(popup)) = (&self.status, self.popup) {
            ui.memory().open_popup(popup);
        } else if let Some(popup) = self.popup {
            if ui.memory().is_popup_open(popup) {
                ui.memory().toggle_popup(self.popup.unwrap());
            }
        }
    }
}
