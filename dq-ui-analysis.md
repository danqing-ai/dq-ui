# dq-ui 问题诊断报告

> 基于 `packages/ui/src/style.css` (34KB)、`packages/shell/src/style.css` (7.7KB) 及 tokens 目录分析

---

## 1. 颜色系统：过度依赖半透明叠加，缺乏层级控制

### 现状

```css
/* 同一界面出现多种透明度，无规律 */
.dq-alert--info {
  background: color-mix(in srgb, var(--dq-accent) 12%, transparent);
  border-color: color-mix(in srgb, var(--dq-accent) 22%, transparent);
}

.dq-tag--light.dq-tag--success {
  background: color-mix(in srgb, var(--dq-system-green) 16%, transparent);
  border-color: color-mix(in srgb, var(--dq-system-green) 28%, transparent);
}
```

### 问题

- 12%、16%、22%、28% 等多种透明度并存，用户感知不到层级规律
- `color-mix` 结果在不同背景上差异大：深色模式发灰、浅色模式发飘
- 缺乏语义化的填充层级（Primary/Secondary/Tertiary/Quaternary）

### macOS 做法

系统仅使用 3-4 个固定透明度层级，每个层级用途明确：

| 层级 | 透明度 | 用途 |
|------|--------|------|
| Primary | 10% | 悬停、选中背景 |
| Secondary | 5% | 默认卡片/列表背景 |
| Tertiary | 2% | 极弱分隔、禁用态 |
| Quaternary | 0% | 纯透明 |

### 修复建议

```css
:root {
  --dq-fill-primary: rgba(255, 255, 255, 0.10);   /* 悬停、选中 */
  --dq-fill-secondary: rgba(255, 255, 255, 0.05);  /* 默认背景 */
  --dq-fill-tertiary: rgba(255, 255, 255, 0.02);   /* 极弱背景 */
  --dq-fill-quaternary: rgba(255, 255, 255, 0.00); /* 纯透明 */
}

/* 统一使用语义变量，禁止动态混色 */
.dq-alert--info {
  background: var(--dq-fill-secondary);
  border-color: var(--dq-border);
}
```

---

## 2. 边框系统：粗细和颜色不统一

### 现状

```css
border: 1px solid var(--dq-separator, rgba(84, 84, 88, 0.65));
border: 0.5px solid var(--dq-glass-border);
border: 1px solid var(--dq-separator-light, rgba(84, 84, 88, 0.35));
```

### 问题

- **0.5px 在 Web 上渲染不稳定**：非 Retina 屏幕会消失或强制变为 1px
- 分隔线、卡片边框、面板边框使用不同颜色系，缺乏统一的光照模型
- 边框颜色与背景色对比度不足，导致界面"糊"在一起

### macOS 做法

- 统一使用 **1px** 物理像素
- 边框颜色基于白色/黑色的固定透明度，而非具体色值
- 分隔线比边框更淡，但属于同一色系

### 修复建议

```css
:root {
  --dq-border: rgba(255, 255, 255, 0.08);         /* 通用边框 */
  --dq-separator: rgba(255, 255, 255, 0.06);      /* 内部分隔（更淡） */
  --dq-border-strong: rgba(255, 255, 255, 0.12);  /* 强调边框 */
}

/* 删除所有 0.5px 用法 */
.dq-glass-panel {
  border: 1px solid var(--dq-border);  /* 替代 0.5px */
}
```

---

## 3. 阴影：只有一层，缺乏环境感

### 现状

```css
box-shadow: var(--dq-shadow-glass, 0 12px 32px rgba(0, 0, 0, 0.35));
```

### 问题

- Dialog、Drawer、Dropdown、Toast 全部使用同一套阴影
- 没有区分"距离用户多远"：Dropdown 应该轻、Dialog 应该重
- 缺乏内高光（inset highlight），导致面板没有"厚度"

### macOS 做法

多层阴影模拟真实环境光：

```css
/* Popover：轻量，近距离 */
--shadow-popover:
  0 0 0 1px rgba(255,255,255,0.08),   /* 内高光（边框感） */
  0 4px 12px rgba(0,0,0,0.15),         /* 近距离阴影 */
  0 16px 48px rgba(0,0,0,0.25);       /* 环境漫射 */

/* Modal：重量，远距离 */
--shadow-modal:
  0 0 0 1px rgba(255,255,255,0.08),
  0 8px 24px rgba(0,0,0,0.20),
  0 32px 64px rgba(0,0,0,0.30);

/* Toast：悬浮，无内高光 */
--shadow-toast:
  0 4px 16px rgba(0,0,0,0.20),
  0 12px 32px rgba(0,0,0,0.15);
```

