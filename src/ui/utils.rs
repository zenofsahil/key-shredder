use eframe::egui;
use std::collections::HashMap;

pub struct KeyConversion<'a> {
    pub keys2char: HashMap<egui::Key, &'a str>,
    pub char2keys: HashMap<&'a str, egui::Key>
}

impl<'a> KeyConversion<'a> {
    pub fn new() -> Self {
        let keys2char = HashMap::from([
                (egui::Key::A, "a"),
                (egui::Key::B, "b"),
                (egui::Key::C, "c"),
                (egui::Key::D, "d"),
                (egui::Key::E, "e"),
                (egui::Key::F, "f"),
                (egui::Key::G, "g"),
                (egui::Key::H, "h"),
                (egui::Key::I, "i"),
                (egui::Key::J, "j"),
                (egui::Key::K, "k"),
                (egui::Key::L, "l"),
                (egui::Key::M, "m"),
                (egui::Key::N, "n"),
                (egui::Key::O, "o"),
                (egui::Key::P, "p"),
                (egui::Key::Q, "q"),
                (egui::Key::R, "r"),
                (egui::Key::S, "s"),
                (egui::Key::T, "t"),
                (egui::Key::U, "u"),
                (egui::Key::V, "v"),
                (egui::Key::W, "w"),
                (egui::Key::X, "x"),
                (egui::Key::Y, "y"),
                (egui::Key::Z, "z")
        ]);

        let char2keys = keys2char
            .iter()
            .map(|(key, val)| (val.clone(), key.clone()))
            .collect();

        Self {
            keys2char,
            char2keys
        }
    }
}

