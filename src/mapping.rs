use crate::model::{BehaviorBrainMap, BehaviorInput, CalculationResult, RegionImpactScore, RiskLevel};
use std::collections::HashMap;

/// 影響分數計算引擎
pub struct MappingEngine {
    behavior_maps: Vec<BehaviorBrainMap>,
    region_cache: HashMap<String, Vec<usize>>, // 腦區到行為映射的快取
}

impl MappingEngine {
    pub fn new(behavior_maps: Vec<BehaviorBrainMap>) -> Self {
        let mut region_cache = HashMap::new();
        
        // 建立腦區索引快取
        for (idx, map) in behavior_maps.iter().enumerate() {
            for region_impact in &map.brain_regions {
                region_cache
                    .entry(region_impact.region.clone())
                    .or_insert_with(Vec::new)
                    .push(idx);
            }
        }

        Self {
            behavior_maps,
            region_cache,
        }
    }

    /// 計算行為對各腦區的影響分數
    pub fn calculate_impact(&self, input: &BehaviorInput) -> Option<CalculationResult> {
        // 找到對應的行為映射
        let behavior_map = self.behavior_maps
            .iter()
            .find(|map| map.behavior == input.behavior_type)?;

        let mut impact_scores = Vec::new();
        let mut total_impact = 0.0;

        // 正規化輸入值
        let normalized_input = if let Some(norm_params) = &behavior_map.normalization_params {
            (input.value - norm_params.mean) / norm_params.std_dev
        } else {
            input.value / 100.0 // 預設正規化
        };

        // 計算每個腦區的影響分數
        for region_impact in &behavior_map.brain_regions {
            let impact_score = region_impact.weight * normalized_input.abs();
            total_impact += impact_score;

            impact_scores.push(RegionImpactScore {
                region: region_impact.region.clone(),
                impact_score,
                normalized_input,
                weight: region_impact.weight,
            });
        }

        // 正規化總影響分數到 0-1 範圍
        total_impact = (total_impact / impact_scores.len() as f32).min(1.0).max(0.0);

        let risk_level = RiskLevel::from_score(total_impact);

        Some(CalculationResult {
            behavior_input: input.clone(),
            impact_scores,
            total_impact,
            risk_level,
        })
    }

    /// 獲取所有可用的行為類型
    pub fn get_behavior_types(&self) -> Vec<String> {
        self.behavior_maps.iter().map(|map| map.behavior.clone()).collect()
    }

