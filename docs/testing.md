# 测试

**面向本仓库的开发者。一条命令跑完测试：`npm run check`。** 它构建程序、检查仓库自身的规则、并跑测试，
用的是假的 Herdr 和按脚本演的假代理：不调用模型、不用真实看板、不用真实 Herdr。何时该跑什么，见
[AGENTS.md](../AGENTS.md)。本页讲这条命令怎么工作，以及为什么这样设计。

| 要跑什么 | 命令 |
|---|---|
| 全部 | `npm run check`，或 `npm run check -- --all` |
| 单个领域：它的测试文件（`tests/board_panel.rs`、`tests/board_panel_*.rs`） | `npm run check board-panel`、`npm run check gate sync` |
| 只跑名字里含某个词的测试 | `npm run check textbox` |
| 明明通过了，还要重跑 | 加 `--fresh` |
| 只跑某个提交以来的改动波及的（看板在修复轮的检查） | `npm run check -- --since <commit>` |

测试在 `tests/`，一个领域一个文件，共用部分在 `tests/common/`，全部属于同一个测试程序
（`tests/main.rs`，见[下文](#一个测试程序)）。单个函数的小测试就写在代码旁边（`#[cfg(test)]`，在
`src/` 和 `crates/` 里）。

## 一条命令

所有跑测试的东西都用同一条命令：你、干活的工作代理（worker）、评审者（reviewer），以及看板（board）
自己的检查（[`.omh.json`](../.omh.json) 里的 `check`）。

- `npm test` 是同一条命令。两者都跑 `scripts/check.sh`；除了名字，完全不用 Node 的东西。
- `.herdr-superwork.json` 说的也是同一条命令，是给在本仓库上干活的看板用的，直到它的机器都跑 omh
  （本程序以前的名字）为止：它只读那个名字。

**按名字挑测试。** 名字先当领域看：`board-panel`（或 `board_panel`）跑 `tests/board_panel.rs` 和所有
`tests/board_panel_*.rs`，就这些（不含 `web_board_panel.rs`）。不是领域的名字，就按 `cargo test` 的方式
匹配每个测试自己的名字：`tests/` 里的测试由文件名和函数名组成
（`board_hints::the_hint_line_names_the_task_once_…`），代码旁的测试用模块路径。所以
`npm run check textbox` 跑所有名字里带 `textbox` 的测试，无论在哪。匹配不到任何测试的词，一个都不跑，
直接通过。

**它做什么**，按这个顺序，第一个失败就停：

1. **构建程序**，用 `quick` profile（`Cargo.toml`）：程序自己的代码不优化、不带调试数据，小改动后很快
   重新构建；它用到的库稍作优化（只构建一次，见[下文](#所有工作树共享一个构建缓存)）。给人安装的文件用
   release build，是另一个 profile（见[安装](install.md)）。
2. **写入 made 表**，过期时才写（`omh docs`）：`docs/keys.md` 里的按键、`docs/settings.md` 里的设置，
   以及你自己文件夹里的 map。它会说写了哪些：把这些表提交掉。
3. **检查仓库自身的规则**（`omh docs --guard`，`tests/guard.rs` 也会跑）：源文件不超过 300 行；每个源文件
   开头有一句注释说明内容；按键的 chord 只写在 `crates/flow/src/board/keys.rs`；没有过期的 made 表。
4. **跑测试**，用你给的名字，多进程并行（`scripts/run-tests.sh`，见[下文](#多进程并行)）。

它经由所有工作树（worktree）共享的构建缓存编译，并占用它那份 CPU 核心：单独跑时占全部，两个一起跑各占
一半（都见下文）。

旧程序的检查得找出用了却没定义的名字，以及一个改动经 import 波及哪些测试文件。前者现在由编译器负责
（这种文件根本编不过），而且构建是一个整体：没有分开的测试文件可挑。

### 记住通过的运行

一次通过的运行会按它通过时的精确代码记住：文件夹当前的 git tree（含未提交的改动和新文件）、编译器完整
版本（`rustc -V`）、以及给它的名字。对同样的东西再问一次，它立刻回答
（`The check already passed for this exact code.`，即"这份代码已经检查通过了"），什么也不跑。

- 结果存在工作树之外（`~/.cache/omh-check`），所有工作树共享：评审者和看板的检查在工作代理的提交上，
  直接复用工作代理那次通过的运行。
- 提交不改动任何文件，所以不改变这里的判定。任何编辑都会。
- 失败的运行从不记录。
- 带名字的运行只按那次运行记：`npm run check textbox` 通过不代表整体通过。
- `--fresh` 无论如何都跑。

### 修复轮的更短检查

任务的后续轮次（评审（review）或检查失败把它打回来）中，看板跑 `npm run check -- --since <commit>`
而不是完整检查，此时 GitHub 的检查仍在合并（merge）前把关：[`.omh.json`](../.omh.json) 里的
`check.again`，看板何时这样做见[修复轮的更短检查](shorter-checks.md)。那个提交是完整检查最后一次通过的
提交。

它像完整检查一样构建、写 made 表、查规则。然后跑代码旁的每个测试（和文档测试），以及那个提交以来的改动
波及的测试文件：之后提交的、还没提交的、新文件都算。`scripts/changed-tests.sh <commit>` 挑选它们，
对每个改动的文件：

| 改动 | 波及的测试文件 |
|---|---|
| 一个测试文件（`tests/sync.rs`） | 那个文件 |
| 测试文件共用的模块（`tests/far/`） | 用到它的文件（`crate::far`） |
| 一个留存的 frame（`tests/frames/`） | `board_frames` |
| 程序的一个文件（`crates/flow/src/sync/conflict.rs`） | 名字里含它的名字或所在文件夹名的（`sync_conflict`、`sync`、`sync_behind`），以及写了它模块路径的（`sync::conflict`）；`mod.rs` 按所在文件夹算 |
| 一个文档页（`docs/keys.md`、`README.md`） | 写了它名字的，以及 `guard` |

**跑全部测试**（它会说原因：`Every test runs: …`，即"全部测试都跑：……"），当它无法判断时：
`tests/main.rs`、`tests/common/` 和 `tests/fixtures/`（每个测试都用）；`src/lib.rs`、`src/main.rs` 和各
crate 的根文件（`src/sw_*.rs`）；没有任何测试文件以之命名或写出其路径的程序文件；其他任何文件
（`Cargo.toml`、`crates/`、`scripts/`、`.github/`）；不在分支上的提交；以及反正会波及所有测试文件的改动。

这按名字挑，不按测试实际跑什么挑：改动可能弄坏它没挑中的测试。所以看板只在 GitHub 会在合并前对 PR 跑
完整检查的地方用它。

- 通过的更短检查按它自身记录（连同它的提交）。
- 跑了全部测试的那次，也算完整检查。
- 这份代码上完整检查已通过，就立刻回答。

### 一个测试程序

`tests/` 里每个文件都是同一个测试程序 `tests/main.rs` 的一个模块（`Cargo.toml` 里 `autotests = false`），
Rust 的常规做法。Cargo 只编译、链接测试一次。（每文件一个程序曾是 191 次编译，每次重新链接整个库，
每个工作树里约 2.3 GB 的程序。）

- 测试文件仍是一个领域一个文件。
- 文件共用的东西（`common`、`far`、`review_kit`、`move_home`）在 `tests/main.rs` 里声明一次。文件里用
  `use crate::common;` 引到它。
- **新测试文件就是 `tests/main.rs` 里的一行 `mod`**，放在列表里它该在的位置。漏掉的文件会被 guard 判
  失败——那样它永远不会被构建或运行。

所有测试在同一个进程里并排跑，所以测试对进程的任何改动都是大家的：环境（world 排队轮流用，见下文；
没有 world 却设置环境的测试走同一个队列，`common::one_at_a_time()`）和工作文件夹（world 在被删掉前先
离开它的文件夹）。

### 多进程并行

大多数测试会造一个 world，而在一个进程里 world 要排队：cargo 直接跑单个测试程序约需 90 秒，4 线程还是
16 线程都一样。所以检查用多个进程并行跑（`scripts/run-tests.sh`）：

1. 构建测试程序（`cargo test --no-run`）；
2. 列出各自包含的、被名字挑中的测试；
3. 轮流分给各进程，全部跑完，报告通过了多少。

失败的进程整段打印，像 cargo test 那样。每个进程有自己的 world（world 的文件夹以进程命名），排队只发生
在进程内部。

**开多少个：** 测试开始时按核心份额的两倍（见[下文](#它分到的核心)），因为测试排队跑的进程大约只用半个
核心。16 核上单跑一次检查，一个测试程序开 32 个进程：测试只要 15 到 20 秒，对比旧的固定 2 核份额下的
44 到 56 秒。`OMH_TEST_PROCS` 可设别的数（1：全部塞进一个进程，和以前一样）。

### 构建轮流进行

Cargo 同一时间只让一个构建用某个文件夹的 `target/`：第二个会说 `Blocking waiting for file lock`
（阻塞，等待文件锁），等第一个完成再继续。每个工作树有自己的 `target/`，所以这只发生在同一文件夹内。
没有别的排队、没有槽位：一次运行就是一次构建加一次跑测试。

### 所有工作树共享一个构建缓存

程序用的库在每个工作树里都一样，所以每台机器只编译一次，放进一个缓存：
[sccache](https://github.com/mozilla/sccache)，固定一个版本，对照发布的 sha256 校验。

- 工作区 setup（`scripts/setup.sh`、`.omh.json` 里的 `workspace.setup`）用 `scripts/build-cache.sh`
  装它并经它构建。缓存在时，检查也经它编译。
- 它住在 `~/.cache/omh`，带着缓存（最多 2 GB，先丢最旧的）和一个所有工作树共用的 server（socket 放在
  那里，你另开的 sccache 不受影响）。只有没 server 应答时检查才启动它，一次只启一个：
  `sccache --start-server` 会在运行中的那个旁边再启一个并抢走它的 socket，以前这曾给每个检查启一个
  自己的 server。这在所有工作树之外，所以删掉工作树（`workspace.cleanup`）只释放那个工作树的 `target/`，
  共享的东西不受影响。
- 没装的地方（CI，它有自己的缓存；别的平台；没网络），构建照旧，各工作树独立。
- `OMH_NO_CACHE=1` 关掉它。你自己设的 `RUSTC_WRAPPER` 优先。
- **命中了吗？** `sh scripts/build-cache.sh --stats`：server 自启动以来的计数（空闲十分钟后退出，计数
  随之消失）。每次构建在自己的进程里编译（`SCCACHE_CLIENT_SIDE`），所以 `Compile requests` 恒为 0，
  别看它：看 `Cache hits (Rust)` 对 `Cache misses (Rust)`。空 `target/` 里全新构建一次，实测 101 命中、
  15 未命中（程序自己的 crate，以及编译器宏）。
- **任何随运行而变的东西都不要放进 `CARGO_` 变量。** Cargo 把所有以 `CARGO_` 开头的变量都递给编译器，
  sccache 以它们为每次编译的 key：同一构建，设与不设 `CARGO_BUILD_JOBS`，实测 0 命中对 101 命中。所以
  检查把核心份额用 `-j` 传给 cargo，绝不用那个变量。
- 装了 mold 的地方构建用 mold 链接（见[链接](linking.md)）。

它留下的是任务不会改动的东西：库。新工作树发现大部分都在。少数要重新构建，因为它们把代码写进该工作树
自己的 `target/`（它们的 build script），或是编译器宏——sccache 从不留它：几秒钟。程序自己的代码不共享：
每个任务都不同，而且编译器在各 `target/` 里给它记自己的笔记（增量编译），小改动后的下一次构建因此很快；
缓存拿不走这些。

曾考虑共享 `target/` 代替缓存，但放弃了：cargo 同一时间只让一个构建进 `target/`，任务会互相等；而且它为
测试构建的程序是那里的一个固定位置的一个文件，两个任务会在对方的测试底下互相覆盖。

### 新工作树热启动

缓存留库，不留程序自己的代码，光构建它约需 35 秒。所以 setup 不让新工作树从零构建：
`scripts/warm-start.sh` 拷入主线最近留存的构建（`target/quick`），只重建此后改动的部分。

- **留哪个构建：** 构建后，如果一个工作树的提交在主线上、比留存的构建新，且构建前和构建中都没有改动，
  就把自己这次构建留作下一个的起点。读不到编译器的版本（`rustc -vV` 失败）就什么也不留、也不取别人的：
  它的话是留存构建的印记，宁可全无。
- **实测** 在 16 核机器上同时十个代理（每个构建两核）：新工作树的 setup 以前要 37 秒，现在略超 1 秒。
  主线前进后的第一个工作树凭编译器的笔记重新构建程序，约 15 秒，并把这次构建留给之后的。
- **在哪：** `~/.cache/omh/warm/<repo>/`，一个仓库一个文件夹，`current` 指向最新的。新的先建在旧的旁边
  再改名，所以同一时刻 setup 的工作树各拷一份完整构建。旧的在不再是 current 后 30 分钟删掉。
- **怎么拷：** 文件系统支持就用 clone（APFS、Btrfs、XFS：不占额外空间），其他地方用普通拷贝（ext4：
  一份留存构建约 500 MB）；绝不用硬链接，因为 cargo 会就地写其中一些文件。
- **时间戳：** cargo 按文件改动时间判断过期，而新工作树的每个文件都是新的。内容与留存构建的提交一致的
  文件给旧时间，cargo 就当已构建；其余（此后主线上改过的，或文件夹里改过的）保持原时间，重新构建。
- **文件夹路径：** cargo 还会把写入了构建时所在文件夹的东西当已构建（`env!("CARGO_MANIFEST_DIR")`），
  在这会是另一个工作树。拷贝会重建这一部分；程序在运行时（`docs::root`）而不是构建时找到自己的仓库。
- **crate：** 程序由多个 crate 组成（见[各 crate](crates.md)），各自代码在自己的文件夹，cargo 以 crate
  的文件夹命名那段代码：自留存构建以来一个文件都没改的 crate 整个当已构建。
- **不拷的：** 另一个编译器构建的，或已超过 2 GB 的（cargo 从不删旧版本的库）。那个工作树从零构建，
  改留它的构建。
- `OMH_NO_WARM=1` 关掉它。

### 它分到的核心

检查的构建和测试占用机器的核心，在当下正跑的检查之间分：单跑的检查拿全部，两个各拿一半（向上取整），
最少一个。工作区 setup 的构建也算一个。以前是核心数除以 `board.agents`，不管实际跑几个：16 核机器上
十个代理就是两核，单独跑的检查也一样。

- **谁在跑：** 每个运行中的检查（和 setup）在 `~/.cache/omh/running/` 写一个文件，以进程命名，记该进程
  何时启动，结束时删掉。被杀掉的会留下文件：进程已消失、或进程号现在属于另一个启动时间不同的进程的
  文件，不计数，并由下一个来查的检查删掉。
- **何时计数：** 每次构建前，以及测试开始时（`scripts/share-cores.sh`、`sc_share`），所以独自启动、到
  测试时才发现别的检查在跑的，从那时起拿一半；和别人一起启动的，等对方完成后又拿全部。已在跑的构建
  保持它启动时的份额。
- **用在哪：** cargo 以 `-j` 拿到它（不是 `CARGO_BUILD_JOBS`：[构建缓存](#所有工作树共享一个构建缓存)
  以它为 key），测试拿到 `RUST_TEST_THREADS` 和两倍数量的进程（见[上文](#多进程并行)）。
- **你的优先：** 自己设了 `CARGO_BUILD_JOBS` 或 `RUST_TEST_THREADS` 就用它代替份额（cargo 仍以 `-j`
  拿到）。`OMH_TEST_PROCS` 设测试进程数。
- **实测** 在 vm-str（16 核，旁边有其他代理干活，没有别的检查在跑），各跑三次，旧的 2 核份额对全部
  16 核：

  | | 2 核（以前） | 16 核（现在） |
  |---|---|---|
  | 改 `crates/core` 后的完整检查（每个 crate 都重建） | 56–68 s | 26–30 s |
  | 光跑测试 | 44–56 s | 15–20 s |
  | 空 `target/` 里构建测试程序（新工作树的第一次检查） | 76–77 s | 56–61 s |

  构建收益最小：crate 一个叠一个地构建，测试程序是一个大 crate。

提前停下的检查会在 stderr 说明原因（`The check stopped while building the program (exit 101).`，即
"检查在构建程序时停下（exit 101）"），即使让它停的那步什么也没说。`tests/check_script.rs` 在空 home、
无构建缓存的环境里启动检查，保证全新机器总能过启动这一关；还在三核、有别的检查（活着的和死了的）在跑的
机器上跑，看它分给 cargo 和测试的份额。

### 它向看板报告进度

看板跑检查时，会在 `OMH_PROGRESS` 里指定一个文件，命令把进度写进去：三步走完了几步（`1/3 steps`），
或者答案已直接得出。看板在任务上显示它：[检查进度](check-progress.md)。

### 没有时间预算

测试和运行都没有时间预算，也不拿钟表衡量。旧检查有按文件的预算，还降低自身优先级，因为每个测试文件都
是自己的程序、还会再启程序。这里的测试只是测试程序里的一个函数，轻量的什么都不启动。所以：绝不等机器
空闲，绝不为赶时间砍测试。

## 轻量测试

让测试慢的是启动程序和等待。新测试两样都不做：

- **它直接调代码**，在测试自己的进程里。`tests/common/mod.rs` 里的 `world()` 给它自己的看板文件夹、
  一个 git 仓库和一个假 Herdr。看板的代码仍然拼同样的 Herdr 命令、读同样的应答和退出状态，但不启动任何
  程序：`herdr::cli::set_fake` 把调用交给假的，它在测试自己的线程里应答。`gh` 同样被假掉
  （`flow::git::set_fake_gh`、`tests/common/gh.rs`）。
- **它不 sleep，也不依赖机器快慢。** `world()` 把代码自身的等待设为零。时钟是 `sw::clock`：`set` 把它
  定在某刻，`advance` 推进它，只在测试线程上。过去或未来的时刻写进数据，不是等出来的。没有东西拿实测
  时长做比较。
- **看板的后台作业由测试说跑才跑。** 本来会是分离 runner 的东西（派发（delegate）、收尾流水线
  （finish line）、合并）放进一个列表（`OMH_INLINE`）。`settle(id)` 跑那个列表，然后让钩子（hook）一轮轮
  看代理的面板（pane），直到没有变化。`settle_all(id)` 对一个同时完成多次的任务做同样的事。
- **一个测试内部共享 setup**，而不是为每个小断言造新 world：world 就是一个 git 仓库加首次提交。
  `to_approval(&w, title)` 是收尾流水线测试大多从这幕开始的场景。
- **git 是唯一仍会启动的程序**，因为看板的活就是 git 的活。每个事实只问一次。

world 会设置进程的环境，所以造 world 的测试要排队，在所有文件里都是（`tests/common/mod.rs` 里一把锁）；
其他测试与它们并行跑。检查开多个进程绕过这一点（见上文）。

**看板必须等的东西，就是测试可以调短的数。** 看板自身的等待在 world 里为零，测真实程序时给它需要的短值
（`OMH_STAMP_MS`：打开的看板隔多久查一次自身新版本；`OMH_TICK_MS`：运行中的检查隔多久被看一次）。对会
报告完成的东西绝不等固定时间：真实终端读到它的画面画完为止。

### 替身

- **假 Herdr**（`tests/common/herdr.rs`，连同 `panes.rs`、`agents.rs`、`worktrees.rs`）：它的整个世界是
  一条 JSON 记录（空间（space）、标签页（tab）、面板、其中的代理、被要求的每次调用），测试读
  （`w.state()`、`calls(&w)`）或改（`w.patch`、`set_status`）。它的工作树用 git 本身造。
- **按脚本演的代理**（`tests/common/agents.rs`）：测试给每个角色（role）安排回合（turn），
  `w.script(json!({"worker": [step, …], "reviewer": […], "picker": […], "coordinator": […]}))`。每条
  提示词（prompt）演该角色的下一步：写文件、提交，并把收尾小结（closing note）、计划、结论或对比写到
  看板指定代理写的地方。一步可以用一条录下的真实代理会话结尾（`tests/fixtures/endings`、
  `tests/common/endings.rs`）结束它的回合，并跑代理自己的回合末报告。
- **看板作业之间写的小结。** 真实代理在看板安排好它要结束的回合之后写收尾小结。假代理在看板的作业之间
  写下已结束各回合的小结（`flush()`，`settle` 会调它），时间戳盖在那之后，不管跑测试的机器多慢。
- **每个画面测试画的行**（`tests/common/fixtures.rs`）：看板展示的每种状态各一个任务，配上会撑破布局的
  长文本、宽文本和古怪文本。新状态就是那里新增一行，每个画面测试都画它。

## 四类测试

- **看板状态的更新。** `common::board::open(&w.root, rows)` 在一个 world 上开真实看板，不用终端。
  `press(&["alt+t", "esc", "ctrl+l"])` 和 `typed("…")` 走看板唯一的 `update`，它们启动的东西立刻运行，
  `text()`、`screen(cols)`、`chat(cols)`、`draft(cols)` 把画面读成纯文本。大多数看板测试是这类。
- **整幅 frame**（`tests/board_frames.rs`）：看板由终端绘制器画进格子，从 10×8 到 200×60 共十种尺寸，
  每种含列表、旁边的任务、完整任务、`?` 菜单和草稿。每幅与 `tests/frames/` 里留存的 frame 对比，一次比
  文本，一次把每格颜色当一个字母比（带图例）。缺的 frame 会写入；`UPDATE_FRAMES=1` 全部重写，用于改动
  看板外观的情况。提交前先看改了什么。每幅 frame 还要守不需要图片的规则：没有行超宽，提示行在最后。
- **给网页的看板数据**（`tests/web_board.rs`）：`common::board::web(&w, call, args)` 在同一个 world 上问
  网页看板页面会问的东西：和终端看板同样的列表、分组、标记和文案，它的按钮按的是看板自己的按键。
- **真实终端**，少数几个：`tests/real_terminal.rs` 在伪终端里启动构建好的程序，像终端一样驱动它：启动并
  绘制，接住一串按键，干净地调整大小，关闭时把终端原样交还。

**测试绝不读写你的真实看板。** `world()` 给它自己的 home 文件夹（`HOME`、`OMH_HOME`），把 Herdr 自己的
变量（面板的 id、它的 socket）、`TMUX`、`NO_COLOR` 从环境里拿掉，并把 Herdr 指向它自己的 socket。不造
world 的测试也有临时看板：`scripts/run-tests.sh` 把 `OMH_HOME` 设为本次运行自己的文件夹，并拿掉
`OMH_DATA_DIR`、`OMH_CONFIG_DIR`、`OMH_REPORTS_DIR`，还把 `RUSTUP_HOME` 钉在真实的工具链上：world 改掉
`HOME` 之后，并行的测试若在那一刻运行 `rustc`，rustup 的代理会把 world 的临时文件夹当自己的家、往里下载
一套工具链，被 world 的删除追上、半途而废。它还设 `OMH_TESTING`：在其下，store 拒绝真实的
`~/.omh`（从系统用户列表查到，不管 `HOME` 怎么说；经链接也一样），并在任何读写发生前就停住测试，给出
该怎么做（`crates/core/src/store/never_real.rs`）的提示。

手动跑这个程序、或跑你基于它代码构建的程序时，先设 `OMH_HOME` 到临时文件夹：新版本可能会转换它在真实
文件夹里找到的东西。一个变量就够，在看板开的标签页里也一样：看板把它启动的东西指向自己的 home 文件夹，
而不是其中的文件夹（只有一样留在别处：见[主文件夹](home-folder.md)）。绝不自己往真实看板的文件夹里写：
有个评审者的探针曾把假任务（`probe/12`）留在了人类的看板上，因为它继承的一个变量把真实数据文件夹指到了
它的临时 `OMH_HOME` 之外。

**只在程序本身是被测对象时才启动真实程序。** 这类测试在名字里写明（`real_programs_…`）：命令行
（`tests/cli.rs`）、另一台机器的入口（`tests/remote.rs`）、真实终端（`tests/real_terminal.rs`）。它们在
测试的 world 里跑 cargo 为测试构建的程序，无 Herdr 可达。仓库自己的 `setup`、`check`、`teardown` 命令
本就是 shell 命令，在相关测试里也作为 shell 命令运行。

## 在 macOS 上也能跑

GitHub 在 Linux 上跑完整检查，在 macOS 上跑那些测试有差异的领域（`.github/workflows/ci.yml`；完整检查在
那也通过了）。测试在两边都成立，靠的是问两边都有的东西：

- 进程状态取自 `ps -o stat= -p <pid>`，不是 `/proc`（macOS 没有）；
- 锁在测试里用 `libc::flock`，不是 `flock` 命令（macOS 没有）：flock 属于文件的一次打开，所以测试里重新
  打开该文件，会像别的程序一样发现它被持有；
- `wc -l` 的计数要修剪（macOS 会拿空格补齐）；
- 临时文件夹取真实路径（`canonicalize`），凡是要拿脚本的 `pwd` 与它比的地方（macOS 的在 `/var` 下，
  即 `/private/var`）；
- 本机名按看板显示的那样，在过长处截断（GitHub 的 macOS runner 名字超过 60 个字符）；
- 拷贝系统程序只在能跑它的地方：Mac 可能杀掉它，或在它重新签名后杀掉它。

这类新领域进 macOS job 的列表。

## 看真实效果

测试把看板读成文本和 frame。真实终端或浏览器截图只用于改变看板或评审页外观的任务。其他任何任务，测试
就是证据。
