use std::fs;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Threshold {
    level: String,
    min: f32,
    message: String,
}

#[derive(Deserialize)]
struct RiskEntry {
    functions: Vec<String>,
    diseases: Vec<String>,
    thresholds: Vec<Threshold>,
}

type RiskMap = HashMap<String, RiskEntry>;

pub struct RiskEngine {
    risk_map: RiskMap,
}

impl RiskEngine {
    pub fn new(json_path: &str) -> Self {
        let data = fs::read_to_string(json_path).expect("讀取風險資料檔失敗");
        let risk_map: RiskMap = serde_json::from_str(&data).expect("解析 JSON 失敗");
        RiskEngine { risk_map }
    }

    pub fn query(&self, region: &str, score: f32) -> Option<(Vec<String>, Vec<String>, String)> {
        self.risk_map.get(region).map(|entry| {
            let mut risk_msg = "無資料".to_string();
            for th in &entry.thresholds {
                if score >= th.min {
                    risk_msg = th.message.clone();
                    break;
                }
            }
            (entry.functions.clone(), entry.diseases.clone(), risk_msg)
        })
    }
}