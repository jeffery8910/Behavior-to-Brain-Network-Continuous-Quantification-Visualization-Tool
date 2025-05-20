mod risk;
use eframe::{egui, epi};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct BehaviorBrainMap {
    behavior: String,
    brain_regions: Vec<BrainRegionImpact>,
}

#[derive(Deserialize, Debug)]
struct BrainRegionImpact {
    region: String,
    weight: f32,
}

fn load_behavior_brain_map(path: &str) -> Vec<BehaviorBrainMap> {
    let data = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

struct App {
    behavior_options: Vec<String>,
    selected_behavior: usize,
    input_value: f32,
    map_data: Vec<BehaviorBrainMap>,
    risk_text: String,
}

impl Default for App {
    fn default() -> Self {
        let map_data = load_behavior_brain_map("data/behavior_brain_map.json");
        let behavior_options = map_data.iter().map(|b| b.behavior.clone()).collect();
        Self {
            behavior_options,
            selected_behavior: 0,
            input_value: 0.0,
            map_data,
            risk_text: String::new(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Behavior-to-Brain Network Quantification Tool"
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("行為-腦網路連續量化可視化工具");
            ui.horizontal(|ui| {
                ui.label("選擇行為:");
                egui::ComboBox::from_id_source("behavior_combo")
                    .selected_text(self.behavior_options.get(self.selected_behavior).unwrap_or(&"-".to_string()))
                    .show_ui(ui, |cb| {
                        for (i, b) in self.behavior_options.iter().enumerate() {
                            cb.selectable_value(&mut self.selected_behavior, i, b);
                        }
                    });
            });
            ui.horizontal(|ui| {
                ui.label("輸入次數/時長/比率:");
                ui.add(egui::DragValue::new(&mut self.input_value));
            });
            if ui.button("估算").clicked() {
                self.risk_text = risk::estimate_risk(self.input_value);
            }
            ui.separator();
            ui.label(format!("風險提示: {}", self.risk_text));
            ui.separator();
            ui.label("腦圖預覽 (SVG)");
            // SVG 載入與顯示（簡化版）
            if let Ok(svg) = std::fs::read_to_string("assets/brain.svg") {
                ui.label(egui::RichText::new(svg).monospace());
            } else {
                ui.label("找不到 brain.svg");
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(App::default()),
        options,
    );
}
