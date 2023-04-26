use std::collections::HashMap;

use crate::components::chat_list_item::ChatListItem;


pub struct AppState {
    counter: i32,
    chat_tree: HashMap<String, Vec<ChatListItem>>,
}

impl AppState {
    fn new_chat(&mut self, topic: String) {
        self.chat_tree.insert(topic, Vec::new());
    }
}

impl Default for AppState {
    fn default() -> Self {
        let mut chat_tree = HashMap::new();
        let mut messages = Vec::new();
        chat_tree.insert("test topic".to_string(), messages);
        Self {
            counter: 0,
            chat_tree,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("side-panel").show(ctx, |ui|{
            ui.with_layout(eframe::egui::Layout::top_down(eframe::egui::Align::Center), |ui| {
                ui.heading("Left Panel");
                eframe::egui::Grid::new("my_grid").striped(true).min_col_width(200.).show(ui, |ui| {
                    for (key, _value) in self.chat_tree.iter() {
                        ui.label(format!("{}", key));
                        if ui.button("Select").clicked() {
                            println!("Selected {}", key);
                        }
                        ui.end_row();
                    } 
                });
                if ui.button("New Chat").clicked() {
                    self.new_chat("New Chat".to_string());
                }
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
