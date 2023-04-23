struct AppState {}

impl Default for AppState {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {}
}
fn main() {
    println!("Hello, world!");

    //init eframe window
    let app = eframe::NativeOptions::default();
    match eframe::run_native(
        "Chat Gipity",
        app,
        Box::new(|_cc| Box::<AppState>::default()),
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
