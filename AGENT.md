# AGENT.md — vcmp_launcher_v2 项目操作手册

> 给后续 AI / 开发者的工程说明。目标是「读完这一份就知道怎么改、怎么跑、哪里会踩坑」。
> 变更记录见 `CHANGELOG.md`，本次会话上下文见 `AGENT-CONTEXT.md`。

## 1. 项目是什么

Vice City Multiplayer（VC-MP）启动器：

- 外壳：**Tauri 2**（不是 tauri-v3，`Cargo.toml` 里是 `tauri = "2"`）+ Vue 3 前端。
- 核心：`crates/launcher_core`（配置、服务器列表/心跳、数据库、资源下载解压、启动编排）。
- 注入器：`crates/vcmp_core`（独立 CLI 可执行文件，创建挂起的 GTA 进程并注入 `vcmp-game.dll`）。
- 目标平台：游戏本体与注入器**只能在 Windows（XP ~ 11）运行**；注入器必须编译为 **i686**（32 位）。
- 当前开发环境：**Linux 无 GUI**，Tauri 窗口无法渲染；前端只能以网页方式在浏览器里跑（但浏览器里没有 `invoke`，见 §7 坑 1）。

## 2. 技术栈与版本

| 层 | 技术 |
| --- | --- |
| 桌面包 | Tauri 2（`tauri` 2.12.3 / `tauri-build` 2.7.2 / `tauri-plugin-opener` 2.7.1 / `tauri-plugin-dialog` 2.8.2、`sevenz-rust2`、`rand`、`tokio`） |
| 前端 | Vue 3.5 `<script setup>`、TypeScript 5.6、Vite 6、vue-router 4、vue-i18n 11、vue-debounce、@vueuse/core（tauri npm 包：api 2.12.3 / plugin-opener 2.7.1 / cli 2.12.2） |
| 样式 | 原生 CSS + CSS 变量（无 Tailwind，`InputEdit.vue` 是从 Tailwind 手写换算过来的） |
| Rust | edition 2021，workspace `resolver = "3"`，nightly 工具链（`vcmp_core` 用到 `c"..."` 字面量） |
| 后端库 | reqwest 0.13、sqlx 0.8(sqlite)、tracing、encoding_rs(GBK)、chrono、toml、url |
| 测试环境 | node v24.21.0 / pnpm 12.8.1（唯一使用的包管理器）/ cargo 1.101.0-nightly |

## 3. 目录结构

```
.
├── cli.py                     # 入口脚本：先编 vcmp_core.exe，再 pnpm tauri dev/build
├── index.html                 # HTML 外壳：全局 reset、字体、滚动条、#app
├── vite.config.ts             # Vite 配置：SVG 雪碧图、端口 1420、es2022
├── .cargo/config.toml         # i686/x86_64 windows-msvc 的 rustflags（SUBSYSTEM 5.01/5.02 + crt-static）
├── crates/
│   ├── src-tauri/             # Tauri 应用（bin 名 vcmp_launcher，lib 名 vcmp_launcher_lib）
│   │   ├── tauri.conf.json    # 窗口 800x600、beforeDev/BuildCommand 用 pnpm
│   │   ├── capabilities/      # core:default + opener:default
│   │   └── src/{main,lib,bridge,state,utils}.rs
│   ├── launcher_core/         # 业务核心（lib）
│   │   ├── build.rs           # 决定 VCMP_CORE_BYTES：嵌入真实 vcmp_core.exe 或 OUT_DIR 空占位
│   │   └── src/{config,constant,models,internet,server,resource,link,database,launch,logger,utils}.rs
│   └── vcmp_core/             # 注入器 CLI（bin + lib），.cargo/config.toml 固定 i686-pc-windows-msvc
│       └── src/{main,lib,launch}.rs + launch/{windows,linux}.rs
└── src/                       # 前端
    ├── main.ts                # createApp + router + i18n + 全局注册 SvgIcon
    ├── App.vue                # 布局壳：Progressbar + header(Switchbar) + main(RouterView) + 主题变量
    ├── constants.ts           # 路由表、全局 config、internetServers、progressbar 等全局状态
    ├── bridge.ts              # 所有 invoke 封装（前端唯一与 Rust 通信的出口）
    ├── types.ts / utils.ts    # 与 Rust 对应的数据结构、hashServer/toMillis
    ├── i18n.ts                # 自动加载 locales/*.json + ${common.x} 模板替换
    ├── locales/{common,en,zh_cn}.json
    ├── components/            # Progressbar / Switchbar / ServerList / InputEdit / SvgIcon / ScreenToSmall
    ├── views/                 # Home / Internet / Settings / About / Debug
    └── assets/                # 字体、svg 图标源（Hero.svg）、componesables/screen.ts
```

