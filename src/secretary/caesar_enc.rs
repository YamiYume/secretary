use super::CryptoAscciExt;
use super::{CiphertextOutput, KeyInput, PlaintextInput, Status};
use super::{EncryptTool, Tool, View};

#[derive(Debug, Default)]
pub struct CaesarEnc {
    plaintext: PlaintextInput<String>,
    key: KeyInput<u32>,
    ciphertext: CiphertextOutput<String, String>,
}

impl Tool for CaesarEnc {
    fn name(&self) -> &'static str {
        "ﲘ Caesar"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .scroll2([false, true])
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}

impl View for CaesarEnc {
    fn ui(&mut self, ui: &mut egui::Ui) {
        self.plaintext.ui(ui);
        if self.plaintext.response.as_ref().unwrap().changed() {
            self.plaintext.format()
        }
        if let Status::Waiting = self.plaintext.status {
            self.valid_plaintext();
        }
        self.key.ui(ui);
        if self.key.response.as_ref().unwrap().changed() {
            self.key.format()
        }
        if let Status::Waiting = self.key.status {
            self.valid_key();
        }
        self.ciphertext.ui(ui);
        if let (Status::Valid, Status::Valid) = (&self.plaintext.status, &self.key.status) {
            self.encrypt();
        } else {
            self.ciphertext.ciphertext = String::default();
        }
    }
}

impl EncryptTool for CaesarEnc {
    fn valid_key(&mut self) {
        if self.key.formated.len() == 1 {
            self.key.formated[0] %= 26;
            self.key.status = Status::Valid;
        } else {
            self.key.status = Status::Invalid("key is supposed to be a single number")
        }
    }
    fn valid_plaintext(&mut self) {
        self.plaintext.status = Status::Valid;
    }
    fn encrypt(&mut self) {
        let mut ciphertext = String::default();
        for c in self.plaintext.formated.chars() {
            let new_char = char::from_crypto((c.as_crypto() + self.key.formated[0]) % 26);
            ciphertext.push(new_char);
        }
        self.ciphertext.ciphertext = ciphertext;
        self.ciphertext.represent();
    }
}
