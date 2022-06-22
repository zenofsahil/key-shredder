use crate::ui::utils;
use eframe::egui::{self, Color32};

pub enum KeyState {
    Pressed,
    Released
}

pub struct Key {
    pub key: egui::Key,
    pub character: String ,
    pub state: KeyState
}

impl Key {
    pub fn from_str(c: &str) -> Self {
        let converter = utils::KeyConversion::new();
        let c = c.to_string();
        Self {
            character: c.clone(),
            state: KeyState::Released,
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
            state: KeyState::Released,
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
pub struct Row {
    keys: Vec<Key>
}

impl Row {
    pub fn from_chars(s: &str) -> Self {
        let keys = s
            .chars()
            .map(|x| x.to_string())
            .map(|x| Key::from_str(&x))
            .collect();
        Self {
            keys
        }
    }
}

#[derive(Default)]
pub struct Keyboard {
    pub first_row: Row,
    pub second_row: Row,
    pub third_row: Row,
}

impl Keyboard {
    pub fn default() -> Self {
        let first_row = Row::from_chars("qwertyuiop");
        let second_row = Row::from_chars("asdfghjkl");
        let third_row = Row::from_chars("zxcvbnm");

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
            let row_margins = vec![
                (&self.first_row, (10., 0.)),
                (&self.second_row, (30., 0.)),
                (&self.third_row, (40., 0.))
            ];

            for (row, (x_margin, y_margin)) in row_margins {
                egui::Frame::none()
                .inner_margin(egui::style::Margin::symmetric(x_margin, y_margin))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                    ui.horizontal(|ui| {
                        for key in &row.keys {
                            key.draw(ui);
                        }
                    });
                });
            }
        });
    }

    pub fn press_key(&self, key: egui::Key) {
    }
}


#[cfg(test)]
mod test_keys_struct {
    use crate::ui::keyboard::{Key, KeyState};
    use eframe::egui;

    #[test]
    fn test_key_initialization_from_string() {
        let key_a = Key::from_str("a");
        assert!(matches!(key_a.state, KeyState::Released));
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
        assert_eq!(keyboard.first_row.keys.len(), 10);
        assert_eq!(keyboard.second_row.keys.len(), 9);
        assert_eq!(keyboard.third_row.keys.len(), 7);
    }
}