### 修复建议

按组件类型分配阴影层级：

| 组件 | 阴影层级 | 说明 |
|------|---------|------|
| Dropdown/Tooltip | `shadow-popover` | 轻量，依附于触发源 |
| Dialog/Drawer | `shadow-modal` | 重量，阻断式交互 |
| Toast | `shadow-toast` | 中等，悬浮通知 |
| Card（静态） | 无阴影或 `0 1px 3px rgba(0,0,0,0.1)` | 微弱层级感 |

---

## 4. 动画：过于简单，缺乏物理感

### 现状

```css
transition: background 0.15s ease, color 0.15s ease;
animation: dq-spin 0.75s linear infinite;
```

### 问题

- `ease` 是 CSS 默认曲线，没有"快启慢停"的质感
- 所有过渡统一 0.15s，未区分交互类型（悬停 vs 按下 vs 弹出）
- 弹出层（Dialog/Drawer）缺乏进入/退出动画
- 列表项悬停无微缩放或位移反馈

### macOS 动画规范

| 交互类型 | 时长 | 曲线 | 效果 |
|---------|------|------|------|
| 悬停（Hover） | 0.08s | `ease` | 背景色渐变 |
| 按下（Active） | 0.08s | `cubic-bezier(0.25, 0.1, 0.25, 1)` | 缩放 0.97 |
| 弹出（Enter） | 0.30s | `cubic-bezier(0.32, 0.72, 0, 1)` | 滑入 + 缩放 |
| 消失（Leave） | 0.20s | `ease-in` | 淡出 |
| 状态切换 | 0.20s | `cubic-bezier(0.25, 0.1, 0.25, 1)` | 平滑过渡 |

### 修复建议

```css
:root {
  --dq-transition-hover: all 0.08s ease;
  --dq-transition-active: transform 0.08s cubic-bezier(0.25, 0.1, 0.25, 1);
  --dq-transition-enter: all 0.30s cubic-bezier(0.32, 0.72, 0, 1);
  --dq-transition-leave: all 0.20s ease-in;
}

/* 按钮：增加按下反馈 */
.dq-btn {
  transition: var(--dq-transition-hover);
}
.dq-btn:active:not(:disabled) {
  transform: scale(0.97);
  transition: var(--dq-transition-active);
}

/* 弹出层：滑入动画 */
@keyframes panelIn {
  from {
    opacity: 0;
    transform: translateY(12px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.dq-dialog-content {
  animation: panelIn 0.30s cubic-bezier(0.32, 0.72, 0, 1);
}

/* 列表项：悬停微位移 */
.dq-list-item {
  transition: var(--dq-transition-hover);
}
.dq-list-item:hover {
  background: var(--dq-fill-secondary);
  transform: translateX(2px);
}
```

---

## 5. 排版：字重和行高缺乏节奏

### 现状

```css
font-size: 13px;
font-weight: 500;  /* 或 400、600 */
line-height: 1;   /* 按钮 */
line-height: 1.5; /* 正文 */
```

### 问题

- 按钮 `line-height: 1` 导致文字偏上或偏下（不同字体基线不同）
- 字重使用随意：400/500/600 没有对应明确的层级
- 未使用系统字体栈的优化字重（SF Pro 支持 350/450 等非整数值）

### macOS 排版规范

| 样式 | 字号 | 字重 | 行高 | 字间距 | 用途 |
|------|------|------|------|--------|------|
| Large Title | 26px | 700 | 1.2 | -0.02em | 应用标题 |
| Title 1 | 22px | 600 | 1.2 | -0.02em | 页面标题 |
| Title 2 | 17px | 600 | 1.3 | -0.01em | 卡片标题 |
| Headline | 13px | 600 | 1.3 | 0 | 列表标题、标签 |
| Body | 13px | 400 | 1.4 | 0 | 正文 |
| Callout | 12px | 400 | 1.4 | 0 | 辅助说明 |
| Caption | 11px | 400 | 1.3 | 0.01em | 最小文字 |

### 修复建议

