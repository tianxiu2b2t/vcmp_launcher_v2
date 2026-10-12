# AGENT-CONTEXT.md — 会话上下文快照

> 用途：把「上一轮发生了什么、下一轮从哪里接着干」记下来，换会话后先读这一份。
> 时间基准：**2026-10-12 14:17 UTC / 2026-10-12 22:17 (UTC+8)**。

## 1. 本轮做了什么

- 通读了整仓库（前端 `src/**`、`crates/src-tauri`、`crates/launcher_core`、`crates/vcmp_core`、构建脚本与配置）。
- 按 `ENTRY.md` 第 4 节要求落地文档：新建 `AGENT.md`（项目操作手册）、`AGENT-CONTEXT.md`（本文件）、`CHANGELOG.md`（改动记录）。
- 用提问向用户确认本轮范围，用户先选择 **「这轮只做基础工作」**：先建立文档/项目结构记录，**前端排版改动下一轮再做**；
  随后用户按 `ENTRY.md` §3.3 追加要求 **「把 yarn 改成 pnpm」**（10-11 16:55 完成）；
  接着按 §3 追加 **「更新 tauri 一些依赖，修复当前报错（Linux 下没有找到，需要忽略一下，windows 正常即可）」**（10-11 17:44 完成）：
  - tauri 系依赖升到 2.x 最新（Rust + npm 两侧，详见 `CHANGELOG.md` 17:44 条目）。
  - 修掉「找不到 vcmp_core.exe」的编译失败：新增 `crates/launcher_core/build.rs`，`build/vcmp_core.exe` 缺失时用 OUT_DIR 空占位兜底；
    并给 `vcmp_core/src/launch.rs` 里调用 `windows` crate 的那一行补上 `cfg(target_os = "windows")`。
- **10-12 14:17：修掉服务器列表获取失败的 panic**（用户实测日志：镜像站 TLS `InternalError` / 返回 HTML 导致
  `internet.rs:10` 在 tokio worker 上 panic 刷屏）：`internet.rs` 改为返回 `Result` + 镜像站回退 + 10s 超时 +
  可读解析错误；`server.rs` 的 UDP 短包/缺包不再 unwrap；前端 Internet 页加了刷新按钮与加载/失败/空态，
  单个服务器 ping 失败不再中断列表。详见 `CHANGELOG.md` 14:17 条目。
- 用户的要求（`ENTRY.md` §4.4）：**提交 PR（分支名 `agent/dsh-coding`），由用户手动合并**。分支已 push（含用户追加的 `ENTRY.md` 提交）。
- 结论性事实（供下次直接引用）：
  - 项目是 **Tauri 2**（`ENTRY.md` 里写的 "tauri-v3" 不准确）+ Vue 3 + TS + Vite。
  - 前端**唯一**与 Rust 通信出口是 `src/bridge.ts` 的 `invoke`；`src/constants.ts` 顶层就 `await getConfig()`。
  - 布局集中在 `src/App.vue`（header/main 的 margin + 主题变量）与 `src/components/Switchbar.vue`。
  - `src/views/Home.vue` 目前是空壳，`Internet.vue` = 刷新按钮 + 三态 + `ServerList`，`Settings.vue` 是 5 个竖排 `InputEdit`。

## 2. 用户明确提出的待办（ENTRY.md §3）

1. **需要修改前端排版布局** —— 尚未开始编码。用户 10-12 又提了一次「2. 排版布局问题」，但**没有说具体哪里不对**；
   已向用户询问：是不满意现状，还是运行时报错/错位？需要截图或文字描述才能动手。
2. 已确认的路线（用户于 10-11 16:2x 选择）：**不复刻原版 Win32 观感，做现代化重构** —— 保留现有信息架构（标签导航 + 路由页面 + ServerList/Settings/About），重做间距、层次与主题一致性；`screenshots/origin/` 仅作**信息与字段参考**，不作为像素目标。

## 3. 下一轮起点建议