## 4. 常用命令

```bash
# 前端依赖（本项目统一用 pnpm；lockfile 只有 pnpm-lock.yaml，见 §7 坑 4）
pnpm install

pnpm dev        # 仅前端 Vite，http://localhost:1420（strictPort）
pnpm build      # vue-tsc --noEmit && vite build  → dist/
pnpm preview    # 预览 dist/
pnpm tauri dev  # 需要图形环境，本机跑不起来
pnpm tauri build

# 后端（注意：仓库根 .cargo/config.toml 设了默认 Windows target，
#        在 Linux 上必须显式 --target 才能在宿主机上检查）
cargo check -p launcher_core --target x86_64-unknown-linux-gnu       # 可在 Linux 上检查
cargo check -p vcmp_core   --target x86_64-unknown-linux-gnu         # 可在 Linux 上检查
cargo build --release -p vcmp_core --target i686-pc-windows-msvc     # 注入器（需先装该 target）
cargo build --release                                                # 整个 workspace（Windows）

python cli.py dev      # 推荐入口：装 i686 target → 编 vcmp_core.exe → 拷到 build/ → pnpm tauri dev
python cli.py build --release
```

**注入器的嵌入方式**：`crates/launcher_core/build.rs` 会在编译期检查 `<repo>/build/vcmp_core.exe`——
存在就把它作为绝对路径塞进 `VCMP_CORE_BYTES` 环境变量、由 `constant.rs` 的 `include_bytes!(env!(...))` 嵌入；
不存在则在 `OUT_DIR` 写一个空占位文件并 `cargo:warning` 提示，源码仍可编译（`launch::init()` 会跳过解包）。
所以「没先跑 `cli.py` 就编译不过」的问题已经解决；但**要打出能启动游戏的包，仍然必须先有 `build/vcmp_core.exe`**
（Windows 宿主的 release 构建在缺失时会直接 panic）。
`cli.py` 的 `cp_build()` 写入的正是 `<repo>/build/vcmp_core.exe`，与 build.rs 的路径一致。

## 5. Rust 侧地图

- `src-tauri/src/lib.rs::run()`：启动时依次 `logger::init` → `config::init_config` → `launch::init`（把内嵌的 vcmp_core.exe 写到 `runner/libs/`）→ `database::init_database()`，然后注册 7 个 command。
- `src-tauri/src/bridge.rs`：`get_config` / `set_config` / `fetch_internet_servers` / `ping_server` / `download_resource` / `random_object_id` / `launch`。参数名要与 `src/bridge.ts` 的 `invoke(name, { ... })` 完全一致。
- `src-tauri/src/state.rs`：`TauriProgressbar` 实现 `ProgressbarBase`，每步 `app_handle.emit(echo_id, {status, progress, total, current})`，前端 `listen(id, ...)` 接收。进度事件用 `random_object_id()` 生成的 id 做频道。
- `launcher_core::config`：全局 `OnceLock<RwLock<Config>>`，文件是 `runner/config.toml`（debug）或 `./config.toml`（release）；`game_exe` 路径不存在时会返回 None 并告警。
- `launcher_core::constant`：所有路径都在 `ROOT` 下 —— debug 是 `../../runner/`（相对 Cargo 工作目录，即仓库根的 `runner/`），release 是 `./`。`VERSIONS`/`GAME_DATA`/`DATABASE_FILE` 都在 `appdata/` 下。
- `launcher_core::link`：把版本目录里的 `serverlogs/`、`store/` 做成 junction（目录）、`vcmp_config.txt`、`debuglog.txt`、`.appdata` 做成硬链接，统一指向 `appdata/game_data`（用 `junction_point`，需 `#![feature(junction_point)]`）。
- `launcher_core::server`：UDP 私有协议。发送 `"VCMP" + ip(be) + port(be) + "i"` 与 `..."c"`，收两包后按 GBK 解析名称/模式/地图/玩家列表，`elapsed` 是首包网络耗时。
- `launcher_core::launch`：组装 `--gta-exe --dll-file --username --ip --port [--password]` 参数调用 `runner/libs/vcmp_core.exe`，并从子进程 stdout 里抓 `pid: N`。
- `vcmp_core`：`CreateProcessW(CREATE_SUSPENDED)` → `VirtualAllocEx` + `WriteProcessMemory` 写入 dll 路径 → `CreateRemoteThread(LoadLibraryW)` 注入 → 返回 pid；`Drop` 时若注入失败则 `TerminateProcess`，成功则 `ResumeThread`。