```css
:root {
  /* 字体栈 */
  --dq-font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Helvetica Neue', sans-serif;

  /* 字号层级 */
  --dq-text-xs: 11px;
  --dq-text-sm: 12px;
  --dq-text-base: 13px;
  --dq-text-lg: 15px;
  --dq-text-xl: 17px;

  /* 字重 */
  --dq-weight-regular: 400;
  --dq-weight-medium: 500;
  --dq-weight-semibold: 600;

  /* 行高 */
  --dq-leading-tight: 1.25;  /* 标题 */
  --dq-leading-normal: 1.4;   /* 正文 */
  --dq-leading-relaxed: 1.5;  /* 长文本 */
}

/* 按钮统一行高 */
.dq-btn {
  line-height: var(--dq-leading-tight);
}
```

---

## 6. 组件级具体问题

### 6.1 Button

| 问题 | 现状 | 修复 |
|------|------|------|
| 尺寸档位 | 仅 28px（默认）和 24px（sm） | 增加 22px（xs）、36px（lg）三档 |
| 圆角 | 统一 6px | 随尺寸变化：xs 4px、sm/md 6px、lg 8px |
| 按下态 | 无 | 增加 `transform: scale(0.97)` |
| 图标按钮 | 36x36 过大 | 标准 28x28，与文字按钮同高 |

```css
.dq-btn--xs { height: 22px; padding: 0 8px; font-size: 11px; border-radius: 4px; }
.dq-btn--sm { height: 24px; padding: 0 10px; font-size: 12px; border-radius: 6px; }
.dq-btn--md { height: 28px; padding: 0 14px; font-size: 13px; border-radius: 6px; }
.dq-btn--lg { height: 36px; padding: 0 20px; font-size: 14px; border-radius: 8px; }
```

### 6.2 Input

| 问题 | 现状 | 修复 |
|------|------|------|
| 聚焦态 | 仅边框变色 | 增加外发光 `box-shadow: 0 0 0 3px rgba(10,132,255,0.15)` |
| 背景 | 玻璃态半透明 | 增加纯色兜底 `background: var(--dq-glass-control-bg-solid)` |
| 高度 | 32px 固定 | 增加 28px（sm）、40px（lg）变体 |

```css
.dq-input:focus {
  outline: none;
  border-color: var(--dq-accent);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--dq-accent) 15%, transparent);
}
```

### 6.3 Switch

| 问题 | 现状 | 修复 |
|------|------|------|
| Thumb 质感 | 纯白，扁平 | 增加内阴影模拟立体感 |
| 过渡 | 仅 thumb 位移 | 增加背景色渐变过渡 |

```css
.dq-switch__thumb {
  background: #fff;
  box-shadow:
    inset 0 0 0 0.5px rgba(0,0,0,0.1),  /* 内描边 */
    0 1px 3px rgba(0,0,0,0.22);          /* 外阴影 */
}
```

### 6.4 Dialog

| 问题 | 现状 | 修复 |
|------|------|------|
| Overlay | 纯黑遮罩 | 半透明黑 + 背景模糊 |
| 进入动画 | 无 | 增加滑入 + 缩放 |
| 关闭按钮 | 22px 文字 | 改为图标，悬停背景圆角 |

```css
.dq-dialog-overlay {
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
}

.dq-dialog-content {
  animation: panelIn 0.30s cubic-bezier(0.32, 0.72, 0, 1);
}
```

### 6.5 Toast

| 问题 | 现状 | 修复 |
|------|------|------|
| 位置 | 右下固定 | 改为顶部居中或跟随操作源 |
| 进入动画 | 无 | 从上方滑入 |
| 阴影 | 与 Dialog 相同 | 使用轻量级 `shadow-toast` |

```css
.dq-toast-host {
  position: fixed;
  top: 16px;
  left: 50%;
  transform: translateX(-50%);
}

@keyframes toastIn {
  from { opacity: 0; transform: translateY(-8px); }
  to { opacity: 1; transform: translateY(0); }
}
```

### 6.6 Section Tabs

| 问题 | 现状 | 修复 |
|------|------|------|
| 下划线 | `::after` 固定宽度 14px | 改为动态宽度或全宽细线 |
| 选中态 | 仅加粗 + 下划线 | 增加背景色微变 |
| 圆角 | 6px 6px 0 0 | 改为 pill 形状（全圆角）或直角 |

```css
/* Safari 风格：全宽细线 */
.dq-section-tabs__list {
  border-bottom: 1px solid var(--dq-separator);
}

.dq-section-tabs__trigger[data-state='active']::after {
  left: 0;
  right: 0;
  height: 2px;
  border-radius: 0;
  box-shadow: none;  /* 删除发光 */
}
```