- 方向既定：现代化重构。先从外壳与单页试点入手，别一次性重写全部页面：
  - 第一步（外壳）`src/App.vue`：统一页面留白/最大宽度、加 `box-sizing` 收敛、让 `header` 与 `main` 的对齐一致，并用 `--main-color`/`--bg-color` 等变量收敛层次。
  - 第二步（试点页）`src/views/Internet.vue` + `src/components/ServerList.vue`：现在是裸 `<table>`，无样式、密码列直接输出 `true/false`、ping 显示裸毫秒。
    参考原版信息架构（列表 / 右侧 Players / 底部 Server Info）做成现代卡片+表格，但视觉按现代风格。
  - 第三步（表单）`src/views/Settings.vue`：5 个裸 `InputEdit` → 分组卡片（账号 / 游戏路径 / 更新与主列表），复用已有 `InputEdit` 的 label 动效。
  - 第四步（首页）`src/views/Home.vue`：目前是空壳，可补首页信息层级（版本、快速启动、收藏/历史入口）。
- 顺手可修：`Switchbar` 的 `active` 不跟随路由同步（刷新后高亮错位）；`index.html` 的 `@font-face` 字体路径错误。
- 别忘了每轮结束更新 `CHANGELOG.md`（精确到时间）和本文件。

## 4. 本机环境实测状态

- 环境：Linux 无 GUI，无法渲染 Tauri 窗口，也**无法真正启动游戏**（`vcmp_core` 要求 Windows + i686）。
- 工具链：node v24.21.0、**pnpm 12.8.1（已取代 npm/yarn，见下）**、cargo/rustc 1.101.0-nightly。
- 依赖安装：**统一用 `pnpm install`**。仓库初始没有 `node_modules`；最终生成的 `pnpm-lock.yaml`(lockfileVersion 9.0) 是唯一 lockfile。
- 包管理器迁移（2026-10-11 本轮完成）：
  - `yarn.lock`、`package-lock.json` **已删除**（可从 git 历史找回）；`package.json` 钉 `"packageManager": "pnpm@12.8.1"`。
  - `tauri.conf.json` 的 `beforeDevCommand/beforeBuildCommand` 与 `cli.py` 已从 `yarn` 改为 `pnpm`。
  - pnpm 10+ 默认拦截依赖构建脚本，已在 `pnpm-workspace.yaml` 写 `allowBuilds: { esbuild: true }`
    （pnpm 12 不再读 `package.json` 的 `pnpm` 字段；`pnpm approve-builds` 会自动写入这一条）。
  - 沙箱里 `/root/.local/share/pnpm/store` 无法硬链接进工作区，pnpm 自动降级用 `node_modules/.pnpm-store`（有 WARN，正常）。
  - npm 在本沙箱不可用（全局缓存只读 EROFS），**不要再回头用 npm**。
- Cargo 侧同样有只读限制：`/root/.cargo/registry` 是只读的，`cargo update` 拉到新版本后下载会 `EROFS`。
  本轮的做法是 `export CARGO_HOME=/tmp/cargo-home`（先把 `/root/.cargo/config.toml` 与 `registry/` 拷过去），
  这样 USTC 镜像仍可用、新 crate 能落盘。以后在本沙箱跑 `cargo update/check` 记得带上。
- 前端路由是 `createWebHistory`，纯静态托管时刷新子路由会 404（Vite dev/preview 已处理）。

## 5. 本轮验证结果与存疑项

已验证：

- ✅ `pnpm install && pnpm build`（`vue-tsc --noEmit && vite build`）**通过**（71 modules、0 error，产 `dist/`，验证后已删）。
  唯一警告：`./fonts/HarmonyOS_Sans_SC_Regular.ttf ... didn't resolve at build time` —— `index.html` 里 `@font-face` 用的是 `/fonts/...`（根相对），
  但字体实际在 `src/assets/fonts/`，所以运行时找不到、会回退系统字体。**属既有 bug，尚未修**（改 `@font-face` 的 src 或在 `public/` 放字体即可）。
- ✅ `pnpm install` 清空 `node_modules` 后从零安装：退出码 0，二次运行 `Already up to date`；升级 tauri npm 包后仍是 0。
- ✅ `cargo check -p launcher_core --target x86_64-unknown-linux-gnu` —— **通过**（唯一输出是「找不到 build/vcmp_core.exe，嵌入空占位文件」的 warning）。
- ✅ `cargo check -p vcmp_core --target x86_64-unknown-linux-gnu` —— **通过**（旧代码在这里报 `E0433: cannot find module or crate 'windows'`）。
- ✅ build.rs 两条分支实测：有 `build/vcmp_core.exe` 时注入绝对路径，删掉后回落到 OUT_DIR 空占位（`-vv` 确认）。
- ✅ 10-12 修复网络 panic 后重跑：`cargo check -p launcher_core` 与 `pnpm build` 均通过。
- ⚠️ `cargo check --workspace` 在 Linux 上**仍会失败**（glib/pango/gdk/cairo/atk、libdbus-sys 的 pkg-config）。
  本环境 `/usr` 只读、apt/oma 都装不了，**按用户要求忽略**（Windows 上正常）。

