# 🧠 行為-腦網路連續量化可視化工具  
**Behavior-to-Brain Network Continuous Quantification & Visualization Tool**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/your-name/behavior-brain-viz)

> **TL;DR** 一個用 **Rust + egui** 打造的桌面應用程式：  
> 使用者輸入「行為次數 / 時間區間」等連續量測後，程式即時估算對各腦區功能性網路的衝擊，並以互動式腦圖（2-D SVG）高亮顯示受影響區域，同時提示可能的日常功能風險與疾病警訊。

![Demo Screenshot](docs/demo_screenshot.png)

---

## 🎯 目標 (Goals)

| 中文 | English |
|------|---------|
| 🔬 將最新「行為 × 功能性腦網路」研究轉化為可操作工具 | 🔬 Translate recent "behavior × functional brain network" findings into a hands-on tool |
| 📊 以 **連續變項**（次數、秒、毫秒、比率…）估算特定腦區之**影響值 (impact score)** | 📊 Accept **continuous metrics** (count, sec, ms, ratios …) and compute an **impact score** per brain area |
| 🎨 於 GUI 右側即時渲染 **腦區著色圖**（SVG layer） | 🎨 Render an interactive **brain map** (SVG heat overlay) in real time |
| ⚠️ 根據影響閾值，自動列出可能受損的日常功能與潛在疾病風險 | ⚠️ When impact ≥ threshold, list daily-life functions at risk and possible pathologies |

---

## ✨ 特色 (Features)

### 🚀 即時計算與視覺化
- **即時輸入 / 即時可視化** – 透過 `egui` 的即時繪圖與 GUI 更新
- **模組化映射表** – `data/behavior_brain_map.json`：整合文獻中「行為指標 ⇄ 腦區」的權重
- **智能正規化** – 基於人群常模的 Z-score 標準化

### 🧠 科學基礎
- **多層級腦區分析** – 從皮質到皮層下結構的全面覆蓋
- **文獻驗證權重** – 基於 HCP、OpenNeuro 等大型資料庫的映射係數
- **DSM-5/ICD-11 對照** – 專業疾病風險評估系統

### 🎨 使用者體驗
- **腦圖熱點渲染** – 以顏色深淺表示 <span style="color:#ff5555">高衝擊</span>、<span style="color:#ffaa00">中衝擊</span>、<span style="color:#66ccff">低衝擊</span>
- **分欄式介面** – 左側控制面板 + 右側視覺化 + 底部詳細報告
- **多語言支援** – 中英文介面切換
- **離線執行** – 編譯後僅一個可執行檔，跨平台相容

---

## 🚀 快速開始 (Quick Start)

### 系統需求
- **作業系統**: Windows 10/11, macOS 10.15+, Linux (Ubuntu 18.04+)
- **記憶體**: 最少 512MB RAM
- **磁碟空間**: 50MB

### 安裝步驟

```bash
# 1. 安裝 Rust (如果尚未安裝)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 下載專案
git clone https://github.com/your-name/behavior-brain-viz.git
cd behavior-brain-viz

# 3. 編譯並執行
cargo run --release
```

### 使用流程

1. **啟動應用程式** – 雙擊執行檔或使用 `cargo run`
2. **選擇行為類型** – 左側下拉選單選擇測試項目（如持續注意力、工作記憶等）
3. **輸入測量數值** – 填入行為表現數據（次數/時間/分數）
4. **點擊計算** – 系統即時運算腦區影響分數
5. **查看結果** – 右側腦圖顯示影響熱點，底部顯示風險評估報告

---

## 📁 專案結構 (Project Structure)

