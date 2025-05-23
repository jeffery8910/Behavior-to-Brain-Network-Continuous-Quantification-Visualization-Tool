use serde::{Deserialize, Serialize};

/// 行為輸入資料結構
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorInput {
    pub behavior_type: String,
    pub value: f32,
    pub unit: String, // 次數、秒、毫秒、比率等
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 腦區影響資料結構
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainRegionImpact {
    pub region: String,
    pub weight: f32,
    pub description: Option<String>,
}

/// 行為-腦區映射表結構
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorBrainMap {
    pub behavior: String,
    pub brain_regions: Vec<BrainRegionImpact>,
    pub normalization_params: Option<NormalizationParams>,
}

/// 正規化參數
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizationParams {
    pub mean: f32,
    pub std_dev: f32,
    pub sample_size: Option<u32>,
}

/// 計算結果結構
#[derive(Debug, Clone)]
pub struct CalculationResult {
    pub behavior_input: BehaviorInput,
    pub impact_scores: Vec<RegionImpactScore>,
    pub total_impact: f32,
    pub risk_level: RiskLevel,
}

/// 腦區影響分數
#[derive(Debug, Clone)]
pub struct RegionImpactScore {
    pub region: String,
    pub impact_score: f32,
    pub normalized_input: f32,
    pub weight: f32,
}

/// 風險等級
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn from_score(score: f32) -> Self {
        if score >= 0.8 {
            RiskLevel::Critical
        } else if score >= 0.6 {
            RiskLevel::High
        } else if score >= 0.3 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }

    pub fn color(&self) -> [u8; 3] {
        match self {
            RiskLevel::Low => [102, 204, 255],      // 淡藍
            RiskLevel::Medium => [255, 170, 0],     // 橙色
            RiskLevel::High => [255, 85, 85],       // 深紅
            RiskLevel::Critical => [139, 0, 0],     // 暗紅
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            RiskLevel::Low => "低風險：目前無明顯異常",
            RiskLevel::Medium => "中風險：建議持續觀察",
            RiskLevel::High => "高風險：請留意日常功能與相關疾病徵兆",
            RiskLevel::Critical => "極高風險：建議諮詢專業醫師",
        }
    }
}
