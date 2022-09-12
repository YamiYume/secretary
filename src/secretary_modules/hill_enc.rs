use super::{Tool, View, plaintext_input};
use image;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct HillEnc {
    pub plaintext: String,
    pub plaintext_texture: Option<egui::TextureHandle>,
    pub ciphertext: String,
    pub ciphertext_texture: Option<egui::TextureHandle>,
    pub key: String,
}

impl Default for HillEnc {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            plaintext_texture: Option::None,
            ciphertext: String::from(""),
            ciphertext_texture: Option::None,
            key: String::from(""),
        }
    }
}

impl Tool for HillEnc {
    fn name(&self) -> &'static str {
        "ﲘ Hill"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> () {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl View for HillEnc {
    fn ui(&mut self, ui: &mut egui::Ui) -> () {

        let (plaintext_edit, plain_error) = plaintext_input(&mut self.plaintext, ui);
        if ui.add(egui::Button::new("load")).clicked() {
            let image_load = load_image_from_path(
                std::path::Path::new(&self.plaintext)
            );
            match image_load {
                Ok(image) => self.plaintext_texture = Some(
                    ui.ctx().load_texture(
                        "plaintext_image",
                        image,
                        egui::TextureFilter::Linear
                    )
                ),
                Err(_) => {
                    ui.memory().open_popup(plain_error);
                    self.plaintext_texture = Option::None;
                },
            }  
        }
        if let Some(texture) = &self.plaintext_texture {
            ui.image(texture, texture.size_vec2());
        }
        let (key_edit, key_error) = super::key_input(&mut self.key, ui);
        if key_edit.changed() && !self.key.is_empty() && self.plaintext_texture.is_some(){
            if self.valid_key() {
                ui.memory().close_popup();
            } else {
                ui.memory().open_popup(key_error);
            }
        }
        egui::popup_below_widget(ui, plain_error, &plaintext_edit, |ui| {
            ui.code("invalid image")
        });
        egui::popup_below_widget(ui, key_error, &key_edit, |ui| {
            ui.code("key must be 9 or 16 integers separated with spaces")
        });

    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError>  {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

impl HillEnc {
    fn valid_key(&self) -> bool {
        let mut answer = !self.key.is_empty()
        && self.key
            .chars()
            .all(|c| c.is_ascii_whitespace() || c.is_ascii_digit());
        let splited_transformed: Vec<u32> = self.key
            .split_whitespace()
            .map(|s| u32::from_str_radix(s, 10).unwrap())
            .collect();
        answer &= splited_transformed.len() == 9 || splited_transformed.len() == 16;
        answer &= splited_transformed.iter().all(|x| &0 <= x && x <= &255);
        answer  
    }
}
