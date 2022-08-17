use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Secretary",
        native_options,
        Box::new(|_cc| Box::new(Secretary::new(_cc))),
    );
}

struct Secretary {
    plaintext: String,
    ciphertext: String,
    key: u32
}

impl Secretary {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for Secretary {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            ciphertext: String::from(""),
            key: 0
        }
    }
}

impl eframe::App for Secretary {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> () {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Secretary");
            ui.vertical(|ui| {
                ui.label("Clear Text");
                ui.text_edit_multiline(&mut self.plaintext);
                ui.add(egui::Slider::new(&mut self.key, 0..=25).text("key"));
                if ui.button("Cipher").clicked() {
                    self.ciphertext = displacement_cipher(&self.plaintext, self.key);
                }
                ui.label(format!("ciphertext: {}", self.ciphertext));
            });
        });
    }
}

fn displacement_cipher(plaintext: &str, key: u32) -> String {
    let mut ciphertext = String::from("");
    for c in plaintext.chars() {
        if !c.is_whitespace() {
            let cipherchar = (c as u32 - 97 + key) % 26 + 65;
            ciphertext.push(char::from_u32(cipherchar).unwrap());
        }
    }
    ciphertext
}

fn displacement_decipher(ciphertext: &str, key: u32) -> String {
    let mut plaintext = String::from("");
    for c in ciphertext.chars() {
        let plainchar = (c as u32 - 65 - key)
    }
}
