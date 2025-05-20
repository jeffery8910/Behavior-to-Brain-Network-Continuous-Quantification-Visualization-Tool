//! 風險判定模組
//! 本模組會從 data/risk_map.json 載入每個腦區的風險對照資料，
//! 根據輸入的「腦區」與「impact score」，回傳該腦區對應的主要功能、疾病，以及動態風險提示。
//! 擴充規則只需修改 risk_map.json，無須更動程式碼。

use std::fs;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Threshold {
    pub level: String,
    pub min: f32,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct RiskEntry {
    pub functions: Vec<String>,
    pub diseases: Vec<String>,
    pub thresholds: Vec<Threshold>,
}

type RiskMap = HashMap<String, RiskEntry>;

pub struct RiskEngine {
    risk_map: RiskMap,
}

impl RiskEngine {
    /// 從指定的 risk_map.json 檔案建立 RiskEngine
    pub fn new(json_path: &str) -> Self {
        let data = fs::read_to_string(json_path).expect("無法讀取風險資料檔");
        let risk_map: RiskMap = serde_json::from_str(&data).expect("risk_map.json 格式錯誤");
        RiskEngine { risk_map }
    }

    /// 查詢指定腦區與 impact score 的功能、疾病、與風險提示
    pub fn query(&self, region: &str, score: f32) -> Option<(Vec<String>, Vec<String>, String)> {
        self.risk_map.get(region).map(|entry| {
            // threshold 請依 min 倒序排列，找到第一個符合的
            let mut risk_msg = "無風險資料".to_string();
            for th in entry.thresholds.iter().sorted_by(|a, b| b.min.partial_cmp(&a.min).unwrap()) {
                if score >= th.min {
                    risk_msg = th.message.clone();
                    break;
                }
            }
            (entry.functions.clone(), entry.diseases.clone(), risk_msg)
        })
    }
}

// 用法範例
/*
fn main() {
    let engine = RiskEngine::new("data/risk_map.json");
    let (region, score) = ("Prefrontal_Cortex", 83.2);
    if let Some((functions, diseases, msg)) = engine.query(region, score) {
        println!("功能: {:?}", functions);
        println!("疾病: {:?}", diseases);
        println!("提示: {}", msg);
    } else {
        println!("查無該腦區資料");
    }
}
*/

// 請確保在 Cargo.toml 加入 serde, serde_json, itertools
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// itertools = "0.12"