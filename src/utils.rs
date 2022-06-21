use eframe::egui;
use std::collections::HashMap;

const KEY2CHAR: HashMap<egui::Key, String> = HashMap::from([
        (egui::Key::A, String::from("a")),
        (egui::Key::B, String::from("b")),
        (egui::Key::C, String::from("c")),
        (egui::Key::D, String::from("d")),
        (egui::Key::E, String::from("e")),
        (egui::Key::F, String::from("f")),
        (egui::Key::G, String::from("g")),
        (egui::Key::H, String::from("h")),
        (egui::Key::I, String::from("i")),
        (egui::Key::J, String::from("j")),
        (egui::Key::K, String::from("k")),
        (egui::Key::L, String::from("l")),
        (egui::Key::M, String::from("m")),
        (egui::Key::N, String::from("n")),
        (egui::Key::O, String::from("o")),
        (egui::Key::P, String::from("p")),
        (egui::Key::Q, String::from("q")),
        (egui::Key::R, String::from("r")),
        (egui::Key::S, String::from("s")),
        (egui::Key::T, String::from("t")),
        (egui::Key::U, String::from("u")),
        (egui::Key::V, String::from("v")),
        (egui::Key::W, String::from("w")),
        (egui::Key::X, String::from("x")),
        (egui::Key::Y, String::from("y")),
        (egui::Key::Z, String::from("z"))
]);

const CHAR2KEY: HashMap<String, egui::Key> = KEY2CHAR
    .into_iter()
    .map(|(key, val)| (val, key))
    .collect();
