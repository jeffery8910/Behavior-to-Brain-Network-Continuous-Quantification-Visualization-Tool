# 開發者指南 (Developer Guide)

## 專案架構

本專案採用模組化設計，主要分為以下幾個模組：

### 核心模組

#### `model.rs` - 資料結構定義
- `BehaviorInput`: 行為輸入資料結構
- `BrainRegionImpact`: 腦區影響映射
- `CalculationResult`: 計算結果
- `RiskLevel`: 風險等級枚舉

#### `mapping.rs` - 計算引擎
- `MappingEngine`: 影響分數計算的核心引擎
- 負責載入映射表並執行分數計算
- 實現正規化和權重計算

#### `risk.rs` - 風險評估
- `RiskAssessment`: 風險評估引擎
- 包含腦區功能和疾病對照表
- 生成個人化建議

#### `gui.rs` - 使用者介面
- `BehaviorBrainApp`: 主應用程式結構
- 實現 egui 介面渲染
- 處理使用者互動

### 資料結構

```rust
// 行為輸入
pub struct BehaviorInput {
    pub behavior_type: String,
    pub value: f32,
    pub unit: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// 計算結果
pub struct CalculationResult {
    pub behavior_input: BehaviorInput,
    pub impact_scores: Vec<RegionImpactScore>,
    pub total_impact: f32,
    pub risk_level: RiskLevel,
}
```

## 算法實現

### 1. 正規化演算法

```rust
fn normalize_input(value: f32, mean: f32, std_dev: f32) -> f32 {
    (value - mean) / std_dev
}
```

### 2. 影響分數計算

```rust
fn calculate_impact(normalized_input: f32, weight: f32) -> f32 {
    weight * normalized_input.abs()
}
```

### 3. 風險等級判定

```rust
impl RiskLevel {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s >= 0.8 => RiskLevel::Critical,
            s if s >= 0.6 => RiskLevel::High,
            s if s >= 0.3 => RiskLevel::Medium,
            _ => RiskLevel::Low,
        }
    }
}
```

## 新增功能流程

### 1. 新增行為類型

1. 在 `data/behavior_brain_map.json` 中添加新項目：
```json
{
  "behavior": "新行為類型",
  "brain_regions": [
    {
      "region": "相關腦區",
      "weight": 0.8,
      "description": "功能描述"
    }
  ],
  "normalization_params": {
    "mean": 50.0,
    "std_dev": 15.0,
    "sample_size": 100
  }
}
```

2. 在 `src/risk.rs` 中更新功能-疾病對照表
3. 測試新功能
4. 更新文檔

### 2. 新增腦區映射

1. 查閱相關文獻獲取權重係數
2. 更新 JSON 資料庫
3. 添加功能描述和疾病關聯
4. 驗證計算結果

## 測試指南

### 單元測試

```bash
cargo test
```

### 整合測試

```bash
cargo test --test integration_tests
```

### 效能測試

```bash
cargo bench
```

## 部署指南

### 發布版本編譯

```bash
cargo build --release
```

### 跨平台編譯

```bash
# Windows
cargo build --target x86_64-pc-windows-gnu

# macOS
cargo build --target x86_64-apple-darwin

# Linux
cargo build --target x86_64-unknown-linux-gnu
```

## 程式碼風格

我們遵循 Rust 標準風格指南：

```bash
# 格式化程式碼
cargo fmt

# 靜態分析
cargo clippy
```

## 效能優化

### 記憶體使用
- 使用 `Vec` 而非 `HashMap` 進行小規模資料存儲
- 實現資料結構的 `Clone` trait 以減少不必要的記憶體分配

### 計算效率
- 快取映射表索引以加速查詢
- 使用並行處理處理大批量計算

### GUI 效能
- 使用 `retain_last` 快取渲染結果
- 實現懶載入以減少初始化時間

## 除錯技巧

### 日誌記錄

```rust
use log::{info, warn, error};

info!("計算完成，影響分數: {}", total_impact);
warn!("找不到對應的行為映射");
error!("載入資料檔案失敗: {}", e);
```

### 效能分析

```bash
# 使用 valgrind 分析記憶體使用
valgrind --tool=memcheck ./target/release/behavior_brain_viz

# 使用 perf 分析 CPU 使用
perf record ./target/release/behavior_brain_viz
perf report
```

## 常見問題

### Q: 如何添加新的風險評估規則？
A: 在 `src/risk.rs` 的 `RiskAssessment::default()` 中添加新的腦區-功能映射。

### Q: 如何調整影響分數的權重？
A: 修改 `data/behavior_brain_map.json` 中的 `weight` 值。

### Q: 如何實現新的可視化效果？
A: 在 `src/gui.rs` 的渲染函數中添加新的繪圖邏輯。
