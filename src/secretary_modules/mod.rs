pub mod secretary_app_windows;
pub mod caesar_enc;

pub use {
    secretary_app_windows::SecretaryWindows,
};

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui) -> ();
}

pub trait Tool {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> ();
}