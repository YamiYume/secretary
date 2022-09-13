use super::{Tool, View, plaintext_input, matrix};
use image;
use image::{GenericImageView, Pixel, ImageBuffer};
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct HillDec {
    pub plaintext: String,
    pub plaintext_texture: Option<egui::TextureHandle>,
    pub ciphertext_texture: Option<egui::TextureHandle>,
    pub ciphertext_buffer: Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    pub key: String,
}

impl Default for HillDec {
    fn default() -> Self {
        Self {
            plaintext: String::from(""),
            plaintext_texture: Option::None,
            ciphertext_texture: Option::None,
            ciphertext_buffer: Option::None,
            key: String::from(""),
        }
    }
}

impl Tool for HillDec {
    fn name(&self) -> &'static str {
        "ﲘ Hill d"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> () {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl View for HillDec {
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
        if key_edit.changed() | plaintext_edit.changed() {
            self.ciphertext_texture = None;
        }
        if !self.key.is_empty() && self.plaintext_texture.is_some() && self.ciphertext_texture.is_none(){
            if self.valid_key() {
                ui.memory().close_popup();
                let vec_key: Vec<i32> = self.key
                    .split_whitespace()
                    .map(|s| i32::from_str_radix(s, 10).unwrap())
                    .collect();
                let size: usize;
                match vec_key.len() {
                    9 => size = 3,
                    16 => size = 4,
                    _ => size = 3
                }
                let mut key_matrix: Vec<Vec<i32>> = vec![vec![0; size]; size];
                for i in 0..size {
                    for j in 0..size {
                        key_matrix[i][j] = vec_key[i * size + j];
                    }
                }
                let image_plain = image::io::Reader::open(std::path::Path::new(&self.plaintext))
                    .unwrap().decode().unwrap();
                let new_image_buffer = HillDec::hill_cipher(image_plain, &key_matrix);
                self.ciphertext_texture = Some(
                    ui.ctx().load_texture(
                        "ciphertext_image",
                        egui::ColorImage::from_rgba_unmultiplied(
                            [new_image_buffer.dimensions().0 as usize, new_image_buffer.dimensions().1 as usize],
                            new_image_buffer.as_flat_samples().as_slice()
                        ),
                        egui::TextureFilter::Linear
                    )
                );
                self.ciphertext_buffer = Some(new_image_buffer);
            } else {
                ui.memory().open_popup(key_error);
            }
        }
        if let Some(texture) = &self.ciphertext_texture {
            ui.image(texture, texture.size_vec2());
        }
        if ui.add(egui::Button::new("Save")).clicked() { 
            if let Some(buffer) = &self.ciphertext_buffer {
                let end = self.plaintext.split("/").last().unwrap().rsplit(".").last().unwrap();
                buffer.save(
                    std::path::Path::new(
                        &self.plaintext.replace(end, &format!("{}{}", end, "cipher"))
                    )
                ).unwrap();
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

impl HillDec {
    fn valid_key(&self) -> bool {
        let mut answer = !self.key.is_empty()
        && self.key
            .chars()
            .all(|c| c.is_ascii_whitespace() || c.is_ascii_digit());
        let splited_transformed: Vec<u32> = self.key
            .split_whitespace()
            .map(|s| u32::from_str_radix(s, 10).unwrap())
            .collect();
        answer = answer 
        && (splited_transformed.len() == 9 
            || splited_transformed.len() == 16);
        answer = answer && splited_transformed.iter()
            .all(|x| &0 <= x && x <= &255);
        answer 
    }
    fn hill_cipher(plain_image: image::DynamicImage, key: &Vec<Vec<i32>>) -> ImageBuffer<image::Rgba<u8>, Vec<u8>>{
        let plain_image_resize = plain_image.resize_exact(
            plain_image.dimensions().0 / key.len() as u32 * key.len() as u32,
            plain_image.dimensions().1 / key.len() as u32 * key.len() as u32,
            image::imageops::FilterType::Gaussian
        );
        let (w, h) = plain_image_resize.dimensions();
        let mut new_image = ImageBuffer::new(w, h);
        let mut accumulator = Vec::new();
        for (x, y, pixel) in plain_image_resize.pixels() {
            if accumulator.len() < key.len().pow(2) {
                accumulator.push((x, y, pixel.to_luma().channels()[0] as i32));
            } else {
                let mut pixel_matrix: Vec<Vec<i32>> = vec![vec![0; key.len()]; key.len()];
                for i in 0..key.len() {
                    for j in 0..key.len() {
                        pixel_matrix[i][j] = accumulator[i * key.len() + j].2;
                    }
                }
                let mut new_pixels = matrix::multiply(key.to_vec(), matrix::modular_matrix_multiplicative_inverse(&pixel_matrix, 256));
                matrix::modulus(&mut new_pixels, 256);
                for i in 0..key.len() {
                    for j in 0..key.len() {
                        let (px, py, _) = accumulator[i * key.len() + j];
                        new_image.put_pixel(
                            px, py,
                            image::Rgba([
                                new_pixels[i][j] as u8,
                                new_pixels[i][j] as u8,
                                new_pixels[i][j] as u8,
                                100
                            ])
                        );
                    }
                }
                accumulator = Vec::new();
            }
        }
        new_image
    }
}
