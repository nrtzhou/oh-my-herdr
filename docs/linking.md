# 链接

**在 Linux 上，本仓库的构建用 [mold](https://github.com/rui314/mold) 链接，它是现有的最快链接器。**没装 mold 的地方，按 Rust 自己的方式链接。构建要跑起来不需要你装任何东西。

为什么重要：每次编辑后的构建都要重新链接程序和测试程序，两者都带着整个库。mold 把链接时间砍半，每次构建约省 0.4 秒。

`OMH_NO_MOLD=1` 关掉 mold。开关它不会触发重新构建。

## 工作原理

- **flag。**`.cargo/config.toml` 给 Linux 构建（x86_64 和 aarch64）加 `-Clink-arg=-Bscripts/ld/`。于是 C 编译器把 `scripts/ld/ld` 当作它的 `ld` 来运行，而那是指向 `scripts/linker.sh` 的链接。仓库里每次构建都如此：检查、工作区（workspace）设置、你自己的 `cargo build` 或 `cargo test`。
- **x86_64 Linux** 还会加 `-Clinker-features=-lld`。那里 Rust 默认向 C 编译器要它自己的 lld，这个 flag 阻止它。
- **ARM Linux（aarch64）只加 `-B` flag。**`-Clinker-features` 在那里不稳定（macOS 也是；x86_64 Linux 上是稳定的）。带上它，stable Rust 上每个构建都会立刻停下，说该 flag 在 `aarch64-unknown-linux-gnu` 目标上不稳定。那里也不需要它：Rust 在那个目标上不要它的 lld，所以 C 编译器照样跑 `scripts/ld/`，同样用 mold 链接。
- **脚本。**`scripts/linker.sh` 运行 mold：固定一个版本，对照其发布的 sha256 校验，与共享构建缓存一起放在 `~/.cache/omh`。工作区设置负责安装它（`sh scripts/linker.sh --install`，在 `scripts/setup.sh` 里）。
- **mold 用满所有核**，和 Rust 的 lld 一样。链接只要零点几秒。在本任务分到的那几核上（[测试](testing.md)），mold 收益不大。
- **没装 mold 的地方**（GitHub CI、全新机器、没有网络、没有 mold 构建的平台），脚本改用 Rust 本来会用的链接器：x86_64 上是它自己的 lld（`rust-lld`，Rust 把它放在链接器 PATH 最前），其他地方（aarch64）是系统的 `ld`。
- **发布构建照旧链接。**产物放进 `release` 目录的链接（`cargo build --release`，带 `--target` 的也算）用同样的默认链接器。发布文件是为 musl 目标构建的，`.cargo/config.toml` 没提它们。
- **macOS** 保留 Apple 自己的链接器（ld-prime，Xcode 15 起的默认）。它在那里和 lld 一样快，而 mold 没有 macOS 构建。

## 测试守住每个平台

`tests/linking.rs` 用配置给每个平台的 flag 运行 `rustc --print cfg`：x86_64 和 aarch64 Linux（gnu 和 musl）、macOS、以及配置点名的每个目标。rustc 拒绝它们、或其中出现 `-Z` flag 时测试失败。rustc 在需要该目标的库之前就会抱怨，所以任何机器（包括 CI）上破坏 ARM Linux 的配置都会被这个测试抓住。

## 为什么用 flag，而不是在配置里指定链接器

Cargo 按构建时的设置记账。在配置里指定链接器（`linker = "scripts/linker.sh"`）会带上该工作树的完整路径。于是从另一个工作树复制来的构建（热启动，[测试](testing.md)）和共享构建缓存（按编译参数做键）里，每个库都会显得过期。那两个 flag 在每个工作树里都是一模一样的字，谁也不会注意它们。

`-B` 目录是相对路径，C 编译器从编译运行的位置去找它：对程序和它的测试来说就是仓库本身。库的构建脚本和宏在它们自己的目录里编译，那里没有 `scripts/ld/`。它们用系统的 `ld` 链接：小程序，只构建一次。

## 省下多少

在 vm-str 上实测（16 核，Rust 1.98.1，同时有其他任务在构建），对 `src/lib.rs` 改一行后，按检查的方式构建（程序，然后测试程序），各跑六次。每格是一次链接的时间范围：

| 链接 | Rust 自带的 lld（之前） | mold 用检查的 2 核 | mold 用全部核（发布时如此） |
|---|---|---|---|
| 程序（39 MB） | 0.22–0.27 s | 0.16–0.20 s | 0.09–0.12 s |
| 测试程序（`tests/main.rs`） | 0.37–0.45 s | 0.28–0.33 s | 0.14–0.19 s |
| 库自己的测试 | 0.12–0.16 s | 0.06–0.10 s | 0.05–0.09 s |
| 带调试数据的程序（`dev`，134 MB） | 0.39–0.51 s | — | 0.17–0.22 s |
| **一次编辑后的全部链接** | **约 0.8 s** | 约 0.6 s | **约 0.4 s** |

mold 链接时间约减半。整个构建却几乎不变（程序约 2.7 秒，测试再多 6 秒，各次运行的波动和收益一样大）：编译占了几乎全部，因为 Rust 的 lld 本来就快。
