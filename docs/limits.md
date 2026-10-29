# 限额：时间与 token

**两个设置给任务的花费封顶。** 触到其一，代理被停下，工作保留，任务等你。什么都不合并。两者默认都是 `none`，不设就一切照旧。

| 设置 | 值 | 含义 |
|---|---|---|
| `limit.time` | 时间（`2h`、`45m`、`90s`）或 `none` | 代理一次运行的最长时间 |
| `limit.tokens` | 数量（`2m`、`500k`、`1.5m`）或 `none` | 一个任务总共最多用多少 token |

```
omh set limit.time 2h                      # for every task of yours
omh set limit.tokens 2m --repo my-app      # for one repo
omh set limit.time 45m --task my-app/12    # for one task, also while it is under way
omh delegate 12 --limit.time 2h --limit.tokens 5m
omh add "Tidy the docs" --limit.time 1h     # with or without --delegate
omh set limit.tokens 1m --automation "Nightly look"
omh unset limit.time --task my-app/12
```

也可以吩咐协调者：“这个最多两小时”“一个任务最多两百万 token”。

它们和其他设置一样（[设置](settings.md)）：

- 就近取值：任务 → 你对仓库的条目 → 你 → 内置；
- `show` 和 `show <id>` 列出它们和值的来源，任务与自动化的设置页也有；
- 被拒的值不改任何东西，并给出最接近的正确值（`2 hours`：“Did you mean 2h?”）；
- **仓库自己的 `.omh.json` 放不了它们**，所以代理写的代码既不能设限额，也不能解除限额。

## 触到限额时会怎样

- 代理被停下：回合被打断，和停止键一样。评审者的标签页关闭。
- **工作原样保留**：工作树、分支、会话记录都在。不为此提交、推送或丢弃任何东西。
- **什么都不合并**，也不为它启动检查或评审。
- 任务在“需要你”下等你，附一行大白话：`stopped at its 2h limit`，或 `stopped at 2m tokens`。Herdr 侧栏同样显示，`board.notify` 通知你（“需要你”），等 `needs-you` 的自动化会触发。
- 不会自己发生第二次：停下的任务在有人继续前不受任何限额检查。

token 在哪里耗尽决定停在哪：

- **在代理回合结束时**：任务停在这里，在检查和评审之前。
- **在结论会退回代理的评审结束时**：结论留在任务上，不会自己退回去。
- **在通过的评审结束时**：任务像任何触到限额的任务一样停在“需要你”，评审成果保留。它不会进入普通待批准，也**不**合并：不自己合（`merge.by itself`），也不是你批准过、看板正在修复的合并。
- **在无物可落的评审结束时**（报告、仓库外的工作）：同样停下，而不是算完成。继续推进才结束它。
- **被服务商掐断的回合**，代理的或评审者的，在触到限额的任务上不会自己重试。

## 如何继续

| 你想 | 做 |
|---|---|
| 继续 | 任务上按 `Ctrl+R`，或 `omh retry <id>`，或用自己的话回复代理 |
| 给它更多 | 设更高的限额（或 `none`）：`omh set limit.time 4h --task <id>` |
| 就这样收下工作 | `Ctrl+L`，或 `omh finish <id>` |

- **继续**由你开口，任务重新拥有它的限额。停下的代理被叫停处继续；这次运行从零计时，token 从原处接着数。被停的评审重做。结论被留住的评审退回代理。完成并评审过的工作按当时的样子提交你批准（`merge.by` 说合就自己合）。
- **更高的限额**几秒内自己继续，不用问谁。计数接着走：2h 时停、给到 4h 的运行还剩两小时；2m token 时停、给到 3m 的任务还剩一百万。（这是 `limit.time` 唯一被停运行本身采纳的改动。）
- **就这样收下**把停下的代理的工作像完成的回合一样送过检查和评审。之后它是普通任务：`merge.by` 决定怎么落地。

自己停任务（`Ctrl+X`、`omh stop <id>`）会去掉限额标记：它只是停了。

## 看板显示什么

