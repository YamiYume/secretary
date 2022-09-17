pub mod secretary_app_windows;

mod ciphertext_input;
mod key_input;
mod plaintext_input;

mod ciphertext_output;
mod plaintext_output;

//pub mod afin_atk;
//pub mod afin_dec;
//pub mod afin_enc;
//pub mod caesar_atk;
//pub mod caesar_dec;
pub mod caesar_enc;
//pub mod hill_enc;
//pub mod perm_dec;
//pub mod perm_enc;
//pub mod vigenere_dec;
//pub mod vigenere_enc;

pub use ciphertext_input::CiphertextInput;
pub use ciphertext_output::CiphertextOutput;
pub use key_input::KeyInput;
pub use plaintext_input::PlaintextInput;
pub use secretary_app_windows::SecretaryWindows;

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Idle,
    Waiting,
    Valid,
    Invalid(&'static str),
}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Tool {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

pub trait EncryptTool {
    fn valid_plaintext(&mut self);
    fn valid_key(&mut self);
    fn encrypt(&mut self);
}

pub trait CryptoAscciExt<Rhs = Self> {
    fn as_crypto(self) -> u32;
    fn from_crypto(z26: u32) -> Rhs;
}

impl CryptoAscciExt for char {
    fn as_crypto(self) -> u32 {
        let x = self as u32;
        match x {
            65..=90 => return x - 65,
            97..=122 => return x - 97,
            _ => panic!("Non alphabetic"),
        }
    }
    fn from_crypto(z26: u32) -> char {
        match z26 {
            0..=25 => return char::from_u32(z26 + 97).unwrap(),
            _ => panic!("Non z26 value"),
        }
    }
}
