pub fn estimate_risk(score: f32) -> String {
    if score > 80.0 {
        "高風險：請留意日常功能與相關疾病徵兆".to_string()
    } else if score > 50.0 {
        "中風險：建議持續觀察".to_string()
    } else if score > 0.0 {
        "低風險：目前無明顯異常".to_string()
    } else {
        "請輸入有效數值".to_string()
    }
}
