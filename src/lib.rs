mod ui;

use eframe::egui;
use crate::ui::Keyboard;

#[derive(Default)]
pub struct KeyShredder {
    keyboard: Keyboard,
    text: String
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

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                use eframe::epaint::{Color32, text::{LayoutJob, TextFormat}, FontFamily, FontId};

                let mut job = LayoutJob::default();
                job.wrap.max_width = wrap_width;

                for word in string.split(" ") {
                    match word {
                        "hello" => {
                            job.append(
                                &format!("{} ", word),
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(14.0, FontFamily::Monospace),
                                    color: Color32::YELLOW,
                                    ..Default::default()
                                },
                            );
                        },
                        "world" => {
                            job.append(
                                &format!("{} ", word),
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(14.0, FontFamily::Monospace),
                                    color: Color32::GREEN,
                                    ..Default::default()
                                },
                            );
                        },
                        _ => {
                            job.append(
                                &format!("{} ", word),
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(14.0, FontFamily::Monospace),
                                    color: Color32::WHITE,
                                    ..Default::default()
                                },
                            );
                        }
                    }
                }

                ui.fonts().layout_job(job)
            };
            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                .desired_width(f32::INFINITY)
                .hint_text("Enter you text here")
                .layouter(&mut layouter)
            );
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
            text: String::new()
        }
    }
}

