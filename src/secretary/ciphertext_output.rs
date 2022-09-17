use rfd::FileDialog;

#[derive(Debug, Default)]
pub struct CiphertextOutput<Format, Representation> {
    pub ciphertext: Format,
    representation: Representation
}

impl CiphertextOutput<String, String> {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.representation)
                        .hint_text(" Your ciphertext will appear here")
                        .interactive(false),
                );
                if ui.add(egui::Button::new(" Copy")).clicked() {
                    ui.output().copied_text = self.representation
                        .as_str().to_string();
                }
            });
        });
    }
    pub fn represent(&mut self) {
        self.representation = self.ciphertext
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .collect();
    }
}

