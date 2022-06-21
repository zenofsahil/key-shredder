mod ui;

use eframe::egui;
use crate::ui::Keyboard;

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

