use super::Tool;
use super::{
    caesar_enc::CaesarEnc, caesar_dec::CaesarDec, caesar_atk::CaesarAtk,
    afin_enc::AfinEnc, afin_dec::AfinDec, afin_atk::AfinAtk,
    vigenere_enc::VigenereEnc, vigenere_dec::VigenereDec,
    perm_enc::PermEnc, perm_dec::PermDec,
    hill_enc::HillEnc,
};
use egui::{Context, ScrollArea, Ui};
use std::collections::BTreeSet;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", derive(default))]

struct Encryptors {
    cryptos: Vec<Box<dyn Tool>>,
    open: BTreeSet<String>,
}

impl Default for Encryptors {
    fn default() -> Self {
        Self::from_cryptos(vec![
            Box::new(CaesarEnc::default()),
            Box::new(VigenereEnc::default()),
            Box::new(AfinEnc::default()),
            Box::new(PermEnc::default()),
            Box::new(HillEnc::default()),
        ])
    }
}

impl Encryptors {
    pub fn from_cryptos(cryptos: Vec<Box<dyn Tool>>) -> Self {
        let open = BTreeSet::new();
        Self { cryptos, open }
    }
    pub fn checkboxes(&mut self, ui: &mut Ui) -> () {
        let Self { cryptos, open } = self;
        for crypto in cryptos {
            let mut is_open = open.contains(crypto.name());
            ui.toggle_value(&mut is_open, crypto.name());
            set_open(open, crypto.name(), is_open);
        }
    }
    pub fn windows(&mut self, ctx: &Context) {
        let Self { cryptos, open } = self;
        for crypto in cryptos {
            let mut is_open = open.contains(crypto.name());
            crypto.show(ctx, &mut is_open);
            set_open(open, crypto.name(), is_open)
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", derive(default))]

struct Decryptors {
    decryptos: Vec<Box<dyn Tool>>,
    open: BTreeSet<String>,
}

impl Default for Decryptors {
    fn default() -> Self {
        Self::from_decryptos(vec![
            Box::new(CaesarDec::default()),
            Box::new(VigenereDec::default()),
            Box::new(AfinDec::default()),
            Box::new(PermDec::default()),
        ])
    }
}

impl Decryptors {
    pub fn from_decryptos(decryptos: Vec<Box<dyn Tool>>) -> Self {
        let open = BTreeSet::new();
        Self { decryptos, open }
    }
    pub fn checkboxes(&mut self, ui: &mut Ui) -> () {
        let Self { decryptos, open } = self;
        for decrypto in decryptos {
            let mut is_open = open.contains(decrypto.name());
            ui.toggle_value(&mut is_open, decrypto.name());
            set_open(open, decrypto.name(), is_open);
        }
    }
    pub fn windows(&mut self, ctx: &Context) {
        let Self { decryptos, open } = self;
        for decrypto in decryptos {
            let mut is_open = open.contains(decrypto.name());
            decrypto.show(ctx, &mut is_open);
            set_open(open, decrypto.name(), is_open)
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", derive(default))]

struct Attackers {
    attacks: Vec<Box<dyn Tool>>,
    open: BTreeSet<String>,
}

impl Default for Attackers {
    fn default() -> Self {
        Self::from_attacks(vec![
            Box::new(CaesarAtk::default()),
            Box::new(AfinAtk::default()),
        ])
    }
}

impl Attackers {
    pub fn from_attacks(attacks: Vec<Box<dyn Tool>>) -> Self {
        let open = BTreeSet::new();
        Self { attacks, open }
    }
    pub fn checkboxes(&mut self, ui: &mut Ui) -> () {
        let Self { attacks, open } = self;
        for attack in attacks {
            let mut is_open = open.contains(attack.name());
            ui.toggle_value(&mut is_open, attack.name());
            set_open(open, attack.name(), is_open);
        }
    }
    pub fn windows(&mut self, ctx: &Context) {
        let Self { attacks, open } = self;
        for attack in attacks {
            let mut is_open = open.contains(attack.name());
            attack.show(ctx, &mut is_open);
            set_open(open, attack.name(), is_open)
        }
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) -> () {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", derive(default))]

pub struct SecretaryWindows {
    encryptors: Encryptors,
    decryptors: Decryptors,
    attackers: Attackers,
}

impl Default for SecretaryWindows {
    fn default() -> Self {
        Self {
            encryptors: Default::default(),
            decryptors: Default::default(),
            attackers: Default::default(),
        }
    }
}

impl SecretaryWindows {
    pub fn ui(&mut self, ctx: &Context) -> () {
        self.desktop_ui(ctx)
    }
    fn desktop_ui(&mut self, ctx: &Context) -> () {
        egui::SidePanel::right("secretary_menu")
            .resizable(false)
            .default_width(160.0)
            .show(ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading(" SecretAry");
                });
                ui.separator();
                use egui::special_emojis::GITHUB;
                ui.vertical_centered(|ui| {
                    ui.hyperlink_to(
                        format!("{} Project github", GITHUB),
                        "https://github.com/YamiYume/secretary",
                    );
                });
                ui.separator();
                self.secretary_list_ui(ui)
            });
        self.show_windows(ctx);
    }
    fn show_windows(&mut self, ctx: &Context) -> () {
        self.encryptors.windows(ctx);
        self.decryptors.windows(ctx);
        self.attackers.windows(ctx);
    }
    fn secretary_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                egui::CollapsingHeader::new("Encrypting Tools")
                    .default_open(true)
                    .show(ui, |ui| {
                        self.encryptors.checkboxes(ui);
                    });
                ui.separator();
                egui::CollapsingHeader::new("Decrypting Tools")
                    .default_open(true)
                    .show(ui, |ui| {
                        self.decryptors.checkboxes(ui);
                    });
                ui.separator();
                egui::CollapsingHeader::new("Attacking Tools")
                    .default_open(true)
                    .show(ui, |ui| {
                        self.attackers.checkboxes(ui);
                    });
            });
        });
    }
}
