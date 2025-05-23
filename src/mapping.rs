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

    #[test]
    fn test_impact_calculation() {
        let behavior_maps = vec![
            BehaviorBrainMap {
                behavior: "測試行為".to_string(),
                brain_regions: vec![
                    BrainRegionImpact {
                        region: "前額葉".to_string(),
                        weight: 0.8,
                        description: None,
                    },
                    BrainRegionImpact {
                        region: "頂葉".to_string(),
                        weight: 0.5,
                        description: None,
                    },
                ],
                normalization_params: Some(NormalizationParams {
                    mean: 50.0,
                    std_dev: 10.0,
                    sample_size: Some(100),
                }),
            }
        ];

        let engine = MappingEngine::new(behavior_maps);
        let input = BehaviorInput {
            behavior_type: "測試行為".to_string(),
            value: 60.0,
            unit: "次".to_string(),
            timestamp: chrono::Utc::now(),
        };

        let result = engine.calculate_impact(&input).unwrap();
        assert_eq!(result.impact_scores.len(), 2);
        assert!(result.total_impact >= 0.0 && result.total_impact <= 1.0);
    }
}
