mod utils;

use eframe::egui::{self, Color32};

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

#[derive(Default)]
struct Keyboard {
    first_row: Vec<egui::Key>,
    second_row: Vec<egui::Key>,
    third_row: Vec<egui::Key>,
}

impl Keyboard {
    fn default() -> Self {
        let converter = utils::KeyConversion::new();
        Self {
            first_row: converter.map_string_to_keys("qwertyuiop"),
            second_row: converter.map_string_to_keys("asdfghjkl"),
            third_row: converter.map_string_to_keys("zxcvbnm")
        }
    }

    fn draw(&self, ui: &mut egui::Ui) {
        egui::Frame::none()
        .inner_margin(egui::style::Margin::symmetric(ui.available_width() / 3.0 , 0.0))
        .show(ui, |ui| {
            egui::Frame::none().inner_margin(egui::style::Margin::symmetric(10., 0.)).show(ui, |ui| {
                ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                ui.horizontal(|ui| {
                    for key in &self.first_row {
                        egui::Frame::none().fill(BLACK).show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:?}", key))
                                .size(20.)
                                .color(Color32::WHITE)
                            )
                        });
                    }
                });
            });
            egui::Frame::none().inner_margin(egui::style::Margin::symmetric(30., 0.)).show(ui, |ui| {
                ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                ui.horizontal(|ui| {
                    for key in &self.second_row {
                        egui::Frame::none().fill(BLACK).show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:?}", key))
                                .size(20.)
                                .color(Color32::WHITE)
                            )
                        });
                    }
                });
            });
            egui::Frame::none().inner_margin(egui::style::Margin::symmetric(40., 0.)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                    for key in &self.third_row {
                        egui::Frame::none().fill(BLACK).show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:?}", key))
                                .size(20.)
                                .color(Color32::WHITE)
                            )
                        });
                    }
                });
            });
        });
    }
}

#[derive(Default)]
pub struct KeyShredder {
    keyboard: Keyboard
}

impl eframe::App for KeyShredder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                ui.heading("Key Shredder");
            })
        });
        egui::TopBottomPanel::bottom("Keyboard").show(ctx, |ui| {
            self.keyboard.draw(ui);
        });
    }

    // fn _update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    //     // ctx.request_repaint();
    //     tracing::error!("Running...");
    //     ctx.set_visuals(egui::Visuals::dark());
    //     let events = ctx.input().events.clone(); // clone to avoid deadlock
    //     let es = events.into_iter().map(|event| {
    //         match event {
    //             Event::Key { key, pressed, modifiers } if pressed == true => {
    //                 println!("{:?}, {:?}, {:?}", key, pressed, modifiers);
    //                 Some(key)
    //             },
    //             _ => {
    //                 println!("No key pressed");
    //                 None
    //             },
    //         }
    //     });

    //     for e in es {
    //         self.render_keyboard(ctx, e);
    //         // if let Some(_key) = e {
    //         //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //         // }
    //     }
    //     // let mut highlight_key = for event in events.iter() {
    //     //     match *event {
    //     //         Event::Key { key, pressed, modifiers } if pressed == true => {
    //     //             println!("{:?}, {:?}, {:?}", key, pressed, modifiers);
    //     //             Some(key)
    //     //         },
    //     //         _ => {
    //     //             println!("No key pressed");
    //     //             None
    //     //         },
    //     //     }
    //     // }
    // }
}

impl KeyShredder {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            keyboard: Keyboard::default()
        }
    }
}