仍存疑 / 未验证：

- **Windows / i686 目标的真实构建没跑过**（本机做不到）——尤其 `vcmp_core` 的 `cfg(windows)` 分支与 Tauri 2.12 的升级结果，需要用户在 Windows 侧确认。
- **网络失败路径没法在本沙箱复现**（需要 GUI 跑起来点进 Internet 页）；只能保证不再走 panic 分支。
- `src/components/ScreenToSmall.vue` 只在 `watch` 里 `console.log`，没有实际的小屏拦截行为。
- 10-12 的排版问题尚未定位：用户说「2. 排版布局问题」，但未描述具体现象，**下一步就是问清/要看截图**。
- 未验证 `pnpm tauri dev/build`（需要图形环境 + Windows 侧产物）；其余 pnpm 链路（`pnpm build`、`node_modules/.bin/tauri`）已通。
- 沙箱 `/root/.cargo/registry` 只读：`cargo update` 之后要下载新 crate 时必须 `export CARGO_HOME=/tmp/cargo-home`（先把 `/root/.cargo/{config.toml,registry}` 拷过去）。
- 本环境疑似有工具会在文件变动后自动 `rustfmt`，曾把十几个无关源文件格式化；已 `git checkout` 还原，提交前请再复查 `git status`。

## 5.1 会话期间出现的参考截图（重要）

`screenshots/origin/` 下新增了 **VC-MP 原版（0.4 Server Browser，Win32）界面截图**，2026-10-11 16:08~16:09 上传：

| 文件 | 内容 |
| --- | --- |
| `internet.png` | 主界面：菜单栏 + Favourites/Internet/Official/History 四个标签 + 服务器表格（Hostname/Ping/Players/Version/Gamemode）+ 右侧 Players 面板 + 底部 Server Info |
| `official.png` | Official 标签页 |
| `history.png` | History 标签页 |
| `favourite.png` | Favourites 标签页（05:59 上传，最早） |
| `settings.png` | Settings 对话框：Username / GTA:VC Directory + Browse / Autolocate CD & Steam / Updater password / Updater URL / Masterlist URL + OK、Cancel |
| `add server.png` | Add Server 小对话框：输入 HOST:PORT + OK/Cancel |

判断：这批截图是 `ENTRY.md` §3「需要修改前端排版布局」的参照物。**用户已决定：不复刻原版，做现代化重构**，
因此把它们当字段/信息架构参考即可（例如原版有 Players 面板与 Server Info 区、Settings 有 CD/Steam 自动定位按钮，
这些能力当前前端还没有对应实现）。

## 6. 相关文件速查

- 需求入口：`ENTRY.md`
- 项目手册：`AGENT.md`
- 改动记录：`CHANGELOG.md`
- 前端布局核心：`src/App.vue`、`src/components/Switchbar.vue`、`src/views/*.vue`
- 前后端契约：`src/bridge.ts` ↔ `crates/src-tauri/src/bridge.rs`
- 注入器嵌入逻辑：`crates/launcher_core/build.rs` ↔ `crates/launcher_core/src/constant.rs`（`VCMP_CORE_BYTES`）

## 7. 交付方式（用户要求）

- `ENTRY.md` §4.4：**提交 PR 即可，由用户手动合并**。
- 本轮分支名由用户指定：**`agent/dsh-coding`**；提交信息按仓库既有风格（`feat:/fix:/chore:` + 中文描述）。
- `origin` = `https://github.com/tianxiu2b2t/vcmp_launcher_v2`，目标基线分支是 `main`。
- 本机**没有 `gh` CLI、没有 GH_TOKEN**，只有 VS Code 的 `GIT_ASKPASS`（非交互，很可能拿不到凭据）；
  已尝试 `git push`，若失败则需用户提供凭据或自行推送，PR 链接用 GitHub 的 compare 页面构造。
