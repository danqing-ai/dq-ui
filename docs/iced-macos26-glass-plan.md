# Iced macOS 26 Liquid Glass 像素级复刻方案

> 目标：在 Iced 0.14 上实现 **≥ Web `dq-mac.css` + `dq-glass.css`** 的视觉质量，并在 macOS 26 (Tahoe) 上通过 **原生 Liquid Glass** 超越 Web（真实折射/采样，而非 CSS 近似）。

---

## 1. 现状与差距

| 维度 | Web UI（已实现） | Iced UI（当前） |
|------|------------------|-----------------|
| mac 主题 token | `dq-mac.css` 全量 ~120 个 `--dq-*` | **无** `mac_tahoe()` preset |
| 玻璃材质 | `backdrop-filter: saturate(180%) blur(20px)` + 半透明 tint | 仅 `bg_translucent` 单色，**无 blur** |
| 玻璃组件 | `.dq-glass--bar/sidebar/popover/...` 7 种 surface | `sidebar_container` 等用 **实色** `BG_BASE` |
| 阴影 | 多层 `box-shadow`（hairline + ambient） | 单层 `Shadow { blur_radius: 4~16 }` |
| 圆角 | mac `12/10/8/6px` | Linear/China  metrics，**非 mac** |
| 窗口 chrome | Tauri `dq-tauri-macos.css` 透明标题栏 | Iced 示例无 overlay titlebar |
| 无障碍 | `prefers-reduced-transparency` fallback | 未实现 |
| 非激活窗口 | `.is-inactive` desaturate | 未实现 |

**结论**：Linear / China Red 已完成语义色对齐；**macOS 26 玻璃路线尚未启动**。

---

## 2. 设计原则（对齐 Apple HIG + 超越 Web）

