mod secretary;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "SecretAry",
        native_options,
        Box::new(|cc| Box::new(Secretary::new(cc))),
    );
}

#[derive(Default)]
struct Secretary {
    menu: secretary::SecretaryWindows,
}

impl Secretary {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "JetBrains".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/JetBrains.ttf")),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "JetBrains".to_owned());
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "JetBrains".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_pixels_per_point(1.4);
        Self::default()
    }
}

impl eframe::App for Secretary {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.menu.ui(ctx)
    }
}
