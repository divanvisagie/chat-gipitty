use crate::app::AppState;

mod app;
mod components;

fn main() {
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
