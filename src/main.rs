use eframe::egui::{self, Event};

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
         let keys_down = ctx.input().keys_down.clone();

         println!("Keys pressed: {:?}", &keys_down);
     }
}