## 6. 前端地图与约定

- **布局壳在 `App.vue`**：顺序为 `<Progressbar />`（顶部 3px 线）、`<header>`（`Switchbar` 标签导航）、`<main>`（`RouterView`）。页面级留白目前只有 `header{margin:0 8px}` 和 `main{margin:1rem}` —— **做排版时优先动这里**。
- **主题变量**定义在 `App.vue` 的 `:root` / `:root.dark`：`--bg-color`、`--main-color`、`--dark-main-color`、`--alpha-main-color`、`--dark-color`、`--color`。暗色由 `constants.ts` 读 `prefers-color-scheme` 给 `<html>` 加 `.dark` 决定。**新样式一律用这些变量，不要写死颜色**。
- **全局状态在 `constants.ts`**：`config`（`toRef(await getConfig())` + deep watch 自动 `setConfig`）、`internetServers`、`refreshInternetServers()`、`progressbar`、`router`、`routes`。
- **`Switchbar.vue` 的坑**：`active` 是内部 `ref`，`watch(active)` 时才 `router.push(path)`；它**不会**跟随路由反向同步，所以「刷新后高亮错位」是已知问题，改导航时注意。
- **`SvgIcon.vue`** 依赖 `vite-plugin-svg-icons` 把 `src/assets/svg/*.svg` 变成 `#icon-<name>`，并按 `removeAttrs(fill|stroke|class|style)` 清掉原色，靠 `color` prop 上色。
- **i18n 约定了 `${common.key}` 占位**：`locales/*.json` 会被自动 glob 加载，`common.json` 的内容注入到每个语言的 `common` 下，其它文件里的 `${common.x.y}` 会在启动时被替换成字面量。
- 视图现状：`Home.vue` 是**空壳**（只有注释）、`Internet.vue` 直接渲染 `ServerList` 并 `onMounted(refreshInternetServers)`、`Settings.vue` 是 5 个 `InputEdit` 的竖排、`About.vue` 有渐变标题 + Hero SVG 装饰、`Debug.vue` 是启动/下载的手动测试台。
- 组件风格：`<script setup lang="ts">` + `defineProps`/`defineEmits`，样式分散在 `App.vue` 全局和组件 `<style scoped>`，缩进 4 空格、单引号、尾随分号（见 `.prettierrc`）。

## 7. 坑与注意事项（重要）

1. **浏览器里跑前端必然报错**：`bridge.ts` 用 `@tauri-apps/api/core` 的 `invoke`，而 `constants.ts` 顶层就 `await getConfig()`，没有 Tauri 运行时就会 reject，页面白屏。要在浏览器看排版，需要临时 mock（例如给 `window.__TAURI_INTERNALS__` 打桩或临时改 `constants.ts`），**改完务必还原**。
2. **`vcmp_core` 必须 i686 + Windows**：`inner_launch_game` 在非 Windows 直接返回 `NotSupportedPlatform`，非 `target_arch = "x86"` 返回 `NotSupportedArch`。Linux 上只能 `cargo check`，不能真正启动游戏。
   `windows` crate 只在 `[target.'cfg(windows)'.dependencies]` 里引入，`launch.rs` 里调用它的那一行也必须带 `#[cfg(target_os = "windows")]`，否则非 Windows 上会报 `E0433`。