```
behavior-brain-viz/
├── 📄 Cargo.toml              # Rust 專案配置
├── 📄 README.md               # 專案說明文件
├── 📄 LICENSE                 # MIT 授權條款
│
├── 🎨 assets/
│   └── brain.svg              # 2-D 大腦輪廓向量圖
│
├── 📊 data/
│   └── behavior_brain_map.json # 行為-腦區映射與權重資料庫
│
├── 🔧 src/
│   ├── main.rs                # 程式進入點與字體設定
│   ├── model.rs               # 資料結構定義
│   ├── mapping.rs             # 影響分數計算引擎
│   ├── risk.rs                # 風險評估與疾病對照
│   └── gui.rs                 # egui 使用者介面
│
├── 📋 docs/                   # 文件與截圖
└── 🎯 target/                 # 編譯輸出目錄
```

---

## 🧮 核心算法 (Core Algorithm)

### 1. 行為數值正規化
```
x_norm = (x - μ_pop) / σ_pop
```
其中 μ_pop 和 σ_pop 來自大型人群資料庫的常模參數

### 2. 腦區影響分數計算
```
S_b,r = w_b,r × |x_norm|
```
w_b,r 為文獻報導的行為-腦區關聯強度（Fisher-z 或 Cohen's d）

### 3. 風險等級判定
- **🟢 低風險** (S < 0.3)：目前無明顯異常
- **🟡 中風險** (0.3 ≤ S < 0.6)：建議持續觀察  
- **🟠 高風險** (0.6 ≤ S < 0.8)：請留意日常功能
- **🔴 極高風險** (S ≥ 0.8)：建議諮詢專業醫師

### 4. 功能-疾病對照
若 S_b,r 超過閾值且該腦區已知與特定功能/疾病相關，系統將顯示：
- **可能受影響功能**（如工作記憶、步態穩定等）
- **潛在病症風險**（如 ADHD、MCI、憂鬱症等）

---

## 📚 科學依據 (Empirical Evidence)

本工具的行為-腦區映射基於以下高品質研究：

| 行為類別 | 主要腦區 | 參考文獻 | 樣本數 |
|---------|---------|----------|--------|
| 持續注意力指敲 (SART) | DMN-TPN 抗相關、FPCN 切換 | Sadaghiani+ 2015, *PNAS* | 200+ |
| 詞對記憶編碼 | 皮質-海馬 θ 相同步 | Phan+ 2024, *Nat. Commun.* | 800+ |
| 自然步行速度 & 變異 | DAN-DMN 抗相關強度 | Zhang+ 2021, *Geriatric Nursing* | 150+ |
| 師生課堂互動 | 額頂網路 α/θ 同步 | Davidesco+ 2023, *Psychol. Sci.* | 120+ |
| 腹腔鏡技能訓練 | 前額-視覺 β 同步 | Omurtag+ 2025, *Sci. Rep.* | 80+ |
| HCP 認知測試 | 多模態功能連結 | Human Connectome Project | 1000+ |
| 工作記憶 N-back | 背外側前額葉、頂內溝 | Owen+ 2005, *Nat. Neurosci.* | 300+ |
| 情緒面孔辨識 | 杏仁核、梭狀回面孔區 | Adolphs 2008, *Nat. Rev. Neurosci.* | 250+ |
| 語言流暢性 | Broca/Wernicke 區域 | Binder+ 2009, *Cereb. Cortex* | 180+ |
| 空間導航 | 海馬體、內嗅皮質 | Ekstrom+ 2003, *Nature* | 120+ |

完整對應表詳見 [`data/behavior_brain_map.json`](data/behavior_brain_map.json)

---

## 🔧 開發指南 (Development Guide)

### 本地開發環境設置

```bash
# 克隆儲存庫
git clone https://github.com/your-name/behavior-brain-viz.git
cd behavior-brain-viz

# 安裝開發依賴
cargo build

# 執行測試
cargo test

# 程式碼格式化
cargo fmt

# 靜態分析
cargo clippy
```

### 新增行為類型

1. 在 `data/behavior_brain_map.json` 中添加新的映射項目
2. 包含行為名稱、相關腦區、權重係數及正規化參數
3. 在 `src/risk.rs` 中更新相關的功能-疾病對照表
4. 提交 Pull Request 並附上文獻 DOI

