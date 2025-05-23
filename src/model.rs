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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_behavior_input_creation() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let behavior = BehaviorInput {
            behavior_type: "反應時間".to_string(),
            value: 250.0,
            unit: "毫秒".to_string(),
            timestamp,
        };

        assert_eq!(behavior.behavior_type, "反應時間");
        assert_eq!(behavior.value, 250.0);
        assert_eq!(behavior.unit, "毫秒");
        assert_eq!(behavior.timestamp, timestamp);
    }

    #[test]
    fn test_brain_region_impact_creation() {
        let impact = BrainRegionImpact {
            region: "前額葉皮質".to_string(),
            weight: 0.7,
            description: Some("執行功能相關".to_string()),
        };

        assert_eq!(impact.region, "前額葉皮質");
        assert_eq!(impact.weight, 0.7);
        assert!(impact.description.is_some());
        assert_eq!(impact.description.unwrap(), "執行功能相關");
    }

    #[test]
    fn test_normalization_params() {
        let params = NormalizationParams {
            mean: 300.0,
            std_dev: 50.0,
            sample_size: Some(1000),
        };

        assert_eq!(params.mean, 300.0);
        assert_eq!(params.std_dev, 50.0);
        assert_eq!(params.sample_size, Some(1000));
    }

    #[test]
    fn test_behavior_brain_map() {
        let brain_regions = vec![
            BrainRegionImpact {
                region: "前額葉皮質".to_string(),
                weight: 0.8,
                description: None,
            },
            BrainRegionImpact {
                region: "頂葉皮質".to_string(),
                weight: 0.6,
                description: None,
            },
        ];

        let map = BehaviorBrainMap {
            behavior: "注意力測試".to_string(),
            brain_regions: brain_regions.clone(),
            normalization_params: None,
        };

        assert_eq!(map.behavior, "注意力測試");
        assert_eq!(map.brain_regions.len(), 2);
        assert_eq!(map.brain_regions[0].region, "前額葉皮質");
        assert_eq!(map.brain_regions[1].weight, 0.6);
    }

    #[test]
    fn test_region_impact_score() {
        let score = RegionImpactScore {
            region: "海馬迴".to_string(),
            impact_score: 0.75,
            normalized_input: 1.2,
            weight: 0.65,
        };

        assert_eq!(score.region, "海馬迴");
        assert_eq!(score.impact_score, 0.75);
        assert_eq!(score.normalized_input, 1.2);
        assert_eq!(score.weight, 0.65);
    }

    #[test]
    fn test_risk_level_from_score() {
        assert_eq!(RiskLevel::from_score(0.1), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(0.4), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(0.7), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(0.9), RiskLevel::Critical);
        
        // 邊界值測試
        assert_eq!(RiskLevel::from_score(0.3), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(0.6), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(0.8), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(0.29), RiskLevel::Low);
    }

    #[test]
    fn test_risk_level_color() {
        assert_eq!(RiskLevel::Low.color(), [102, 204, 255]);
        assert_eq!(RiskLevel::Medium.color(), [255, 170, 0]);
        assert_eq!(RiskLevel::High.color(), [255, 85, 85]);
        assert_eq!(RiskLevel::Critical.color(), [139, 0, 0]);
    }

    #[test]
    fn test_risk_level_description() {
        assert_eq!(RiskLevel::Low.description(), "低風險：目前無明顯異常");
        assert_eq!(RiskLevel::Medium.description(), "中風險：建議持續觀察");
        assert_eq!(RiskLevel::High.description(), "高風險：請留意日常功能與相關疾病徵兆");
        assert_eq!(RiskLevel::Critical.description(), "極高風險：建議諮詢專業醫師");
    }

    #[test]
    fn test_calculation_result() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let behavior_input = BehaviorInput {
            behavior_type: "記憶測試".to_string(),
            value: 85.0,
            unit: "分數".to_string(),
            timestamp,
        };

        let impact_scores = vec![
            RegionImpactScore {
                region: "海馬迴".to_string(),
                impact_score: 0.8,
                normalized_input: 1.1,
                weight: 0.9,
            },
        ];

        let result = CalculationResult {
            behavior_input: behavior_input.clone(),
            impact_scores: impact_scores.clone(),
            total_impact: 0.72,
            risk_level: RiskLevel::High,
        };

        assert_eq!(result.behavior_input.behavior_type, "記憶測試");
        assert_eq!(result.impact_scores.len(), 1);
        assert_eq!(result.total_impact, 0.72);
        assert_eq!(result.risk_level, RiskLevel::High);
    }

    #[test]
    fn test_serialization_deserialization() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let behavior = BehaviorInput {
            behavior_type: "反應時間".to_string(),
            value: 250.0,
            unit: "毫秒".to_string(),
            timestamp,
        };

        // 測試序列化
        let json = serde_json::to_string(&behavior).unwrap();
        assert!(json.contains("反應時間"));
        assert!(json.contains("250"));

        // 測試反序列化
        let deserialized: BehaviorInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.behavior_type, behavior.behavior_type);
        assert_eq!(deserialized.value, behavior.value);
        assert_eq!(deserialized.unit, behavior.unit);
        assert_eq!(deserialized.timestamp, behavior.timestamp);
    }

    #[test]
    fn test_brain_region_impact_serialization() {
        let impact = BrainRegionImpact {
            region: "前額葉皮質".to_string(),
            weight: 0.7,
            description: Some("測試描述".to_string()),
        };

        let json = serde_json::to_string(&impact).unwrap();
        let deserialized: BrainRegionImpact = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.region, impact.region);
        assert_eq!(deserialized.weight, impact.weight);
        assert_eq!(deserialized.description, impact.description);
    }

    #[test]
    fn test_behavior_brain_map_serialization() {
        let brain_regions = vec![
            BrainRegionImpact {
                region: "前額葉皮質".to_string(),
                weight: 0.8,
                description: None,
            },
        ];

        let normalization_params = NormalizationParams {
            mean: 300.0,
            std_dev: 50.0,
            sample_size: Some(1000),
        };

        let map = BehaviorBrainMap {
            behavior: "測試行為".to_string(),
            brain_regions,
            normalization_params: Some(normalization_params),
        };

        let json = serde_json::to_string(&map).unwrap();
        let deserialized: BehaviorBrainMap = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.behavior, map.behavior);
        assert_eq!(deserialized.brain_regions.len(), 1);
        assert!(deserialized.normalization_params.is_some());
        
        let params = deserialized.normalization_params.unwrap();
        assert_eq!(params.mean, 300.0);
        assert_eq!(params.std_dev, 50.0);
        assert_eq!(params.sample_size, Some(1000));
    }

    #[test]
    fn test_risk_level_boundary_conditions() {
        // 測試邊界條件
        assert_eq!(RiskLevel::from_score(0.0), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(1.0), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(-0.1), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(1.5), RiskLevel::Critical);
    }

    #[test]
    fn test_partial_eq_for_risk_level() {
        assert_eq!(RiskLevel::Low, RiskLevel::Low);
        assert_ne!(RiskLevel::Low, RiskLevel::High);
        assert_eq!(RiskLevel::Critical, RiskLevel::Critical);
    }
}
