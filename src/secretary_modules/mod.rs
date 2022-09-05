pub mod secretary_app_windows;
pub mod caesar_enc;
pub mod vigenere_enc;
pub mod caesar_dec;
pub mod vigenere_dec;
pub mod afin_enc;
pub mod afin_dec;

pub use secretary_app_windows::SecretaryWindows;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui) -> ();
}

pub trait Tool {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> ();
}

pub trait EncryptTool {
    fn valid_plaintext(&self) -> bool;
    fn update_ciphertext(&mut self) -> ();
}

pub trait DecryptTool {
    fn valid_ciphertext(&self) -> bool;
    fn update_plaintext(&mut self) -> ();
}
