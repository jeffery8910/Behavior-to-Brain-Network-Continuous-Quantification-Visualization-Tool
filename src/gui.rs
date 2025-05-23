use eframe::egui;
use crate::model::*;
use crate::mapping::MappingEngine;
use crate::risk::{RiskAssessment, RiskReport};

/// GUI 主應用程式結構
pub struct BehaviorBrainApp {
    // 核心引擎
    mapping_engine: MappingEngine,
    risk_assessment: RiskAssessment,
    
    // UI 狀態
    selected_behavior: usize,
    input_value: f32,
    input_unit: String,
    
    // 計算結果
    current_result: Option<CalculationResult>,
    current_risk_report: Option<RiskReport>,
    
    // 腦圖相關
    brain_svg_data: String,
    show_brain_regions: bool,
    
    // 歷史記錄
    calculation_history: Vec<CalculationResult>,
}

impl BehaviorBrainApp {
    pub fn new(behavior_maps: Vec<BehaviorBrainMap>) -> Self {
        let brain_svg_data = std::fs::read_to_string("assets/brain.svg")
            .unwrap_or_else(|_| "<svg>Brain SVG not found</svg>".to_string());
        
        Self {
            mapping_engine: MappingEngine::new(behavior_maps),
            risk_assessment: RiskAssessment::default(),
            selected_behavior: 0,
            input_value: 0.0,
            input_unit: "次".to_string(),
            current_result: None,
            current_risk_report: None,
            brain_svg_data,
            show_brain_regions: true,
            calculation_history: Vec::new(),
        }
    }

    /// 渲染左側控制面板
    fn render_control_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🧠 行為輸入");
        
        ui.separator();
        
        // 行為選擇
        ui.horizontal(|ui| {
            ui.label("行為類型:");
            let behavior_types = self.mapping_engine.get_behavior_types();
            if !behavior_types.is_empty() {
                egui::ComboBox::from_id_source("behavior_combo")
                    .selected_text(behavior_types.get(self.selected_behavior).unwrap_or(&"未選擇".to_string()))
                    .show_ui(ui, |cb| {
                        for (i, behavior) in behavior_types.iter().enumerate() {
                            cb.selectable_value(&mut self.selected_behavior, i, behavior);
                        }
                    });
            }
        });

        ui.add_space(10.0);

        // 數值輸入
        ui.horizontal(|ui| {
            ui.label("數值:");
            ui.add(egui::DragValue::new(&mut self.input_value).speed(0.1));
        });

        ui.horizontal(|ui| {
            ui.label("單位:");
            egui::ComboBox::from_id_source("unit_combo")
                .selected_text(&self.input_unit)
                .show_ui(ui, |cb| {
                    cb.selectable_value(&mut self.input_unit, "次".to_string(), "次");
                    cb.selectable_value(&mut self.input_unit, "秒".to_string(), "秒");
                    cb.selectable_value(&mut self.input_unit, "毫秒".to_string(), "毫秒");
                    cb.selectable_value(&mut self.input_unit, "比率".to_string(), "比率");
                    cb.selectable_value(&mut self.input_unit, "分數".to_string(), "分數");
                });
        });

        ui.add_space(20.0);

        // 計算按鈕
        if ui.add_sized([200.0, 40.0], egui::Button::new("🔍 計算影響分數")).clicked() {
            self.calculate_impact();
        }

        ui.separator();
        
