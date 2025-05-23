mod model;
mod mapping;
mod risk;
mod gui;

use eframe::egui;
use gui::BehaviorBrainApp;
use mapping::load_behavior_brain_map;

fn main() -> Result<(), eframe::Error> {
    // 設定 egui 視窗選項
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // 載入行為-腦區映射資料
    let behavior_maps = load_behavior_brain_map("data/behavior_brain_map.json")
        .unwrap_or_else(|e| {
            eprintln!("警告：無法載入 behavior_brain_map.json: {}", e);
            Vec::new()
        });

    if behavior_maps.is_empty() {
        eprintln!("警告：沒有載入到任何行為映射資料");
    }

    // 啟動應用程式
    eframe::run_native(
        "行為-腦網路連續量化可視化工具 v0.1.0",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(BehaviorBrainApp::new(behavior_maps))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    use eframe::egui::{FontDefinitions, FontFamily, FontData};
    
    let mut fonts = FontDefinitions::default();

    // 嘗試載入中文字體
    let chinese_fonts = [
        ("Microsoft JhengHei", "C:/Windows/Fonts/msjh.ttc"),
        ("Microsoft YaHei", "C:/Windows/Fonts/msyh.ttc"),
        ("DFKai-SB", "C:/Windows/Fonts/kaiu.ttf"),
        ("SimSun", "C:/Windows/Fonts/simsun.ttc"),
    ];

    for (font_name, font_path) in &chinese_fonts {
        if let Ok(font_data) = std::fs::read(font_path) {
            fonts.font_data.insert(
                font_name.to_string(),
                FontData::from_owned(font_data),
            );
            
            // 添加到字體家族
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, font_name.to_string());
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .insert(0, font_name.to_string());
            
            println!("已載入字體: {}", font_name);
            break; // 只要載入一個成功的字體就夠了
        }
    }

    ctx.set_fonts(fonts);
}

