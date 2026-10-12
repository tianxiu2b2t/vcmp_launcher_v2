# CHANGELOG

> 记录本仓库的实际改动，**精确到时间（时区 UTC，括号内为 UTC+8）**。
> 格式：`## [YYYY-MM-DD HH:MM UTC (HH:MM UTC+8)] 标题`，下面按「新增 / 修改 / 验证 / 备注」列出具体内容。

## [2026-10-11 17:44 UTC (2026-10-12 01:44 UTC+8)] 更新 Tauri 依赖 + 修复「找不到 vcmp_core.exe」导致的编译失败

### 修改（Tauri 相关依赖）
- Rust（`Cargo.lock` 由 `cargo update` 刷新到 2.x 最新）：
  | crate | 旧 | 新 |
  | --- | --- | --- |
  | `tauri` | 2.11.3 | **2.12.3** |
  | `tauri-build` | 2.6.3 | **2.7.2** |
  | `tauri-plugin-opener` | 2.5.4 | **2.7.1** |
  | `tauri-plugin-dialog` | 2.7.1 | **2.8.2** |
  | `wry` | 0.55.1 | **0.57.0** |
  同时把 `crates/src-tauri/Cargo.toml` 里 `"2"` / `"2.6.0"` 这类浮动的版本要求钉到上述具体版本，避免下次解析漂移。
- 前端（`package.json` + `pnpm-lock.yaml`，`pnpm install` 自更新）：
  | 包 | 旧 | 新 |
  | --- | --- | --- |
  | `@tauri-apps/api` | 2.12.2 | **2.12.3** |
  | `@tauri-apps/plugin-opener` | 2.7.0 | **2.7.1** |
  | `@tauri-apps/cli` | 2.12.1 | **2.12.2** |

### 修复（Linux 上报「找不到 vcmp_core」导致编译不过）
问题根源有两处，都会让 `cargo check` 在非 Windows 上直接失败：

1. **`launcher_core/src/constant.rs` 写死 `include_bytes!("../../../build/vcmp_core.exe")`**
   —— 注入器 exe 只能由 `cli.py` 在 Windows/i686 下产出，没产出前源码根本没法编译；
   而且这个相对路径实际指向 **仓库外**（`crates/launcher_core/src/` 往上三层是 `crates/`，第四层才是仓库根），
   与 `cli.py` 写入的 `<repo>/build/vcmp_core.exe` 并不一致。
   现在改为由新增的 **`crates/launcher_core/build.rs`** 在编译期决定，输出 `VCMP_CORE_BYTES` 环境变量：
   - `build/vcmp_core.exe` 存在 → 指向其绝对路径，**嵌入真实注入器（Windows 行为不变）**；
   - 不存在 → 在 `OUT_DIR` 写一个空占位文件并 `cargo:warning` 提示，源码仍可编译；
   - 不存在 **且** 宿主为 Windows **且** release → 直接 panic，防止打出没有注入器的发布包。
   `cargo:rerun-if-changed` 同时监视 `build/` 目录，`cli.py` 之后生成 exe 会触发重新嵌入。
2. **`vcmp_core/src/launch.rs` 末尾的 `windows::launcher_common_game(...)` 是无条件编译的**
   —— 该 crate 的 `windows` 依赖现在按 `[target.'cfg(windows)'.dependencies]` 引入，非 Windows 上会报 `E0433: cannot find module or crate 'windows'`。
   已给这一行加上同样的 `#[cfg(target_os = "windows")]` 门槛（前面两个 early-return 已保证非 Windows/非 x86 到不了这里）。
   顺带把 `crates/vcmp_core/Cargo.toml` 里的 `windows` 依赖挪到 `cfg(windows)` 段，Linux 上不再尝试编译它。

### 验证
- ✅ `cargo check -p launcher_core --target x86_64-unknown-linux-gnu` —— **通过**，唯一输出是预期的
  `warning: ../../build/vcmp_core.exe not found; embedding an empty vcmp_core.exe (run 'python cli.py' ...)`。
