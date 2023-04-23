 fn test() {
    println!("test");
}
pub struct ChatEntry {
    name: String,
    message: String,
}
pub struct AppState {
    counter: i32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            counter: 0,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("side-panel").show(ctx, |ui|{
            ui.with_layout(eframe::egui::Layout::bottom_up(eframe::egui::Align::Center), |ui| {
                ui.heading("Left Panel");
                ui.add(eframe::egui::widgets::Button::new("Button"));
            }); 
        }); 
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("test").clicked() {
                self.counter += 1;
            }
            ui.label(format!("Counter: {}", self.counter));
        });
    }
}
