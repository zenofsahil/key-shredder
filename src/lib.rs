mod ui;
mod custom_types;

use custom_types::ReservedString;
use eframe::egui::{
    self,
    TextFormat,
    text::LayoutJob,
    Color32,
    epaint::text::{FontId, FontFamily}};
use crate::ui::Keyboard;

#[derive(Default)]
pub struct KeyShredder {
    keyboard: Keyboard,
    text: ReservedString
}

impl eframe::App for KeyShredder {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                ui.heading("Key Shredder");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {

            let hint_text = "Enter your text here";

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut job = LayoutJob::default();
                job.wrap.max_width = wrap_width;

                // This block will not be needed if we have a custom string
                // type limiting the characters the string can hold in the
                // first place.
                // if string.len() > hint_text.len() {
                //     string = &string[0..hint_text.len()];
                // }

                let hint_words = hint_text.split(" ");
                let user_words = string.split(" ");

                for (hint_word, user_word) in hint_words.zip(user_words) {
                    match (hint_word, user_word) {
                        (hint_word, user_word) if hint_word == user_word => {
                            job.append(
                                &format!("{} ", user_word),
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(14.0, FontFamily::Monospace),
                                    color: Color32::GREEN,
                                    ..Default::default()
                                },
                            );
                        },
                        (hint_word, user_word) if hint_word.len() != user_word.len() => {
                            job.append(
                                &format!("{} ", user_word),
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(14.0, FontFamily::Monospace),
                                    color: Color32::YELLOW,
                                    ..Default::default()
                                },
                            );
                        },
                        (hint_word, user_word) if (
                            (hint_word.len() == user_word.len()) &&
                            (hint_word != user_word)
                        ) => {
                            job.append(
                                &format!("{} ", user_word),
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(14.0, FontFamily::Monospace),
                                    color: Color32::RED,
                                    ..Default::default()
                                },
                            );
                        },
                        _ => { println!("{}, {}", hint_word, user_word) }
                    }
                }

                ui.fonts().layout_job(job)
            };

            let desired_width = f32::INFINITY;
            let text_edit: egui::TextEdit = egui::TextEdit::multiline(&mut self.text)
                .desired_width(desired_width)
                .layouter(&mut layouter);

            let text_edit_output = text_edit.show(ui);
            let painter = ui.painter_at(text_edit_output.response.rect);

            let galley = painter.layout(
                hint_text.to_string(),
                FontId::new(14., FontFamily::Monospace),
                Color32::from_rgba_premultiplied(100, 100, 100, 0),
                desired_width
            );

            painter.galley(text_edit_output.text_draw_pos, galley);
        });

        let events = ctx.input().events.clone(); // clone to avoid deadlock
        for event in events.iter() {
            match *event {
                egui::Event::Key {
                    key,
                    pressed,
                    modifiers: _
                } if pressed == true => {
                    self.keyboard.press_key(key);
                },
                egui::Event::Key {
                    key: egui::Key::W,
                    pressed: _,
                    modifiers,
                } if modifiers.mac_cmd || modifiers.command => {
                        frame.quit()
                },
                _ => { },
            }
        };
        if let Some(pressed_key) = &self.keyboard.pressed_key {
            if ctx.input().key_released(pressed_key.key) {
                self.keyboard.pressed_key = None;
            }
        }
        egui::TopBottomPanel::bottom("Keyboard").show(ctx, |ui| {
            self.keyboard.draw(ui);
        });
    }
}

impl KeyShredder {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            keyboard: Keyboard::default(),
            text: ReservedString(String::new(), 10)
        }
    }
}

