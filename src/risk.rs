use crate::model::{CalculationResult, RiskLevel, RegionImpactScore};
use std::collections::HashMap;

/// 風險評估和功能分析引擎
pub struct RiskAssessment {
    region_functions: HashMap<String, Vec<String>>,
    region_diseases: HashMap<String, Vec<String>>,
}

impl Default for RiskAssessment {
    fn default() -> Self {
        let mut region_functions = HashMap::new();
        let mut region_diseases = HashMap::new();

        // 前額葉相關功能和疾病
        region_functions.insert("前額葉".to_string(), vec![
            "執行功能".to_string(),
            "工作記憶".to_string(),
            "注意力控制".to_string(),
            "決策能力".to_string(),
            "情緒調節".to_string(),
        ]);
        region_diseases.insert("前額葉".to_string(), vec![
            "注意力缺陷過動症 (ADHD)".to_string(),
            "憂鬱症".to_string(),
            "焦慮症".to_string(),
            "額顳葉型失智症".to_string(),
        ]);

        // 頂葉相關功能和疾病
        region_functions.insert("頂葉".to_string(), vec![
            "空間認知".to_string(),
            "身體感覺整合".to_string(),
            "注意力分配".to_string(),
            "視覺空間處理".to_string(),
        ]);
        region_diseases.insert("頂葉".to_string(), vec![
            "阿茲海默症".to_string(),
            "失用症".to_string(),
            "視覺忽略症".to_string(),
        ]);

        // 小腦相關功能和疾病
        region_functions.insert("小腦".to_string(), vec![
            "運動協調".to_string(),
            "平衡控制".to_string(),
            "步態穩定".to_string(),
            "精細動作控制".to_string(),
        ]);
        region_diseases.insert("小腦".to_string(), vec![
            "小腦萎縮症".to_string(),
            "帕金森氏症".to_string(),
            "運動失調症".to_string(),
        ]);

        // 運動皮質相關功能和疾病
        region_functions.insert("運動皮質".to_string(), vec![
            "隨意運動控制".to_string(),
            "動作規劃".to_string(),
            "肌肉協調".to_string(),
        ]);
        region_diseases.insert("運動皮質".to_string(), vec![
            "中風".to_string(),
            "肌萎縮性側索硬化症 (ALS)".to_string(),
            "運動皮質病變".to_string(),
        ]);

        // HCP 相關腦區
        region_functions.insert("Middle Frontal Gyrus".to_string(), vec![
            "Executive Control".to_string(),
            "Working Memory".to_string(),
            "Cognitive Flexibility".to_string(),
        ]);
        region_diseases.insert("Middle Frontal Gyrus".to_string(), vec![
            "ADHD".to_string(),
            "Schizophrenia".to_string(),
            "Frontotemporal Dementia".to_string(),
        ]);

        region_functions.insert("Anterior Cingulate Cortex (ACC)".to_string(), vec![
            "Conflict Monitoring".to_string(),
            "Emotion Regulation".to_string(),
            "Error Detection".to_string(),
        ]);
        region_diseases.insert("Anterior Cingulate Cortex (ACC)".to_string(), vec![
            "Depression".to_string(),
            "Anxiety Disorders".to_string(),
            "OCD".to_string(),
        ]);

        region_functions.insert("Hippocampus".to_string(), vec![
            "Episodic Memory".to_string(),
            "Spatial Navigation".to_string(),
            "Memory Consolidation".to_string(),
        ]);
        region_diseases.insert("Hippocampus".to_string(), vec![
            "Alzheimer's Disease".to_string(),
            "Mild Cognitive Impairment".to_string(),
            "PTSD".to_string(),
        ]);

        Self {
            region_functions,
            region_diseases,
        }
    }
}

impl RiskAssessment {
    /// 基於計算結果生成風險評估報告
    pub fn generate_risk_report(&self, result: &CalculationResult) -> RiskReport {
        let mut affected_functions = Vec::new();
        let mut potential_diseases = Vec::new();
        let mut high_impact_regions = Vec::new();

        // 分析高影響的腦區
        for impact in &result.impact_scores {
            if impact.impact_score > 0.5 {
                high_impact_regions.push(impact.region.clone());

                // 收集相關的功能
                if let Some(functions) = self.region_functions.get(&impact.region) {
                    affected_functions.extend(functions.clone());
                }

                // 收集相關的疾病
                if let Some(diseases) = self.region_diseases.get(&impact.region) {
                    potential_diseases.extend(diseases.clone());
                }
            }
        }

        // 去重
        affected_functions.sort();
        affected_functions.dedup();
        potential_diseases.sort();
        potential_diseases.dedup();

        RiskReport {
            risk_level: result.risk_level.clone(),
            total_impact: result.total_impact,
            high_impact_regions,
            affected_functions,
            potential_diseases,
            recommendations: self.generate_recommendations(&result.risk_level),
        }
    }

    fn generate_recommendations(&self, risk_level: &RiskLevel) -> Vec<String> {
        match risk_level {
            RiskLevel::Low => vec![
                "持續保持良好的生活習慣".to_string(),
                "定期進行腦力訓練活動".to_string(),
                "維持規律的運動習慣".to_string(),
            ],
            RiskLevel::Medium => vec![
                "建議增加認知訓練活動".to_string(),
                "注意睡眠品質和作息規律".to_string(),
                "考慮減少壓力來源".to_string(),
                "定期追蹤相關指標".to_string(),
            ],
            RiskLevel::High => vec![
                "建議諮詢神經科或精神科醫師".to_string(),
                "進行更詳細的神經心理學評估".to_string(),
                "考慮認知復健訓練".to_string(),
                "密切監控症狀變化".to_string(),
            ],
            RiskLevel::Critical => vec![
                "立即諮詢專業醫療人員".to_string(),
                "安排完整的神經學檢查".to_string(),
                "考慮影像學檢查 (MRI/fMRI)".to_string(),
                "制定個人化治療計畫".to_string(),
            ],
        }
    }
}

/// 風險評估報告
#[derive(Debug, Clone)]
pub struct RiskReport {
    pub risk_level: RiskLevel,
    pub total_impact: f32,
    pub high_impact_regions: Vec<String>,
    pub affected_functions: Vec<String>,
    pub potential_diseases: Vec<String>,
    pub recommendations: Vec<String>,
}

/// 簡化的風險估算函數（向後相容）
pub fn estimate_risk(score: f32) -> String {
    let risk_level = RiskLevel::from_score(score / 100.0);
    risk_level.description().to_string()
}