### 架構擴展

- **新增 3D 腦模型**: 考慮整合 `wgpu` + GLTF 格式
- **批次資料處理**: 實現 CSV 匯入/匯出功能  
- **AI 輔助解釋**: 整合 ChatGPT API 提供結果說明
- **多語言支援**: 擴展至日韓文等其他語言

---

## 🗺️ 專案藍圖 (Roadmap)

### v0.3.0 - 2024 Q4
- [ ] 🎨 3-D 腦模型渲染 (wgpu + GLTF)
- [ ] 📈 歷史趨勢圖表顯示
- [ ] 💾 結果匯出功能 (PDF/CSV)

### v0.4.0 - 2025 Q1  
- [ ] 🔌 插件系統：自訂行為-腦區權重
- [ ] 🤖 AI 助理：ChatGPT API 即時解釋結果
- [ ] 📊 批次資料分析功能

### v1.0.0 - 2025 Q2
- [ ] 🌐 Web 版本 (WASM)
- [ ] 📱 移動端適配
- [ ] 🏥 臨床驗證版本

---

## 🤝 貢獻指南 (Contributing)

我們歡迎各種形式的貢獻！

### 如何貢獻

1. **Fork** 本儲存庫
2. 創建功能分支：`git checkout -b feature/amazing-feature`
3. 提交變更：`git commit -m 'Add amazing feature'`
4. 推送分支：`git push origin feature/amazing-feature`
5. 開啟 **Pull Request**

### 貢獻類型

- 🐛 **Bug 修復**：回報或修復程式錯誤
- ✨ **新功能**：實現新的行為測試項目
- 📚 **文獻更新**：添加新的行為-腦區映射研究
- 🌐 **本地化**：翻譯介面到其他語言
- 📖 **文件改善**：完善使用說明或 API 文件

### 資料貢獻

若有新文獻、不同年齡層常模或疾病風險係數，請：
1. 更新 `behavior_brain_map.json`
2. 在 PR 中附上文獻 DOI 或資料來源
3. 提供樣本數量和統計參數

---

## 📄 授權條款 (License)

- **原始碼**：[MIT License](LICENSE)
- **brain.svg**：基於 Human Connectome Project 公共領域資源
- **研究資料**：請參考個別文獻的授權條款

---

## 🙏 致謝 (Acknowledgements)

感謝以下專案和研究團隊的開源貢獻：

### 技術框架
- [**Rust**](https://www.rust-lang.org/) - 高效能系統程式語言
- [**egui**](https://github.com/emilk/egui) - 即時模式 GUI 框架  
- [**eframe**](https://github.com/emilk/egui/tree/master/crates/eframe) - egui 的原生視窗後端

### 科學資源
- [**Human Connectome Project**](https://www.humanconnectome.org/) - 大規模腦連結體資料
- [**OpenNeuro**](https://openneuro.org/) - 開放神經影像資料平台
- [**StudentLife Dataset**](https://studentlife.cs.dartmouth.edu/) - 行為感測資料集

### 研究社群
感謝全球神經科學與認知心理學研究者的辛勤工作，讓行為-腦網路的科學理解不斷進步。

---

## 📞 聯絡方式 (Contact)

- **Issues**: [GitHub Issues](https://github.com/your-name/behavior-brain-viz/issues)
- **討論**: [GitHub Discussions](https://github.com/your-name/behavior-brain-viz/discussions)  
- **Email**: your.email@example.com

---

<div align="center">

**🧠 Stay curious, quantify wisely, and keep your brain healthy! 🧠**

[![GitHub stars](https://img.shields.io/github/stars/your-name/behavior-brain-viz?style=social)](https://github.com/your-name/behavior-brain-viz/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/your-name/behavior-brain-viz?style=social)](https://github.com/your-name/behavior-brain-viz/network)

</div>