参考 [WWDC25 — Build an AppKit app with the new design](https://developer.apple.com/videos/play/wwdc2025/310/)：

1. **玻璃只用于浮层 chrome** — sidebar、toolbar、popover、sheet；内容区保持实底，避免「全 UI 都是毛玻璃」。
2. **移除 legacy NSVisualEffectView** — Tahoe 上 sidebar 玻璃由系统提供；App 内嵌 glass 用 **NSGlassEffectView**（macOS 26+）。
3. **邻近 glass 合并** — 用 **NSGlassEffectContainerView** 控制 spacing，避免 Web 上各自独立 blur 的「双层雾感」。
4. **Legibility layer** — 玻璃上的文字/图标需系统 legibility treatment；Web 靠固定 opacity，Native 可动态适应背景。

**超越 Web 的切入点**：

| Web 局限 | Iced 可做得更好的地方 |
|----------|------------------------|
| `backdrop-filter` 无真正折射，只是模糊+ tint | macOS 26 **NSGlassEffectView** 系统级 Liquid Glass |
| 全窗口统一 blur radius | 按 surface 类型分级（bar 12px / popover 24px）+ 容器 merge |
| 无法感知窗口 inactive | 原生 `NSWindow.isKeyWindow` → palette + saturation 降级 |
| 无 specular highlight | shader / CALayer 顶部 1px highlight（`--dq-glass-highlight`） |
| Tauri 透明窗 + CSS 叠层有合成 bug | 单进程 GPU 管线，blur 采样一致 |

---

## 3. 架构总览

```
┌─────────────────────────────────────────────────────────────┐
│  App (studio-create / teams / mail)                         │
├─────────────────────────────────────────────────────────────┤
│  dq-layout          dq-components                           │
│  (glass sidebar,    (glass modal, popover, inspector)       │
│   floating topbar)                                          │
├─────────────────────────────────────────────────────────────┤
│  dq-glass  ← NEW                                            │
│  · GlassSurface enum + container styles                     │
│  · BlurBackend trait (Native / Shader / Solid fallback)     │
│  · macOS: NSGlassEffectView bridge (objc2-app-kit)          │
│  · cross-platform: wgpu Kawase blur + tint composite        │
├─────────────────────────────────────────────────────────────┤
│  dq-theme                                                   │
│  · mac_tahoe_theme()                                        │
│  · glass_bar_container / glass_sidebar / glass_popover ...    │
├─────────────────────────────────────────────────────────────┤
│  dq-tokens                                                  │
│  · SemanticPalette::mac_tahoe()  ← from dq-mac.css          │
│  · GlassTokens { tint, blur_radius, saturation, highlight }│
│  · ThemeMetrics::mac_tahoe()                                │
└─────────────────────────────────────────────────────────────┘
         ▲                              ▲
         │ source of truth (colors)    │ reference (behavior)
         │                              │
  packages/tokens/src/dq-mac.css   Apple NSGlassEffectView API
  packages/tokens/src/dq-glass.css
```

---

## 4. Token 规格（Web → Rust 映射）

### 4.1 新增 `SemanticPalette::mac_tahoe()`

直接从 `dq-mac.css` 导入（与 Linear/China 相同工作流）：

| Web CSS | Rust 字段 | 值 |
|---------|-----------|-----|
| `--dq-material-chrome` | `bg_base` | `#1c1c1e` |
| `--dq-bg-page` | `bg_page` | `#000000` |
| `--dq-bg-elevated` | `bg_panel` | `#2c2c2e` |
| `--dq-accent` | `accent` | `#0a84ff` |
| `--dq-label-primary` | `text_primary` | `rgba(255,255,255,0.92)` |
| `--dq-separator` | `separator` | `rgba(84,84,88,0.45)` |
| … | … | 全量映射见 `scripts/sync_tokens_from_css.py`（待建） |

### 4.2 新增 `GlassTokens` 结构体

Web 玻璃变量在 Rust 中独立建模（Iced 需要数值而非 CSS 字符串）：

```rust
pub struct GlassTokens {
    pub tint_bar: Color,           // --dq-glass-bar-bg
    pub tint_sidebar: Color,       // --dq-glass-sidebar-bg
    pub tint_popover: Color,       // --dq-glass-popover-bg
    pub tint_panel: Color,         // --dq-glass-panel-bg
    pub tint_control: Color,       // --dq-glass-control-bg (fallback solid)
    pub border: Color,             // --dq-glass-border
    pub border_strong: Color,      // --dq-glass-border-strong
    pub highlight: Color,          // --dq-glass-highlight (top edge specular)
    pub blur_standard: f32,        // 20.0  ← --dq-glass-blur
    pub blur_light: f32,           // 12.0
    pub blur_heavy: f32,           // 24.0
    pub saturation: f32,           // 1.8   ← saturate(180%)
    pub shadow_glass: ShadowSpec,  // --dq-shadow-glass 解析
}
```

### 4.3 `ThemeMetrics::mac_tahoe()`

```rust
radius_group: 12.0,
radius_control: 10.0,
radius_control_sm: 8.0,
radius_button: 6.0,
radius_input: 8.0,
radius_glass: 12.0,
row_gutter: 16.0,
```

---

## 5. 渲染管线：三档降级（Quality Tiers）

```
Tier A — Native Liquid Glass (macOS 26+, dq-glass feature "native")
  NSGlassEffectView + NSGlassEffectContainerView
  · 真实动态采样、legibility、容器 merge
  · Iced widget 树中插入 native subview（raw-window-handle）

Tier B — GPU Shader Blur (all platforms, dq-glass feature "shader")
  wgpu Kawase dual-filter blur
  · 每帧：capture backdrop → blur → saturate → tint → composite
  · 用于 popover/modal/sidebar（无 native 时）

Tier C — Solid Fallback (prefers-reduced-transparency / software)
  对齐 Web @media (prefers-reduced-transparency)
  · --dq-glass-bar-bg-strong / --dq-glass-grouped-bg-solid
  · 无 blur，保证 a11y
```

**运行时选择**：

```rust
pub enum GlassBackend {
    Native,   // macOS 26+
    Shader,   // wgpu
    Solid,    // fallback
}

pub fn detect_glass_backend() -> GlassBackend {
    if cfg!(all(target_os = "macos", feature = "native-glass")) {
        if macos_version() >= (26, 0) && !accessibility_reduce_transparency() {
            return GlassBackend::Native;
        }
    }
    if cfg!(feature = "shader-glass") {
        return GlassBackend::Shader;
    }
    GlassBackend::Solid
}
```

---

## 6. 组件级复刻清单

| Web class | Iced 对应 | Glass tier | 备注 |
|-----------|-----------|------------|------|
| `.dq-glass--sidebar` | `glass_sidebar_container()` | A/B/C | 浮动侧栏，hairline 右边框 |
| `.dq-glass--bar` | `glass_topnav_container()` | A/B/C | 透明 menu bar 下浮动 toolbar |
| `.dq-glass--bar-strong` | scroll 时 promote | A/B | 滚动时提高 opacity |
| `.dq-glass--grouped` | `glass_inspector_stack()` | B/C | shell inspector |
| `.dq-glass--popover` | `glass_popover()` | B | 菜单、pick_list 面板 |
| `.dq-glass--panel` | `glass_modal_panel()` | B | dialog 内容区 |
| `.dq-glass` | 通用 `glass_surface()` | B/C | |
| `dq-tauri-macos` titlebar | `MacTitlebarInset` layout | — | 28px drag region + 60px nav |

### 6.1 阴影复刻（多层）

Web `--dq-shadow-popover-elevated` 三层 → Iced 叠加：

```rust
pub fn glass_popover_shadow() -> Vec<Shadow> {
    vec![
        Shadow { color: hairline, offset: (0., 0.), blur: 0. },      // 1px ring
        Shadow { color: ambient1, offset: (0., 4.), blur: 12. },
        Shadow { color: ambient2, offset: (0., 16.), blur: 48. },
    ]
}
```

Iced 0.14 `container::Style` 仅支持单个 `shadow` → **Phase 1** 用 Canvas overlay 或 **Phase 2** 扩展 `dq-glass` 自定义 widget 绘制多层。

### 6.2 Highlight 边（specular）

Web `--dq-glass-highlight: rgba(255,255,255,0.06)` → 顶部 1px 线性渐变，在 `GlassSurface` canvas program 中绘制。

---

## 7. macOS 原生集成（Tier A 细节）

### 7.1 依赖

```toml
# crates/dq-glass/Cargo.toml
[features]
default = ["shader-glass"]
native-glass = ["objc2", "objc2-app-kit", "raw-window-handle"]

[target.'cfg(target_os = "macos")'.dependencies]
objc2 = "0.5"
objc2-app-kit = "0.2"  # NSGlassEffectView @ macOS 26 SDK
```

### 7.2 Bridge 模式

```rust
/// 在 Iced 窗口的 NSView 层级插入 glass backing view
pub struct NativeGlassHost {
    effect_view: Retained<NSGlassEffectView>,
    container: Option<Retained<NSGlassEffectContainerView>>,
}

impl NativeGlassHost {
    pub fn attach(window: &Window, frame: Rect, corner_radius: f32) { ... }
    pub fn set_content_frame(&self, frame: Rect) { ... }
    pub fn set_merge_spacing(&self, spacing: f32) { ... }  // NSGlassEffectContainerView
}
```

**关键约束**（来自 Apple 文档）：

- 仅 `contentView` 保证在 glass 内部；Iced 的 wgpu layer 与 native glass 的 z-order 需实验确定。
- **推荐路径**：chrome 区域（sidebar/topbar）用 native glass **作为背景层**；Iced 在该区域渲染 **透明背景** 的 widget（文字/图标仍由 Iced 绘制）。

### 7.3 窗口配置

```rust
iced::window::Settings {
    transparent: true,
    decorations: false,  // 或 fullSizeContentView + titlebarAppearsTransparent
    ..
}
```

对齐 Tauri `dq-tauri-macos`：28px titlebar inset + 60px nav = `--dq-shell-header-height: 88px`。

---

## 8. wgpu Shader Blur（Tier B 细节）

当 Native 不可用（Linux/Windows/旧 macOS）：

```
Frame pipeline:
1. Render opaque content (page bg) to texture T0
2. For each GlassRegion:
   a. Copy region backdrop from T0 → blur ping-pong (Kawase 4-6 passes)
   b. Apply saturation matrix (1.8×)
   c. Alpha-composite tint color (--dq-glass-*-bg)
   d. Draw highlight gradient + border + shadows
3. Composite glass layers back; render foreground widgets
```

**性能预算**（1080p, M1）：

- Sidebar + topbar：≤ 1.5ms/frame
- Popover 打开：≤ 0.8ms 额外
- 缓存静态 backdrop，仅在 scroll/resize 时重算

**实现位置**：`crates/dq-glass/src/shader/` + Iced `canvas::Program` 或 custom `Shader` widget（需 spike）。

---

## 9. 实施阶段

### Phase 0 — Token & 实色 baseline（1 周）

- [ ] `SemanticPalette::mac_tahoe()` + `ThemeMetrics::mac_tahoe()` + `GlassTokens::mac_tahoe()`
- [ ] `mac_tahoe_theme()` in `dq-theme`
- [ ] 所有 container 改用 mac token（Tier C solid fallback，视觉对齐 Web reduced-transparency 模式）
- [ ] `examples/studio-create` 切换 `mac_tahoe_theme()` demo
- [ ] 脚本 `scripts/sync_tokens_from_css.py`：CI 校验 CSS ↔ Rust 像素值

**验收**：与 Web `dq-mac.css` + solid fallback 截图 diff ≤ 2%（SSIM）。

### Phase 1 — Shader glass（2–3 周）

- [ ] 新 crate `dq-glass`
- [ ] `GlassSurface` widget + Kawase blur
- [ ] 迁移 `sidebar_container` / `topnav_container` / `modal` / `inspector` 到 glass API
- [ ] 多层 shadow + highlight edge
- [ ] `prefers-reduced-transparency` → Tier C

**验收**：macOS 上 Shader tier 与 Web `backdrop-filter` 截图 diff ≤ 3%；60fps scroll。

### Phase 2 — Native Liquid Glass（2–3 周，仅 macOS 26+）

- [ ] `NSGlassEffectView` bridge
- [ ] `NSGlassEffectContainerView` merge sidebar + topbar
- [ ] Inactive window desaturation（对齐 `dq-tauri-macos.is-inactive`）
- [ ] Transparent window + titlebar inset layout

**验收**：与系统 Settings / Safari toolbar 并排对比；原生 glass merge 无断层。

### Phase 3 — 超越 Web（1–2 周）

- [ ] Scroll-linked bar strength（`.dq-glass--bar` → `--dq-glass--bar-strong`）
- [ ] Dynamic accent vibrancy on glass controls
- [ ] Popover morph animation（参考 SwiftUI `.glassEffect` transition）
- [ ] 同步回 Web：把验证过的 tint/blur 参数反写 `dq-mac.css`（optional）

---

## 10. 文件结构（新增）

```
crates/
├── dq-glass/                    # NEW
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── tokens.rs            # GlassTokens
│       ├── surface.rs           # GlassSurface widget
│       ├── backend.rs           # GlassBackend trait
│       ├── solid.rs             # Tier C
│       ├── shader/              # Tier B
│       │   ├── kawase.rs
│       │   └── composite.rs
│       └── native/              # Tier A (macOS)
│           └── macos26.rs
├── dq-theme/src/
│   ├── mac.rs                   # mac_tahoe_theme + glass containers
│   └── linear.rs                # (existing)
└── dq-tokens/src/
    └── semantic.rs              # + mac_tahoe()

docs/
└── iced-macos26-glass-plan.md   # this file

examples/
└── glass-showcase/              # NEW — 7 surface types side-by-side with Web iframe
```

---

## 11. 视觉 QA 矩阵

| 场景 | Web 参考 | Iced 目标 tier | 对比方式 |
|------|----------|----------------|----------|
| Sidebar + page | `demo/mac-glass.html` | A | 并排截图 SSIM |
| Top toolbar scroll | Studio shell | A + scroll promote | 录屏 |
| Inspector grouped | Settings pane | B | pixel diff |
| Modal / dialog | DqDialog glass | B | pixel diff |
| Popover / menu | pick_list | B | pixel diff |
| Reduced transparency | System a11y ON | C | 与 Web fallback 一致 |
| Inactive window | click away | A | saturation 目测 |
| 非 macOS | N/A | B | 不低于 Web |

---

## 12. 风险与对策

| 风险 | 对策 |
|------|------|
| Iced 0.14 无官方 blur API | 自定义 Canvas / wgpu pipeline；长期关注 iced 上游 |
| NSGlassEffectView 与 wgpu layer 冲突 | chrome 仅 native 背景 + Iced 透明 foreground；spike 优先 |
| 性能（4K 多 glass 层） | container merge + backdrop 缓存 + blur radius LOD |
| macOS 25 及以下无 NSGlass | 自动降级 Tier B |
| Token 漂移 | CSS→Rust sync 脚本 + CI |

---

## 13. 与现有主题的关系

```
Theme family          Source CSS              Glass
─────────────────────────────────────────────────────
macOS Tahoe (default) dq-mac.css + dq-glass.css  YES (本方案)
Linear Dark           dq-linear-dark.css       optional（低 blur）
China Red Dark        dq-china-red-dark.css    no（实色漆器美学）
```

DanQing 产品默认路线：

- **Tauri/Web 桌面**：继续 `dq-mac.css` + `dq-glass.css`
- **Iced 原生 Studio**：`mac_tahoe_theme()` + `dq-glass` Tier A/B/C
- **Linear / China**：生产力备选，无 glass

---

## 14. 下一步（建议立即启动 Phase 0）

1. 在 `semantic.rs` 添加 `mac_tahoe()`（纯 token，无 blur）
2. 添加 `dq-theme/src/mac.rs` 与 `mac_tahoe_theme()`
3. `studio-create` 增加 `--theme mac|linear|china` CLI flag
4. 创建 `examples/glass-showcase` 与 Web demo 并排对照页

预估 **Phase 0→2 全量交付：6–8 周**（1 人）；Phase 3 抛光 +2 周。
