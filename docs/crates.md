# Crates 分层

**程序是五个 crate，一层叠一层。** 改动只重建它自己的 crate 和它上面的那些。

| crate | 文件夹 | 里面装什么 |
|---|---|---|
| sw-core | `crates/core/` | 数据（`store/`）、Herdr 与各代理、以及各处都用的小工具 |
| sw-flow | `crates/flow/` | 任务的流程：派发、收尾流水线、落地、看守、其他机器、自动化 |
| sw-board | `crates/board/` | 看板的屏幕（`board/`）：它的状态、按键做什么、画出什么 |
| sw-web | `crates/web/` | web UI 里的看板，Ovis（`webui/`）：前门和 web 看板 |
| omh | `Cargo.toml`、`src/` | 命令行（`cli/`）和生成页面（`docs.rs`）；库是 `sw` |

为什么这么分：编译器在任何编辑后都要整个 crate 重读一遍（测试再读一遍）。当它还是一个 27,000 行的单 crate 时，每次编辑，哪怕看板屏幕上的一行，都要付全部代价。

多数改动是看板的屏幕和 web 看板。这些重建 sw-board、sw-web 和命令行，sw-core 和 sw-flow 保持原样。改 sw-core 会重建全部五个。

## 新模块放哪

**放在用到其代码的最底层 crate**：在那个 crate 的 `src/lib.rs` 里加一行 `pub mod`。文件夹里的新文件是在那个文件夹的 `mod.rs` 里加一行。

模块在每个 crate 里路径相同。每个 crate 的 `src/lib.rs` 点名自己的模块（`pub mod store;`），并把下层 crate 的模块透传成自己的（`pub use sw_core::*;`）。所以模块在每个 crate 里是 `crate::store`，在测试里是 `sw::store`。把模块移到别的 crate 不会改变任何使用它的路径。

## 一个 crate 只用它下面的 crate

编译器强制这一点：sw-flow 里的代码写 `crate::board::view` 编译不过。当下层代码需要上层的东西时，要么把它移下去，要么让上面的调用者递进来。

- **移下去的。**
  - 按键表（`crates/flow/src/board/keys.rs`）和主题（`crates/flow/src/board/theme/`）建在 sw-flow 里，是它自己的 `board` 模块：流程的消息会点名某个按键，评审页要用看板的颜色。sw-board 的 `board` 模块把它们透传出去，所以到处都是 `crate::board::keys`。
  - 流程和看板屏幕用到的 web UI 部分（`crates/flow/src/webui/`）同理：它在哪（`find.rs`）、它的 PCs（`pcs.rs`）、它的链接（`link.rs`、`command.rs`）、把请求转发出去（`proxy.rs`）、从外部启动的前门（`keep.rs`）。前门自己那个服务 web 看板的循环，在 sw-web 的 `door_loop.rs`。
- **递进来的。** 页面服务器（`review_page::serve_main`，sw-flow）也是 web UI（Ovis）的前门。命令行把前门的循环（`webui::door_loop::run_door`，sw-web）递给它。

## 新的工作树

每个 crate 的代码在自己的文件夹里，所以 cargo 靠那个文件夹内部的路径来跟踪一个 crate 是用什么建的，在每个工作树里都一样。新的工作树（worktree）从主线的温构建开始（[测试](testing.md)）。在那里，从那次构建以来没有文件改动的 crate 算作已建好：只有改动碰到的 crate 会被重建。

共享构建缓存存的是库，不存这些 crate。编译器对它们的笔记（增量编译）存在各自的 `target/` 里，缓存没法共享。

## 省下多少

在 vm-str 上实测（16 核，同时有其他任务在构建），本任务占两个核，对每个 crate 的一个文件做一行改动（三次运行取中位数；新工作树只跑一次）：

| 改动在哪 | 程序重建，前 → 后 | 从温构建起新工作树，前 → 后 |
|---|---|---|
| 看板的屏幕（sw-board） | 2.7 s → 1.5 s | 12.8 s → 4.8 s |
| web 看板（sw-web） | 2.9 s → 1.1 s | 13.1 s → 2.6 s |
| 命令行（`src/`） | 2.8 s → 0.7 s | 13.5 s → 1.5 s |
| 流程（sw-flow） | 2.8 s → 2.4 s | 12.5 s → 10.7 s |
| 数据（sw-core） | 2.7 s → 3.2 s | 12.9 s → 13.1 s |

从零构建差不多（33 s → 30 s）。测试程序也一样（改动后 4 s，新工作树 15 s）：它是自己单独的一个 crate，`tests/main.rs`，它下面任何改动后都会重建。
