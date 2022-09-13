pub mod secretary_app_windows;

//Using code from https://github.com/FredrikAugust/rusty-crypto with GNU GENERAL PUBLIC LICENSE
pub mod matrix;
pub mod euclidean_algorithm;

pub mod caesar_enc;
pub mod caesar_dec;
pub mod caesar_atk;
pub mod afin_enc;
pub mod afin_dec;
pub mod afin_atk;
pub mod vigenere_enc;
pub mod vigenere_dec;
pub mod perm_enc;
pub mod perm_dec;
pub mod hill_enc;
pub mod hill_dec;

pub use secretary_app_windows::SecretaryWindows;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui) -> ();
}

pub trait Tool {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> ();
}

pub fn plaintext_input(plaintext: &mut String, ui: &mut egui::Ui) 
    -> (egui::Response, egui::Id) {
    let plaintext_edit = ui.add(
            egui::TextEdit::multiline(plaintext)
                .hint_text("Write your plaintext here")
    );
    let plain_popup_id = ui.make_persistent_id("plain_error");
    (plaintext_edit, plain_popup_id)
}

pub fn ciphertext_input(ciphertext: &mut String, ui: &mut egui::Ui) 
    -> (egui::Response, egui::Id) {
    let ciphertext_edit = ui.add(
            egui::TextEdit::multiline(ciphertext)
                .hint_text("Write your ciphertext here")
    );
    let cipher_popup_id = ui.make_persistent_id("cipher_error");
    (ciphertext_edit, cipher_popup_id)
}

pub fn key_input(key: &mut String, ui: &mut egui::Ui) 
    -> (egui::Response, egui::Id) {
    let key_edit = ui.add(
            egui::TextEdit::multiline(key)
                .hint_text("Write your key here")
    );
    let key_popup_id = ui.make_persistent_id("key_error");
    (key_edit, key_popup_id)
}

pub fn plaintext_output<T: ToString>(
    plaintext: &mut String, keys: &Vec<T>,
    ui: &mut egui::Ui)
    -> () {
    ui.horizontal_top(|ui| {
        ui.add(
            egui::TextEdit::multiline(plaintext)
                .hint_text("Here will appear decrypted plaintext")
                .interactive(false)
        );
        ui.vertical_centered_justified(|ui| {
            if ui.add(egui::Button::new("Copy")).clicked() {
                ui.output().copied_text = plaintext.to_string();
            }
            if ui.add(egui::Button::new("Copy key")).clicked() {
                ui.output().copied_text = keys
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
            }
        });
    });
}

pub fn ciphertext_output<T: ToString>(
    ciphertext: &mut String, keys: &Vec<T>,
    ui: &mut egui::Ui)
    -> () {
    ui.horizontal_top(|ui| {
        ui.add(
            egui::TextEdit::multiline(ciphertext)
                .hint_text("Here will appear encrypted ciphertext")
                .interactive(false)
        );
        ui.vertical_centered_justified(|ui| {
            if ui.add(egui::Button::new("Copy")).clicked() {
                ui.output().copied_text = ciphertext.to_string();
            }
            if ui.add(egui::Button::new("Copy key")).clicked() {
                ui.output().copied_text = keys
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
            }
        });
    });
}

pub fn error_popup(
    id: egui::Id, response: &egui::Response,
    ui: &mut egui::Ui, text: &str)
    -> () {
    egui::popup_below_widget(ui, id, response, |ui| {
        ui.code(text)
    });
}

pub fn valid_plaintext(plaintext: &String) -> bool {
    plaintext
        .chars()
        .all(|c| c.is_ascii_lowercase() | c.is_ascii_whitespace())
}

pub fn valid_ciphertext(ciphertext: &String) -> bool {
    ciphertext
        .chars()
        .all(|c| c.is_ascii_uppercase())
}

pub fn whiteless(text :&String) -> String {
    text
    .chars()
    .filter(|c| !c.is_ascii_whitespace())
    .collect()
}

pub fn mod_inv(a: i32, module: i32) -> i32 {
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

