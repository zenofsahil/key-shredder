use eframe::egui::{self, Event, Key, Modifiers};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Key Shredder",
        options,
        Box::new(|_cc| Box::new(KeyShredder::default())),
    );
}

#[derive(Default)]
struct KeyShredder {
}

impl eframe::App for KeyShredder {
     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for e in ctx.input().events.iter() {
            match *e {
                Event::Key { key, pressed, modifiers } if pressed == true => {
                    println!("{:?}, {:?}, {:?}", key, pressed, modifiers);
                },
                _ => {},
            }
        }
     }
}
