use std::collections::HashMap;

pub struct ChatEntry {
    pub name: String,
    pub message: String,
}
pub struct AppState {
    counter: i32,
    chat_tree: HashMap<String, ChatEntry>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut chat_tree = HashMap::new();
        chat_tree.insert("test".to_string(), ChatEntry {
            name: "test".to_string(),
            message: "test".to_string(),
        });
        Self {
            counter: 0,
            chat_tree: HashMap::new(),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("side-panel").show(ctx, |ui|{
            ui.with_layout(eframe::egui::Layout::top_down(eframe::egui::Align::Center), |ui| {
                ui.heading("Left Panel");
                eframe::egui::Grid::new("my_grid").striped(true).show(ui, |ui| {
                    ui.label("Label");
                });
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
