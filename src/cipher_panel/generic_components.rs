use eframe::egui::{self, RichText, Color32, TextStyle};
use rand::prelude::ThreadRng;
use crate::ciphers::Cipher;

pub fn encrypt_decrypt(ui: &mut egui::Ui, cipher: &dyn Cipher, input: &mut String, output: &mut String, errors: &mut String) {
    ui.horizontal(|ui| {
        if ui.button(RichText::from("ENCRYPT").color(Color32::GOLD)).clicked() {
            errors.clear();
            match cipher.encrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        };
        if ui.button(RichText::from("DECRYPT").color(Color32::GOLD)).clicked() {
            errors.clear();
            match cipher.decrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        }
    });
}

pub fn randomize_button(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    let mut rng = ThreadRng::default();
    if ui.button("Randomize\nSettings").clicked() {
        cipher.randomize(&mut rng)
    }
}

pub fn input_alphabet(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
    ui.label("Alphabet");
    ui.add(egui::TextEdit::singleline(cipher.get_mut_input_alphabet()).font(TextStyle::Monospace)).on_hover_text("You can change this alphabet to whatever you like.");
}


pub fn control_text_edit(ui: &mut egui::Ui, text: &mut String) {
    ui.add(egui::TextEdit::singleline(text).font(TextStyle::Monospace));
}

// pub fn select_square_alphabet(ui: &mut egui::Ui, cipher: &mut dyn Cipher) {
//     ui.label("Select Alphabet");
//     ui.horizontal(|ui| {
//         if ui.button("No Q").clicked() { cipher.set_alphabet(PresetAlphabet::BasicLatinNoQ) };
//         if ui.button("No J").clicked() { cipher.set_alphabet(PresetAlphabet::BasicLatinNoJ) };
//         if ui.button("Alphanumeric").clicked() { cipher.set_alphabet(PresetAlphabet::BasicLatinWithDigits) };
//         if ui.button("Base64").clicked() { cipher.set_alphabet(PresetAlphabet::Base64) };
//     });
// }