- ✅ `cargo check -p vcmp_core --target x86_64-unknown-linux-gnu` —— **通过**（旧代码在此报 E0433）。
- ✅ build.rs 两条分支都实测过：`build/vcmp_core.exe` 存在时 `cargo:rustc-env=VCMP_CORE_BYTES=<绝对路径>`；
  删掉后自动回落到 OUT_DIR 的空占位文件（`-vv` 输出确认）。
- ✅ `pnpm install && pnpm build` —— 退出码 0，71 modules，`dist/` 正常（验证后已删）。
- ⚠️ `cargo check --workspace` 在 Linux 上仍会失败：Tauri 的 GUI 依赖链（glib/pango/gdk/cairo/atk、libdbus-sys 等）
  需要 GTK/WebKit 系统库，本环境 `/usr` 只读、apt/oma 都装不了，**属环境限制，按约定忽略**（Windows 上正常）。

### 备注
- `Cargo.lock` 是一次整体 `cargo update`，除上面的 tauri 系外还顺带升级了不少间接依赖（diff 看起来很大属正常）。
- 本轮工作环境里似乎有工具会在文件变动后跑 `rustfmt`，把 `launcher_core` / `src-tauri` / `vcmp_core` 多个**与本次无关**的源文件格式化了；
  已全部 `git checkout` 还原，最终 diff 只保留上面列出的真实改动。
- 未验证：`pnpm tauri dev/build`（需要 GUI + Windows 侧产物）、以及 Windows/i686 目标的真实构建。
- `cli.py` 的 `cp_build()` 写入的是 **仓库根 `build/vcmp_core.exe`**，与修好后的路径一致（旧的三层相对路径是错的）。

## [2026-10-11 16:55 UTC (2026-10-12 00:55 UTC+8)] 包管理器从 yarn 迁移到 pnpm

### 修改
- `crates/src-tauri/tauri.conf.json`：`beforeDevCommand` 改为 `pnpm dev`，`beforeBuildCommand` 改为 `pnpm build`（原为 `yarn dev` / `yarn build`）。
- `cli.py`：`dev_tauri()` 改为 `pnpm tauri dev`，构建分支改为 `pnpm tauri build`（原为 `yarn ...`）。
- `package.json`：新增 `"packageManager": "pnpm@12.8.1"` 用于钉住包管理器版本。

### 新增
- `pnpm-lock.yaml`：pnpm 生成的唯一 lockfile（`lockfileVersion: '9.0'`，518 个条目，`pnpm install` 成功生成）。
- `pnpm-workspace.yaml`：pnpm 设置文件，内含 `allowBuilds: { esbuild: true }`。
  pnpm 10+ 默认拦截依赖的 postinstall 脚本，不放行时 `pnpm install` 会以 `ERR_PNPM_IGNORED_BUILDS` 结束；
  pnpm 12 起不再读取 `package.json` 里的 `pnpm` 字段，因此该配置只能写在 `pnpm-workspace.yaml`（由 `pnpm approve-builds` 写入）。

### 删除
- `yarn.lock`、`package-lock.json`：迁移后不再需要，避免三个 lockfile 互相干扰（可从 git 历史找回）。
- 旧 `node_modules/`（由 npm 安装）一并清空，改由 pnpm 重新链接。

### 验证
- ✅ `pnpm install`（清空 `node_modules` 后从零安装）：退出码 **0**，8.6s 完成 494 个包，无 `ERR_PNPM_IGNORED_BUILDS`。
- ✅ `pnpm install` 二次运行：`Already up to date`，退出码 0（lockfile 稳定，无二次改动）。
- ✅ `pnpm build`（`vue-tsc --noEmit && vite build`）：退出码 **0**，71 modules，0 error，产物在 `dist/`（验证后已删）。
- ✅ `node_modules/.pnpm/esbuild@0.25.12/node_modules/esbuild/bin/esbuild` 存在 —— 证实 esbuild 的 postinstall 确实执行了。
- ✅ `node_modules/.bin/tauri --version` → `tauri-cli 2.12.1`，说明 `pnpm tauri` 链路可用。
- ⚠️ 每次安装会打印 WARN：`The store at /root/.local/share/pnpm/store/v11 is not used because packages cannot be hard linked from it`
  —— 沙箱限制，pnpm 自动降级到工作区内的 `node_modules/.pnpm-store/v11`，不影响结果。