3. **默认 target 是 Windows，Linux 上检查要显式指定**：仓库根 `.cargo/config.toml` 与 `crates/vcmp_core/.cargo/config.toml` 都设了 windows target；
   在 Linux 上必须 `cargo check -p <crate> --target x86_64-unknown-linux-gnu` 才会按宿主机检查。
   `cargo check --workspace`（含 Tauri app）在 Linux 上仍会失败于 GTK/WebKit 的 pkg-config（glib/pango/gdk/cairo/atk、libdbus-sys），
   **这是环境限制、按约定忽略**，Windows 上正常。
   另外本沙箱 `/root/.cargo/registry` 只读，若 `cargo update` 后要下载新版 crate，需要 `CARGO_HOME` 指到可写目录（例如 `export CARGO_HOME=/tmp/cargo-home` 并先把 `/root/.cargo/{config.toml,registry}` 拷过去）。
4. **包管理器统一为 pnpm（2026-10-11 从 yarn 迁移）**：唯一 lockfile 是 `pnpm-lock.yaml`（lockfileVersion 9.0）；
   `yarn.lock` 与 `package-lock.json` 已删除（需要时可从 git 历史找回）。`package.json` 里钉了 `"packageManager": "pnpm@12.8.1"`。
   - `tauri.conf.json` 的 `beforeDevCommand`/`beforeBuildCommand` 与 `cli.py` 都改成了 `pnpm`。
   - **pnpm 10+ 默认拦截依赖的 postinstall**：本仓库用 `pnpm-workspace.yaml` 的 `allowBuilds: { esbuild: true }` 放行
     （pnpm 12 已不再读取 `package.json` 的 `pnpm` 字段，配置只能写在 `pnpm-workspace.yaml`；用 `pnpm approve-builds` 会让它自动写入）。
     没有这条时 `pnpm install` 会以 `ERR_PNPM_IGNORED_BUILDS` 结束（`pnpm build` 其实仍能成功，但 install 退出码是错的）。
   - 沙箱内 `/root/.local/share/pnpm/store` 存在但**无法硬链接**到工作区，pnpm 会自动降级到工作区内的
     `node_modules/.pnpm-store`（会打印一条 WARN，属正常）。**不要再退回 npm**：npm 全局缓存 `/root/.npm/_cacache` 只读会 `EROFS`。
5. **`ROOT` 是相对路径**：debug 用 `../../runner`，取决于进程工作目录。用 `cargo run`/`tauri dev` 从不同目录启动时，`runner/` 可能出现两份。
6. **`internet.rs` 里 `.unwrap()` 直接炸**：master 请求失败会 panic（代码里已标 `TODO: Cache this`）。
7. **`launch` 会阻塞式读子进程 stdout**，且 `resource.rs` 解压用 7z 无进度细粒度，改这两处要顺手考虑 UI 反馈。
8. `ScreenToSmall.vue` 只 `console.log`，没有实际拦截逻辑；`Home.vue` 是空壳 —— 排版工作大概率要从这两个地方展开。
9. **字体路径不对**：`index.html` 的 `@font-face` 指向 `/fonts/HarmonyOS_Sans_SC_Regular.ttf`，而字体实际在 `src/assets/fonts/`，构建时会警告且运行时回退系统字体。改排版若要统一字体，先修这里。
10. **原版界面参考在 `screenshots/origin/`**：VC-MP 0.4 Server Browser 截图（internet/official/history/favourite/settings/add server）。改排版前先看这几张，它们是目标形态的直观依据。
11. **文档约定**：每次改动要写进 `CHANGELOG.md`（精确到时间，含时区），会话上下文同步进 `AGENT-CONTEXT.md`。
12. **小心 rustfmt 把无关文件一起改掉**：本项目的 Rust 源码不是 rustfmt 格式（例如 `lib.rs` 的 `pub mod` 顺序、`constant.rs` 的缩进式注释）。
    但某些环境（IDE/扩展/工具）在文件变动后会自动 `cargo fmt` 式格式化，一次性改动十几个无关文件。**提交前务必 `git status` 复查**，
    无关的格式化用 `git checkout -- <path>` 还原，只保留真实改动。