- **任务面板**：代理带着限额跑任务时，Status 下有 `Time 1h12m of 2h, this run` 和 `Tokens 1.2m of 2m`。从限额的五分之四起变琥珀色。Settings 下：`Limits 2h a run · 2m tokens`。Session 全屏：`Used so far`，该任务计的 token（一旦为它设过限额）。
- **行**从五分之四起显示，在代理在做的事之前：`1h40m of its 2h · Running the tests`。
- **停下的任务**有专属一块“Stopped at a limit”：什么停了它、用了多少、三条出路。
- **`details <id>`** 有 `limits` 条目：每项限额、对它用了多少，以及若有、那次停止。**`show <id>`** 列出两个设置和各自的值来源。

web 看板显示同样的面板和行，“Send back”和“Yes”按钮对应继续和就这样收下。任务上的 **Settings** 按钮打开该任务的设置页：所有工作设置，含两个限额，各自的明文标签、值、来源、可取值和含义。行在那里改（[web 界面里的看板](web-ui.md)）。

## 计量的是什么

### 一次运行

代理工作的一段：从被叫起（你，或看板带着失败检查的输出、评审的结论）到该回合结束。对评审者，一次运行就是一次评审。

时钟是看板自己的，每几秒由维持看板更新的人（看守，或开着的看板）看一次，所以看板关着也有用。弹对话框等你回答的代理不会因等待被停。

### token

按整个任务计：它的代理、每次评审、每一轮，以及代理新开的聊天。看板从各程序自己保存的会话记录里读：模型读写的全部，每个请求累加，按服务商的算法。

**请求从服务商缓存重读的部分也计。** 一次请求读整个会话，所以数字涨得快：100k 上下文的五十步是五百万 token，一次评审轻松五十万。按这个来选限额（`show <id>` 和任务面板显示已用量）。

| 程序 | 计什么 |
|---|---|
| Claude Code | input、cache-write、cache-read 和 output token |
| Codex | 它累计的 input 和 output |
| pi | input、cache-read、cache-write 和 output |

- **计数在每个回合结束时做**，代理的和评审者的。所以一个回合可能越过限额：`limit.tokens 5m` 的任务首回合用掉 6m，停在 6m，不是 5m。长回合归 `limit.time` 管。
- **任务没设限额时，看板根本不读会话记录**：只记下它有哪些。一设限额，全部读进来，所以设得晚的限额仍算整个任务，含之前结束的每次评审。
- **token 数不出来的程序**不加数，`show <id>` 对它说 `limit.tokens` 不起作用（`does nothing for <program>: its tokens cannot be counted`）。看板读会话记录的三个程序（Claude Code、Codex、pi）都能计；Herdr 的其他程序不能（[代理程序](agents.md)）。

### 各自何时读取

- `limit.time` 在运行开始时读取，对该次运行有效。运行中设置或修改，从下一次运行（下次叫起代理，或下次评审）算起。
- `limit.tokens` 在每个回合结束、计数时读取。
- （运行的限额由第一次看到该运行时记在任务上，或由改动本身记上：保存新值前对进行中的每次运行都保留旧值，运行头几秒也算。）

## 与其他设置的关系

- **[并行尝试](attempts.md)**：同时做几遍的任务对限额是一个任务。每个尝试读任务的限额。`limit.time` 对各尝试自己的运行有效；`limit.tokens` 对所有尝试和任务一起有效，不管回合在哪个里结束。
- **[先计划](plan.md)**：token 按会话记录文件计、在任务上累加，所以计划者的会话记录多一个文件。一次运行是任务当前阶段里干活的那个代理。
- **[备用代理](fallback.md)**：触到限额的停下是看板普通的停下加个标记（`limitStop`）。触到限额的任务不交接：它等你。

这些设置来自任务 omh/154 的调研。

## 代码位置

- `crates/flow/src/limits/`：`run.rs`（轻的部分：一次运行和计什么）、`spent.rs`（计数）、`reached.rs`（停下）、`tick.rs`（巡检）、`shown.rs`（显示什么）；
- `crates/flow/src/actions/limits.rs`：继续，和就这样收下；
- `crates/core/src/agents/read_tokens.rs`：每程序一个计数器；
- token 限额在回合结束时检查：`crates/flow/src/finish/turn_ended.rs` 和 `crates/flow/src/finish/pipeline.rs`，任何自动合并之前也查：`crates/flow/src/finish/auto_merge.rs`；
- 测试：`tests/limits.rs`、`tests/limits_more.rs`、`tests/limits_tokens.rs`。