### 备注
- 本轮仍未改动任何前端/后端源码；排版布局依旧留到下一轮。
- 未跑 `pnpm tauri dev/build`：需要图形环境，且 `launcher_core` 的 `include_bytes!(".../build/vcmp_core.exe")` 目前缺文件，Rust 侧无法编译（见上一条记录）。
- `pnpm-workspace.yaml` 的缩进由 `pnpm approve-builds` 生成时为 2 空格，本次手工统一为仓库风格的 4 空格；若下次用 `approve-builds` 重写会回到 2 空格（YAML 语义相同）。

## [2026-10-11 16:10 UTC (2026-10-12 00:10 UTC+8)] 建立项目文档与会话上下文

### 新增
- `AGENT.md`：项目操作手册。覆盖项目定位（Tauri 2 + Vue 3）、技术栈版本、完整目录结构、前后端构建命令、Rust 各 crate 的职责地图（配置/常量路径/心跳协议/启动编排/注入器）、前端布局与主题变量约定（`App.vue` / `Switchbar.vue` / `constants.ts` / i18n `common.json` 模板机制），以及 9 条已知坑位（浏览器无 `invoke` 会白屏、`vcmp_core` 必须 i686+Windows、`include_bytes!` 依赖 `build/vcmp_core.exe`、npm 缓存只读、`ROOT` 相对路径等）。
- `AGENT-CONTEXT.md`：会话上下文快照。记录本轮范围决策（用户选择「只做基础工作」，排版改动留到下一轮）、`ENTRY.md` 的待办、下一轮起点建议（Home 空壳 / Settings 无分组 / ServerList 裸表格 / Switchbar 高亮不同步）、本机环境实测状态与未验证项。
- `CHANGELOG.md`：本文件，建立时间基准与记录约定。

### 变更（依赖）
- 安装前端依赖 `node_modules/`（此前不存在）：因沙箱内 npm 全局缓存 `/root/.npm/_cacache` 只读（EROFS），改用
  `npm install --no-audit --no-fund --cache /tmp/npm-cache`，共 494 个包。
- `package-lock.json`：由上述 `npm install` 重写（+7874 / −3727 行），与当前 npm 11 的 lockfile 格式对齐。
- `yarn.lock`：`npm install` 期间被改写为 Yarn 1 格式的解析结果（+510 / −721 行，主要为 esbuild 多平台可选依赖被裁剪），**已 `git checkout` 还原**，保持仓库原有 lockfile 不变。

### 验证
- `npm run build`（`vue-tsc --noEmit && vite build`）**通过**：71 modules transformed，产物生成于 `dist/`（验证后已删除）。仅有 1 条无害警告：
  `./fonts/HarmonyOS_Sans_SC_Regular.ttf referenced in ./fonts/... didn't resolve at build time`
  —— `index.html` 的 `@font-face` 用的是相对根路径 `/fonts/...`，而字体实际位于 `src/assets/fonts/`，属既有问题（运行时回退到系统字体），本轮未改。
- `cargo check -p launcher_core` **失败（预期）**：
  `error: couldn't read crates/launcher_core/src/../../../build/vcmp_core.exe: No such file or directory`
  —— 印证 `constant.rs` 的 `include_bytes!` 需要先由 `python cli.py` 产出 `build/vcmp_core.exe`，否则整个 workspace 无法编译。本轮未生成该文件（需要 i686-pc-windows-msvc target）。

### 备注
- 未修改任何前端 / Rust 源码，排版布局按用户要求留到下一轮。
- 会话期间 `screenshots/origin/` 下新增了 VC-MP 原版服务器浏览器的参考截图
  （`internet.png`、`official.png`、`history.png`、`favourite.png`、`settings.png`、`add server.png`，16:08–16:09 上传；`favourite.png` 为更早的 05:59）。
  这些是「原版布局参考」，本轮未据其改动代码。
- 排版方向已定（16:2x，用户选择）：**不复刻原版 Win32 观感，做现代化重构** —— 保留现有信息架构，重做间距、层次与主题一致性；
  原版截图仅作信息/字段参考。执行顺序（下一轮）：`App.vue` 外壳 → `Internet.vue`+`ServerList.vue` 试点 → `Settings.vue` 分组 → `Home.vue` 首页。
