pub mod secretary_app_windows;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Tool {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