        // 設定選項
        ui.collapsing("⚙️ 顯示設定", |ui| {
            ui.checkbox(&mut self.show_brain_regions, "顯示腦區標籤");
        });
    }

    /// 渲染右側腦圖面板
    fn render_brain_map_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗺️ 腦區影響圖");
        
        ui.separator();
        
        // 腦圖顯示區域
        let available_rect = ui.available_rect_before_wrap();
        let brain_rect = egui::Rect::from_min_size(
            available_rect.min,
            egui::Vec2::new(available_rect.width(), available_rect.height() * 0.6)
        );
        
        ui.allocate_ui_at_rect(brain_rect, |ui| {
            ui.centered_and_justified(|ui| {
                if self.brain_svg_data.contains("<svg") {
                    // 這裡應該實現真正的 SVG 渲染
                    // 目前顯示placeholder
                    let painter = ui.painter();
                    let rect = ui.max_rect();
                    
                    // 繪製腦部輪廓（簡化版）
                    painter.rect_filled(rect, 10.0, egui::Color32::from_gray(240));
                    painter.rect_stroke(rect, 10.0, egui::Stroke::new(2.0, egui::Color32::DARK_GRAY));
                    
                    // 如果有計算結果，顯示影響區域
                    if let Some(result) = &self.current_result {
                        self.render_impact_overlay(&painter, &rect, &result.impact_scores);
                    }
                    
                    ui.label("腦圖 (SVG 渲染開發中...)");
                } else {
                    ui.label("❌ 找不到 brain.svg 文件");
                }
            });
        });

        ui.add_space(10.0);

        // 影響分數詳情
        if let Some(result) = &self.current_result {
            ui.collapsing("📊 詳細影響分數", |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for impact in &result.impact_scores {
                        ui.horizontal(|ui| {
                            let color = self.get_impact_color(impact.impact_score);
                            ui.colored_label(color, "●");
                            ui.label(&impact.region);
                            ui.label(format!("{:.3}", impact.impact_score));
                        });
                    }
                });
            });
        }
    }

    /// 渲染底部結果面板
    fn render_results_panel(&mut self, ui: &mut egui::Ui) {
        if let Some(risk_report) = &self.current_risk_report {
            ui.separator();
            ui.heading("📋 風險評估報告");
            
            // 風險等級顯示
            ui.horizontal(|ui| {
                let (color, icon) = match risk_report.risk_level {
                    RiskLevel::Low => (egui::Color32::from_rgb(102, 204, 255), "🟢"),
                    RiskLevel::Medium => (egui::Color32::from_rgb(255, 170, 0), "🟡"),
                    RiskLevel::High => (egui::Color32::from_rgb(255, 85, 85), "🟠"),
                    RiskLevel::Critical => (egui::Color32::from_rgb(139, 0, 0), "🔴"),
                };
                ui.label(icon);
                ui.colored_label(color, format!("風險等級: {}", risk_report.risk_level.description()));
                ui.label(format!("總影響分數: {:.3}", risk_report.total_impact));
            });

            ui.add_space(10.0);

            // 分欄顯示詳細資訊
            ui.columns(2, |columns| {
                // 左欄：受影響功能
                columns[0].collapsing("⚠️ 可能受影響的功能", |ui| {
                    for function in &risk_report.affected_functions {
                        ui.label(format!("• {}", function));
                    }
                });

                // 右欄：潛在疾病風險
                columns[1].collapsing("🏥 潜在疾病風險", |ui| {
                    for disease in &risk_report.potential_diseases {
                        ui.label(format!("• {}", disease));
                    }
                });
            });

            ui.add_space(10.0);

            // 建議事項
            ui.collapsing("💡 建議事項", |ui| {
                for recommendation in &risk_report.recommendations {
                    ui.label(format!("• {}", recommendation));
                }
            });
        }
    }

    /// 計算影響分數
    fn calculate_impact(&mut self) {
        let behavior_types = self.mapping_engine.get_behavior_types();
        if let Some(behavior_type) = behavior_types.get(self.selected_behavior) {
            let input = BehaviorInput {
                behavior_type: behavior_type.clone(),
                value: self.input_value,
                unit: self.input_unit.clone(),
                timestamp: chrono::Utc::now(),
            };

            if let Some(result) = self.mapping_engine.calculate_impact(&input) {
                let risk_report = self.risk_assessment.generate_risk_report(&result);
                
                self.calculation_history.push(result.clone());
                self.current_result = Some(result);
                self.current_risk_report = Some(risk_report);
            }
        }
    }

    /// 繪製影響疊加層
    fn render_impact_overlay(&self, painter: &egui::Painter, rect: &egui::Rect, impacts: &[RegionImpactScore]) {
        // 簡化版：在腦圖上顯示影響點
        for (i, impact) in impacts.iter().enumerate() {
            if impact.impact_score > 0.1 {
                let color = self.get_impact_color(impact.impact_score);
                let pos = egui::Pos2::new(
                    rect.min.x + (i as f32 + 1.0) * rect.width() / (impacts.len() as f32 + 1.0),
                    rect.center().y + (i as f32 % 2.0 - 0.5) * 30.0,
                );
                let radius = 5.0 + impact.impact_score * 10.0;
                painter.circle_filled(pos, radius, color);
                
                if self.show_brain_regions {
                    painter.text(
                        pos + egui::Vec2::new(0.0, radius + 5.0),
                        egui::Align2::CENTER_TOP,
                        &impact.region,
                        egui::FontId::default(),
                        egui::Color32::BLACK,
                    );
                }
            }
        }
    }

    /// 根據影響分數獲取顏色
    fn get_impact_color(&self, score: f32) -> egui::Color32 {
        let risk_level = RiskLevel::from_score(score);
        let rgb = risk_level.color();
        egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2])
    }
}

impl eframe::App for BehaviorBrainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 主標題
            ui.heading("🧠 行為-腦網路連續量化可視化工具");
            ui.add_space(10.0);

            // 主要內容區域：左右分欄
            ui.horizontal(|ui| {
                // 左側控制面板 (30% 寬度)
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width() * 0.3, ui.available_height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            self.render_control_panel(ui);
                        });
                    }
                );

                ui.separator();

                // 右側腦圖面板 (70% 寬度)
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width(), ui.available_height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        self.render_brain_map_panel(ui);
                    }
                );
            });

            // 底部結果面板
            self.render_results_panel(ui);
        });
    }
}
