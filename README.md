# 行為-腦網路連續量化可視化工具  
Behavior-to-Brain Network Continuous Quantification & Visualization Tool
================================================================================

> **TL;DR** 一個用 **Rust + egui** 打造的桌面應用程式：  
> 使用者輸入「行為次數 / 時間區間」等連續量測後，程式即時估算對各腦區功能性網路的衝擊，並以互動式腦圖（2-D SVG）高亮顯示受影響區域，同時提示可能的日常功能風險與疾病警訊。  

---

## 目標 (Goals)

| 中文 | English |
|------|---------|
| - 將最新「行為 × 功能性腦網路」研究轉化為可操作工具。<br>- 以 **連續變項**（次數、秒、毫秒、比率…）估算特定腦區之<strong>影響值 (impact score)</strong>。<br>- 於 GUI 右側即時渲染 <strong>腦區著色圖</strong>（SVG layer）。<br>- 根據影響閾值，自動列出可能受損的日常功能與潛在疾病風險。 | - Translate recent “behavior × functional brain network” findings into a hands-on tool.<br>- Accept **continuous metrics** (count, sec, ms, ratios …) and compute an **impact score** per brain area.<br>- Render an interactive **brain map** (SVG heat overlay) in real time.<br>- When impact ≥ threshold, list daily-life functions at risk and possible pathologies. |

---

## 特色 (Features)

1. **即時輸入 / 即時可視化** – 透過 `egui` 的即時繪圖與 GUI 更新。  
2. **模組化映射表**  
   - `data/behavior_brain_map.json`：整合文獻中「行為指標 ⇄ 腦區」的權重。  
   - 方便後續以 YAML/CSV 更新或社群 PR。  
3. **風險提示引擎**  
   - `src/risk.rs` 依《DSM-5》、《ICD-11》與頂尖期刊整理的功能-疾病對照表，動態產生警示。  
4. **腦圖渲染**  
   - 以 `egui_extras::RetainedImage` 載入向量檔 `assets/brain.svg`。  
   - 以 alpha 疊色呈現 <span style="color:#ff5555">高衝擊</span>、<span style="color:#ffaa00">中衝擊</span>、<span style="color:#66ccff">低衝擊</span>。  
5. **離線單檔執行** – 編譯後僅一個可執行檔，跨 Windows / macOS / Linux。  

---

## 快速開始 (Quick Start)

# 1. 安裝 Rust stable
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 下載專案
git clone https://github.com/your-name/behavior-brain-viz.git
cd behavior-brain-viz

# 3. 編譯並執行
cargo run --release

啟動後：

1. 左側「行為輸入區」選擇行為類型 (e.g., 持續注意力指敲、自然步行…)


2. 輸入 次數 / 時長 / 比率


3. 點擊 Calculate


4. 右側「Brain Map」即時著色；下方列出 功能風險 & 疾病警訊




---

專案結構 (Project Layout)

behavior-brain-viz/
├── Cargo.toml
├── assets/
│   └── brain.svg          # 2-D 大腦輪廓向量圖
├── data/
│   └── behavior_brain_map.json  # 行為-腦區映射與權重
└── src/
    ├── main.rs            # egui 視窗框架
    ├── gui.rs             # 左側表單 & 右側腦圖
    ├── model.rs           # 行為輸入資料結構
    ├── mapping.rs         # 影響分數計算邏輯
    └── risk.rs            # 風險/疾病提示規則


---

核心算法 (Core Algorithm)

1. Normalize 行為值



x_{\text{norm}} = \frac{x - \mu_{\text{pop}}}{\sigma_{\text{pop}}}

S_{b,r} = w_{b,r} \times x_{\text{norm}}

w 來自文獻中報導的 Fisher-z 或相干 effect size。


3. 閾值判定

：高衝擊 → 深紅

：中衝擊 → 橙

：低衝擊 → 淡藍



4. 風險對照

若 S_{b,r} 高且該腦區已知與某功能/疾病相關，於側欄呈現：

可能受影響功能（如工作記憶、步態穩定…）

潛在病症（如 ADHD、MCI、抑鬱症…）






---

依據文獻 (Empirical Mapping Sources)

行為類別	主要腦區	參考文獻

持續注意力指敲 (SART)	DMN-TPN 抗相關、FPCN 切換	Sadaghiani+ 2015, PNAS
詞對記憶編碼	皮質-海馬 θ 相同步	Phan+ 2024, Nat. Commun.
自然步行速度 & 變異	DAN-DMN 抗相關強度	Zhang+ 2021, Geriatric Nursing
師生課堂互動	額頂網路 α/θ 同步	Davidesco+ 2023, Psychol. Sci.
腹腔鏡技能訓練	前額-視覺 β 同步	Omurtag+ 2025, Sci. Rep.


完整對應表詳見 data/behavior_brain_map.json。


---

Roadmap

[ ] 3-D 腦模型（wgpu + GLTF）

[ ] CSV 批次匯入／匯出

[ ] 插件：自訂行為-腦區權重

[ ] AI 助理：ChatGPT API 即時解釋結果



---

貢獻 (Contributing)

歡迎 PR / Issue！
若有新文獻、不同年齡層常模或疾病風險係數，請更新 behavior_brain_map.json 並在 PR 中附上 DOI。


---

授權 (License)

原始碼：MIT License

brain.svg：出自 Human Connectome Project 公共領域資源



---

致謝 (Acknowledgements)

感謝以下公開資料與工具：Rust、egui、eframe、serde_json、egui_extras, Human Connectome Project, OpenNeuro, StudentLife dataset, 以及眾多行為-腦網路研究者。

> Stay curious, quantify wisely, and keep your brain healthy!





