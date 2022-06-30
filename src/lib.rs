mod ui;
mod custom_types;

use std::time::{SystemTime, UNIX_EPOCH};
use custom_types::ReservedString;
use eframe::egui::{
    self,
    TextFormat,
    text::LayoutJob,
    Color32,
    epaint::text::{FontId, FontFamily}};
use crate::ui::Keyboard;

pub struct KeyShredder {
    keyboard: Keyboard,
    text: ReservedString,
    corpus: Vec<String>,
    start_time: Option<u64>,
    word_completion_time: std::collections::HashMap<String, u64>
}

impl eframe::App for KeyShredder {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.text.0.len() {
            0 => {
                self.start_time = None // Reset start time if all the text has been removed
            },
            1 => {
                let in_ms = get_current_time();
                if let None = self.start_time {
                    self.start_time = Some(in_ms)
                }
            }
            _ => {}
        };

        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                ui.heading("Key Shredder");
                // println!("In {:?}", self.start_time);
                if let Some(time) = self.start_time {
                    // println!("Found time");
                    ui.heading(format!("{}", time));
                }
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
                        (_, "") => {
                            continue
                        },
                        (hint_word, user_word) if hint_word == user_word => {

                            self.word_completion_time
                                .entry(user_word.to_string())
                                .or_insert(get_current_time());

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

            ui.vertical_centered(|ui| {
                if let Some(time) = self.start_time {
                    ui.label(format!("{}", time));
                }
                for (word, time) in &self.word_completion_time {
                    ui.label(format!("{} : {}", word, time));
                }
            })
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

        // let (word_tx, word_rx) = channel();

        let key_shredder = Self {
            keyboard: Keyboard::default(),
            corpus,
            text,
            // start_timer_tx,
            start_time: None,
            word_completion_time: std::collections::HashMap::new()
        };

        key_shredder
    }
}

fn get_current_time() -> u64 {
    let now = SystemTime::now();
    let now_from_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_ms = now_from_epoch.as_secs() * 1000 +
        now_from_epoch.subsec_nanos() as u64 / 1_000_000;

    in_ms
}