---

## 7. 最致命的缺失：全局交互语言不统一

### 现状

各组件各自为政，缺乏一致的"系统感"：

| 状态 | 你的现状 | macOS |
|------|---------|-------|
| **选中态** | 蓝色背景 + 白字 | 蓝色背景 + 白字 + **左侧 3px 强调线** |
| **悬停态** | 背景变浅 | 背景变浅 + **0.08s 快速响应** |
| **按下态** | 无 | **缩放 0.97** + 背景加深 |
| **禁用态** | opacity 0.45 | 颜色变灰 + **无 hover 反馈** |
| **聚焦态** | outline 或 border 变色 | **蓝色外发光 ring** |
| **加载态** | 旋转图标 | **骨架屏** 或 **脉冲动画** |

### 修复：建立全局状态规范

```css
/* === 全局交互状态规范 === */

/* 1. 悬停（Hover） */
.dq-hoverable:hover:not(:disabled):not([data-disabled]) {
  background: var(--dq-fill-secondary);
  transition: all 0.08s ease;
}

/* 2. 按下（Active） */
.dq-pressable:active:not(:disabled):not([data-disabled]) {
  transform: scale(0.97);
  transition: transform 0.08s cubic-bezier(0.25, 0.1, 0.25, 1);
}

/* 3. 选中（Selected） */
.dq-selected {
  background: var(--dq-accent);
  color: #fff;
  position: relative;
}
.dq-selected::before {
  content: '';
  position: absolute;
  left: 0;
  top: 4px;
  bottom: 4px;
  width: 3px;
  border-radius: 0 2px 2px 0;
  background: rgba(255,255,255,0.8);
}

/* 4. 聚焦（Focus） */
.dq-focusable:focus-visible {
  outline: none;
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--dq-accent) 15%, transparent);
}

/* 5. 禁用（Disabled） */
.dq-disabled,
:disabled,
[data-disabled] {
  opacity: 0.45;
  cursor: not-allowed;
  pointer-events: none;
}
```

---

## 8. 修复优先级建议

### P0（立即修复）

1. **固化 Design Token**：收敛颜色、阴影、动画、间距到 10 个以内核心变量
2. **统一动画曲线**：全部替换为 `cubic-bezier(0.25, 0.1, 0.25, 1)` 或更弹簧的曲线
3. **删除 0.5px 边框**：统一为 1px，用透明度区分强弱

### P1（本周修复）

4. **增加按压反馈**：所有可点击元素加 `:active` 状态（`scale(0.97)`）
5. **阴影分层**：Popover/Modal/Toast 使用不同层级阴影
6. **输入框聚焦外发光**：`box-shadow: 0 0 0 3px rgba(10,132,255,0.15)`

### P2（本月修复）

7. **排版节奏**：建立字号/字重/行高的明确层级
8. **组件尺寸体系**：Button/Input 增加 xs/sm/md/lg 四档
9. **Toast 位置**：从右下改为顶部居中 + 滑入动画

### P3（长期优化）

10. **减少玻璃态使用**：在 `prefers-reduced-transparency` 下提供纯色兜底
11. **系统字体优化**：使用 `-apple-system` 字体栈，支持动态字重
12. **微交互打磨**：列表悬停微位移、按钮图标旋转、加载脉冲等

---

## 9. 参考标杆

| 应用 | 可借鉴点 |
|------|---------|
| **Safari 设置页** | 表单布局、标签左对齐、开关样式 |
| **Xcode Inspector** | 三栏布局、信息密度、分组间距 |
| **Things 3** | 侧边栏选中态、动画曲线、极简工具栏 |
| **Raycast** | 命令面板、搜索交互、模糊背景 |
| **Linear** | 视觉风格、信息密度、暗色模式 |

---

## 10. 一句话总结

> `dq-ui` **单个组件看起来是对的**，但组合在一起缺乏**统一的光照模型、动画语言和层级系统**。精致感来自"每个像素都有理由"，而当前代码里太多随意值（12%、22%、0.5px、0.15s ease）。

**核心原则**：
1. **少即是多**：减少变量数量，增加语义清晰度
2. **快即是慢**：悬停 0.08s、按下 0.08s、弹出 0.30s，区分交互节奏
3. **暗即是亮**：用透明度而非混色，保持色彩纯净
4. **静即是动**：静态时安静，交互时有物理反馈
