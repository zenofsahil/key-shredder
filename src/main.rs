fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Key Shredder",
        options,
        Box::new(|cc| Box::new(key_shredder::KeyShredder::new(cc))),
    );
}