    /// 獲取特定腦區相關的所有行為
    pub fn get_behaviors_for_region(&self, region: &str) -> Vec<&str> {
        if let Some(indices) = self.region_cache.get(region) {
            indices.iter()
                .map(|&idx| self.behavior_maps[idx].behavior.as_str())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 獲取所有腦區列表
    pub fn get_brain_regions(&self) -> Vec<String> {
        self.region_cache.keys().cloned().collect()
    }
}

/// 從 JSON 文件載入行為-腦區映射資料
pub fn load_behavior_brain_map(path: &str) -> Result<Vec<BehaviorBrainMap>, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    let maps: Vec<BehaviorBrainMap> = serde_json::from_str(&data)?;
    Ok(maps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    use chrono::Utc;

    fn create_test_behavior_maps() -> Vec<BehaviorBrainMap> {
        vec![
            BehaviorBrainMap {
                behavior: "反應時間".to_string(),
                brain_regions: vec![
                    BrainRegionImpact {
                        region: "前額葉皮質".to_string(),
                        weight: 0.8,
                        description: Some("執行功能相關".to_string()),
                    },
                    BrainRegionImpact {
                        region: "頂葉皮質".to_string(),
                        weight: 0.6,
                        description: Some("注意力處理".to_string()),
                    },
                ],
                normalization_params: Some(NormalizationParams {
                    mean: 300.0,
                    std_dev: 50.0,
                    sample_size: Some(1000),
                }),
            },
            BehaviorBrainMap {
                behavior: "記憶測試".to_string(),
                brain_regions: vec![
                    BrainRegionImpact {
                        region: "海馬迴".to_string(),
                        weight: 0.9,
                        description: Some("記憶形成".to_string()),
                    },
                    BrainRegionImpact {
                        region: "前額葉皮質".to_string(),
                        weight: 0.7,
                        description: Some("工作記憶".to_string()),
                    },
                ],
                normalization_params: None,
            },
        ]
    }

    #[test]
    fn test_mapping_engine_creation() {
        let behavior_maps = create_test_behavior_maps();
        let engine = MappingEngine::new(behavior_maps.clone());
        
        assert_eq!(engine.behavior_maps.len(), 2);
        assert!(engine.region_cache.contains_key("前額葉皮質"));
        assert!(engine.region_cache.contains_key("頂葉皮質"));
        assert!(engine.region_cache.contains_key("海馬迴"));
        
        // 測試前額葉皮質被兩個行為引用
        assert_eq!(engine.region_cache.get("前額葉皮質").unwrap().len(), 2);
    }

    #[test]
    fn test_impact_calculation() {
        let behavior_maps = create_test_behavior_maps();
        let engine = MappingEngine::new(behavior_maps);
        
        let input = BehaviorInput {
            behavior_type: "反應時間".to_string(),
            value: 350.0, // 高於平均值
            unit: "毫秒".to_string(),
            timestamp: Utc::now(),
        };

        let result = engine.calculate_impact(&input).unwrap();
        
        assert_eq!(result.impact_scores.len(), 2);
        assert_eq!(result.behavior_input.behavior_type, "反應時間");
        assert!(result.total_impact >= 0.0 && result.total_impact <= 1.0);
        
        // 檢查正規化輸入值
        let expected_normalized = (350.0 - 300.0) / 50.0; // (value - mean) / std_dev
        assert!((result.impact_scores[0].normalized_input - expected_normalized).abs() < 0.001);
    }

    #[test]
    fn test_impact_calculation_without_normalization() {
        let behavior_maps = create_test_behavior_maps();
        let engine = MappingEngine::new(behavior_maps);
        
        let input = BehaviorInput {
            behavior_type: "記憶測試".to_string(),
            value: 80.0,
            unit: "分數".to_string(),
            timestamp: Utc::now(),
        };

        let result = engine.calculate_impact(&input).unwrap();
        
        assert_eq!(result.impact_scores.len(), 2);
        // 沒有正規化參數，應該使用預設正規化 (value / 100.0)
        let expected_normalized = 80.0 / 100.0;
        assert!((result.impact_scores[0].normalized_input - expected_normalized).abs() < 0.001);
    }

    #[test]
    fn test_nonexistent_behavior() {
        let behavior_maps = create_test_behavior_maps();
        let engine = MappingEngine::new(behavior_maps);
        
        let input = BehaviorInput {
            behavior_type: "不存在的行為".to_string(),
            value: 50.0,
            unit: "分數".to_string(),
            timestamp: Utc::now(),
        };

        let result = engine.calculate_impact(&input);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_behavior_types() {
        let behavior_maps = create_test_behavior_maps();
        let engine = MappingEngine::new(behavior_maps);
        
        let behavior_types = engine.get_behavior_types();
        assert_eq!(behavior_types.len(), 2);
        assert!(behavior_types.contains(&"反應時間".to_string()));
        assert!(behavior_types.contains(&"記憶測試".to_string()));
    }

    #[test]
    fn test_get_behaviors_for_region() {
        let behavior_maps = create_test_behavior_maps();
        let engine = MappingEngine::new(behavior_maps);
        
        let behaviors = engine.get_behaviors_for_region("前額葉皮質");
        assert_eq!(behaviors.len(), 2);
        assert!(behaviors.contains(&"反應時間"));
        assert!(behaviors.contains(&"記憶測試"));
        
        let behaviors = engine.get_behaviors_for_region("海馬迴");
        assert_eq!(behaviors.len(), 1);
        assert!(behaviors.contains(&"記憶測試"));
        
        let behaviors = engine.get_behaviors_for_region("不存在的腦區");
        assert_eq!(behaviors.len(), 0);
    }

    #[test]
    fn test_get_brain_regions() {
        let behavior_maps = create_test_behavior_maps();
        let engine = MappingEngine::new(behavior_maps);
        
        let regions = engine.get_brain_regions();
        assert_eq!(regions.len(), 3);
        assert!(regions.contains(&"前額葉皮質".to_string()));
        assert!(regions.contains(&"頂葉皮質".to_string()));
        assert!(regions.contains(&"海馬迴".to_string()));
    }    #[test]
    fn test_risk_level_calculation() {
        let behavior_maps = vec![
            BehaviorBrainMap {
                behavior: "風險測試".to_string(),
                brain_regions: vec![
                    BrainRegionImpact {
                        region: "測試腦區".to_string(),
                        weight: 1.0,
                        description: None,
                    },
                ],
                normalization_params: None,
            }
        ];
        
        let engine = MappingEngine::new(behavior_maps);
        
        // 測試低風險 (10.0 / 100.0 = 0.1)
        let low_input = BehaviorInput {
            behavior_type: "風險測試".to_string(),
            value: 10.0,
            unit: "分數".to_string(),
            timestamp: Utc::now(),
        };
        let result = engine.calculate_impact(&low_input).unwrap();
        assert_eq!(result.risk_level, RiskLevel::Low);
        
        // 測試中風險 (40.0 / 100.0 = 0.4)
        let medium_input = BehaviorInput {
            behavior_type: "風險測試".to_string(),
            value: 40.0,
            unit: "分數".to_string(),
            timestamp: Utc::now(),
        };
        let result = engine.calculate_impact(&medium_input).unwrap();
        assert_eq!(result.risk_level, RiskLevel::Medium);
        
        // 測試高風險 (70.0 / 100.0 = 0.7)
        let high_input = BehaviorInput {
            behavior_type: "風險測試".to_string(),
            value: 70.0,
            unit: "分數".to_string(),
            timestamp: Utc::now(),
        };
        let result = engine.calculate_impact(&high_input).unwrap();
        assert_eq!(result.risk_level, RiskLevel::High);
        
        // 測試極高風險 (90.0 / 100.0 = 0.9)
        let critical_input = BehaviorInput {
            behavior_type: "風險測試".to_string(),
            value: 90.0,
            unit: "分數".to_string(),
            timestamp: Utc::now(),
        };
        let result = engine.calculate_impact(&critical_input).unwrap();
        assert_eq!(result.risk_level, RiskLevel::Critical);
    }

    #[test]
    fn test_negative_values() {
        let behavior_maps = vec![
            BehaviorBrainMap {
                behavior: "負值測試".to_string(),
                brain_regions: vec![
                    BrainRegionImpact {
                        region: "測試腦區".to_string(),
                        weight: 0.5,
                        description: None,
                    },
                ],
                normalization_params: Some(NormalizationParams {
                    mean: 100.0,
                    std_dev: 20.0,
                    sample_size: Some(100),
                }),
            }
        ];
        
        let engine = MappingEngine::new(behavior_maps);
        
        let input = BehaviorInput {
            behavior_type: "負值測試".to_string(),
            value: 60.0, // 會產生負的正規化值
            unit: "分數".to_string(),
            timestamp: Utc::now(),
        };
        
        let result = engine.calculate_impact(&input).unwrap();
        
        // 負值應該被取絕對值處理
        assert!(result.impact_scores[0].impact_score >= 0.0);
        assert!(result.total_impact >= 0.0);
    }

    #[test]
    fn test_empty_behavior_maps() {
        let engine = MappingEngine::new(vec![]);
        
        let input = BehaviorInput {
            behavior_type: "任何行為".to_string(),
            value: 50.0,
            unit: "分數".to_string(),
            timestamp: Utc::now(),
        };
        
        let result = engine.calculate_impact(&input);
        assert!(result.is_none());
        
        assert_eq!(engine.get_behavior_types().len(), 0);
        assert_eq!(engine.get_brain_regions().len(), 0);
    }

    #[test]
    fn test_total_impact_normalization() {
        let behavior_maps = vec![
            BehaviorBrainMap {
                behavior: "多腦區測試".to_string(),
                brain_regions: vec![
                    BrainRegionImpact {
                        region: "腦區1".to_string(),
                        weight: 0.9,
                        description: None,
                    },
                    BrainRegionImpact {
                        region: "腦區2".to_string(),
                        weight: 0.8,
                        description: None,
                    },
                    BrainRegionImpact {
                        region: "腦區3".to_string(),
                        weight: 0.7,
                        description: None,
                    },
                ],
                normalization_params: None,
            }
        ];
        
        let engine = MappingEngine::new(behavior_maps);
        
        let input = BehaviorInput {
            behavior_type: "多腦區測試".to_string(),
            value: 100.0, // 最大值
            unit: "分數".to_string(),
            timestamp: Utc::now(),
        };
        
        let result = engine.calculate_impact(&input).unwrap();
        
        // 總影響分數應該在 0-1 範圍內
        assert!(result.total_impact >= 0.0 && result.total_impact <= 1.0);
        assert_eq!(result.impact_scores.len(), 3);
    }

    // 創建一個模擬的 JSON 測試
    #[test]
    fn test_json_structure_compatibility() {
        let json_data = r#"[
            {
                "behavior": "測試行為",
                "brain_regions": [
                    {
                        "region": "前額葉皮質",
                        "weight": 0.8,
                        "description": "執行功能相關"
                    }
                ],
                "normalization_params": {
                    "mean": 300.0,
                    "std_dev": 50.0,
                    "sample_size": 1000
                }
            }
        ]"#;
        
        let maps: Result<Vec<BehaviorBrainMap>, _> = serde_json::from_str(json_data);
        assert!(maps.is_ok());
        
        let maps = maps.unwrap();
        assert_eq!(maps.len(), 1);
        assert_eq!(maps[0].behavior, "測試行為");
        assert_eq!(maps[0].brain_regions.len(), 1);
        assert!(maps[0].normalization_params.is_some());
    }
}
