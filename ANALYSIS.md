# DanQing UI (`dq-ui`) 问题分析报告

## 项目概述
- **目标**: macOS 26 (Tahoe) 风格桌面软件 UI 库
- **架构**: Monorepo (pnpm workspace), 3 个包: `@danqing/dq-tokens`, `@danqing/dq-ui`, `@danqing/dq-shell`
- **技术栈**: Vue 3 + Reka UI + Lucide icons, 无 Element Plus

---

## 1. 工程化与构建问题

| 问题 | 严重程度 | 详情 |
|------|----------|------|
| **无构建流程** | 高 | `package.json` 的 `exports` 直接指向 `src/` 源文件，没有编译/打包步骤，消费者直接消费原始 Vue/TS |
| **tsconfig 过于简陋** | 高 | 缺少 `lib`、`types`、`resolveJsonModule` 等关键配置，无法保证类型安全 |
| **无代码规范** | 中 | 没有 ESLint、Prettier、Stylelint 配置，代码风格不统一 |
| **无测试覆盖** | 高 | 零测试覆盖，组件行为无保障 |
| **外部依赖硬编码** | 中 | `Makefile` 依赖 `../DanQing-Studio`，无法独立运行质量检查 |
| **版本不一致** | 中 | `tokens/src/index.ts` 导出 `DQ_TOKENS_VERSION = '0.2.0'`，但 `package.json` 是 `0.1.0` |

---

## 2. CSS 架构问题

- **单文件巨兽**：`style.css` 1724 行，所有组件样式堆在一起，维护困难
- **无 Scoped Styles**：Vue 组件完全没有 `<style scoped>`，全靠全局类名，极易冲突
- **暗黑模式单一**：`dq-mac.css` 只定义了 `:root, html.dark`，无亮色模式（Light Mode）支持
- **硬编码回退值**：大量 `var(--dq-xxx, #硬编码色值)`，回退值与 token 系统脱节
- **CSS 重复**：`.dq-icon-btn` 选择器定义了两次（style.css line 124 & 142）

---

## 3. 组件设计缺陷

| 组件 | 问题 | 位置 |
|------|------|------|
| **DqSelect** | `toSelectRootValue`/`fromSelectRootValue` 转换逻辑过度复杂；`size` prop 同时读取 attrs，API 混乱 | `DqSelect.vue` |
| **DqButton** | 同时支持 `props.plain` 和 `class="is-plain"` 两种模式，兼容包袱重 | `DqButton.vue` |
| **Toast** | 使用全局 `reactive(toastState)`，无上限控制，长时间运行可能内存泄漏 | `toastState.ts` |
| **Confirm** | 全局单例状态，不支持同时弹出多个确认框 | `confirmState.ts` |
| **Feedback 系统** | `createApp` 在 `installFeedback` 中创建独立 Vue 实例，与主应用状态隔离 | `installFeedback.ts` |

---

## 4. macOS 26 "Liquid Glass" 风格问题

- **材质效果浅层**：只有简单的 `backdrop-filter: blur()`，没有 macOS 26 的动态材质（根据背景内容自适应透光度）
- **无系统强调色**：`--dq-accent` 写死为 `#0a84ff`，未跟随用户系统偏好（System Preferences → Accent Color）
- **圆角保守**：`--dq-radius-group: 12px`，macOS 26 趋势是更大圆角（16-20px）
- **无深度层级**：缺少 macOS 26 的多层阴影/深度（elevation）系统，Z轴层次感不足
- **静态毛玻璃**：所有 glass 效果都是静态 CSS，无动态响应背景变化的材质系统

---

## 5. 可访问性 (a11y) 问题

- Toast 关闭按钮使用 `"×"` 字符而非语义化图标
- 部分表单组件缺少 `aria-*` 属性
- 无键盘焦点环的视觉一致性保障（`--dq-focus-ring` 仅在部分组件使用）
- Dialog/Drawer 缺少 `aria-modal`、`aria-labelledby` 等必要属性

---

## 6. 依赖与包管理问题

- **本地依赖硬编码**：`shell` 包通过 `file:../ui` 本地引用 `dq-ui`，无法管理版本升级
- **peerDependencies 配置不当**：`dq-ui` 声明 `vue` 为 peer，但 devDependencies 中安装了 `vue`，可能导致重复安装
- **缺少 lockfile 统一**：各子包有独立的 `package-lock.json`，但主工程使用 pnpm

---

## 7. 建议优先级

### 🔴 高优先级

1. **引入构建流程**：Vite/Rollup 构建，输出 ESM + 类型声明 + CSS 提取
2. **拆分 CSS**：将 `style.css` 拆分为各组件的 `<style scoped>` 或 CSS Modules
3. **添加亮色模式**：支持 `html.light` / `html.dark` 双模式
4. **添加测试**：至少为公共 API（toast, confirm, 表单组件）添加单元测试

### 🟡 中优先级

5. **接入系统强调色**：使用 CSS `accent-color` 或媒体查询 `prefers-color-scheme`
6. **为 Toast/Confirm 添加堆栈上限**：防止内存泄漏
7. **统一代码规范**：添加 ESLint + Prettier + Stylelint
8. **修复版本不一致**：统一 package.json 与源码中的版本号

### 🟢 低优先级

9. **实现真正的动态 Liquid Glass 材质**：需 Canvas/WebGL 分析背景（高级特性）
10. **增加 macOS 26 风格的大圆角和深度层级**：`--dq-radius-group: 16px` 或更大
11. **支持多确认框并发**：将 confirm 状态从单例改为队列管理
12. **完善可访问性**：添加 ARIA 属性，支持键盘导航和屏幕阅读器

---

## 附录：文件清单

```
dq-ui/
├── packages/
│   ├── tokens/
│   │   ├── src/
│   │   │   ├── dq-mac.css          # 主 token 文件（191行）
│   │   │   ├── dq-tauri-macos.css  # Tauri 特殊样式（32行）
│   │   │   ├── dq-glass.css        # Glass 工具类（87行）
│   │   │   └── index.ts            # 版本导出
│   │   └── package.json
│   ├── ui/
│   │   ├── src/
│   │   │   ├── index.ts            # 主入口（45行）
│   │   │   ├── style.css           # 组件样式（1724行！）
│   │   │   ├── components/         # 30+ Vue 组件
│   │   │   ├── feedback/           # Toast, Confirm, Loading
│   │   │   ├── form/               # Select 相关逻辑
│   │   │   └── icons/              # Lucide 图标注册
│   │   ├── package.json
│   │   └── tsconfig.json           # 过于简陋
│   └── shell/
│       ├── src/
│       │   ├── index.ts            # 重导出 ui
│       │   ├── style.css           # Inspector 样式（404行）
│       │   └── components/         # 9 个 Shell 组件
│       └── package.json
├── package.json                    # Workspace 根
├── pnpm-workspace.yaml
├── Makefile                        # 依赖外部仓库
└── README.md
```

---

*分析时间: 2026-05-19*
