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
    text: ReservedString,
    corpus: Vec<String>
}

impl eframe::App for KeyShredder {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                ui.heading("Key Shredder");
            })
        });
        let hint_text = &self.corpus[0].clone();
        egui::CentralPanel::default().show(ctx, |ui| {

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut job = LayoutJob::default();
                job.wrap.max_width = wrap_width;

                let hint_words = hint_text.split(" ");
                let user_words = string.split(" ");

                let zipped_words = hint_words.zip(user_words);

                for (hint_word, user_word) in zipped_words {
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
                        (hint_word, user_word) if hint_word.len() < user_word.len() => {
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
                        (hint_word, user_word) if hint_word.len() > user_word.len() => {
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
                        _ => { println!("{}, {}", hint_word, user_word) }
                    }
                }

                ui.fonts().layout_job(job)
            };

            if self.text.0 == self.corpus[0] {
                if self.corpus.len() > 1 {
                    self.corpus.remove(0);
                    self.text = ReservedString(String::new(), self.corpus[0].len())
                }
            }

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
        let corpus = vec![
            "Enter your text",
            "This is the second line",
            "This is the third line"
        ];
        let corpus: Vec<_> = corpus.iter().map(|x| x.to_string()).collect();

        let text = ReservedString(String::new(), corpus[0].len() - 1);

        Self {
            keyboard: Keyboard::default(),
            corpus,
            text,
        }
    }
}

