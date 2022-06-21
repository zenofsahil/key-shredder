use crate::ui::utils;
use eframe::egui::{self, Color32};

pub struct Key {
    pub key: egui::Key,
    pub character: String ,
    pub pressed: bool
}

impl Key {
    pub fn from_str(c: &str) -> Self {
        let converter = utils::KeyConversion::new();
        let c = c.to_string();
        Self {
            character: c.clone(),
            pressed: false,
            key: converter.char2keys
                .get(&c.to_string() as &str)
                .copied()
                .expect(&format!("Invalid key: {}", c))
        }
    }

    pub fn from_key(key: egui::Key) -> Self {
        let converter = utils::KeyConversion::new();
        Self {
            key,
            pressed: false,
            character: converter.keys2char
                .get(&key)
                .copied()
                .map(|x| x.to_string())
                .unwrap_or(String::new())
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        egui::Frame::none().fill(Color32::BLACK).show(ui, |ui| {
            ui.label(
                egui::RichText::new(&self.character)
                .size(20.)
                .color(Color32::WHITE)
            )
        });
    }
}

#[derive(Default)]
pub struct Keyboard {
    pub first_row: Vec<Key>,
    pub second_row: Vec<Key>,
    pub third_row: Vec<Key>,
}

impl Keyboard {
    pub fn default() -> Self {
        let first_row = "qwertyuiop"
            .chars()
            .map(|x| x.to_string())
            .map(|x| Key::from_str(&x))
            .collect();

        let second_row = "asdfghjkl"
            .chars()
            .map(|x| x.to_string())
            .map(|x| Key::from_str(&x))
            .collect();

        let third_row = "zxcvbnm"
            .chars()
            .map(|x| x.to_string())
            .map(|x| Key::from_str(&x))
            .collect();

        Self {
            first_row,
            second_row,
            third_row
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        egui::Frame::none()
        .inner_margin(egui::style::Margin::symmetric(ui.available_width() / 3.0 , 0.0))
        .show(ui, |ui| {
            egui::Frame::none().inner_margin(egui::style::Margin::symmetric(10., 0.)).show(ui, |ui| {
                ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                ui.horizontal(|ui| {
                    for key in &self.first_row {
                        key.draw(ui);
                    }
                });
            });
            egui::Frame::none().inner_margin(egui::style::Margin::symmetric(30., 0.)).show(ui, |ui| {
                ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                ui.horizontal(|ui| {
                    for key in &self.second_row {
                        key.draw(ui);
                    }
                });
            });
            egui::Frame::none().inner_margin(egui::style::Margin::symmetric(40., 0.)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                    for key in &self.third_row {
                        key.draw(ui);
                    }
                });
            });
        });
    }
}


#[cfg(test)]
mod test_keys_struct {
    use crate::ui::keyboard::Key;
    use eframe::egui;

    #[test]
    fn test_key_initialization_from_string() {
        let key_a = Key::from_str("a");
        assert_eq!(key_a.pressed, false);
        assert_eq!(key_a.character, "a");
        assert_eq!(key_a.key, egui::Key::A);
    }

    #[test]
    #[should_panic]
    fn test_period_key_initialization_from_string() {
        let _period = Key::from_str(".");
    }
}

#[cfg(test)]
mod test_keyboard_struct {
    use crate::ui::keyboard::Keyboard;
    // use eframe::egui;

    #[test]
    fn test_keyboard_initialization() {
        let keyboard = Keyboard::default();
        assert_eq!(keyboard.first_row.len(), 10);
        assert_eq!(keyboard.second_row.len(), 9);
        assert_eq!(keyboard.third_row.len(), 7);
    }
}
