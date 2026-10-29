# Herdr 的侧栏

**Herdr 的侧栏显示每个活着的代理在看板上的行的小副本**，用同样的词：任务的编号和标题、谁在干、干到哪，以及百分比和剩余时间。不显示行的第二行——代理说自己在干什么（`Reading the list code`、问题本身）：那留在看板上，而第 2 行已经说了干到哪。

每个活着的代理一条：任务的代理、评审进行时的评审者、以及协调者（Coordinator）。没人干的任务（待办、已完成并清理）在 Herdr 里没有面板，也就没有条目。

**完成的任务自己离开侧栏**：一旦它合并（这里或 GitHub 上）、无可合并而收尾、或由你说完成，它整个空间关闭。这就是设置 `workspace.cleanup`，除非你关掉，否则开着。留下的空间都有原因，任务的行会说明（`Kept: … (uncommitted changes)`）：见[任务完成时](how-work-gets-done.md#任务完成时)和[完成的任务留在侧栏里](troubleshooting.md#完成的任务留在-herdr-侧栏)。

要拿到全部三行，让看板在它的第一个界面把侧栏行加进 Herdr 的配置（[安装](install.md)）。

## 一个条目的三行

```
 ◐ HSW-152 Show automations in the m…      1. Herdr's dot, the task's number and title
   agent · working · R#1 PR#113 · 6m       2. who is on it, where it stands, review round, pull request, tags, time
   8% 16m                                  3. the percent and the time to go
```

### 第 1 行：Herdr 的圆点、编号和标题

和看板上一样：`HSW-152 …`，仓库的 key 加编号（[任务编号](task-numbers.md)）。编号在前，Herdr 在行尾截断，所以窄侧栏截的是标题，绝不截编号。机器名只在侧栏显示多台机器时才在它前面：那是 Herdr 自己的 `machine` token。

### 第 2 行：谁在干、干到哪

来自状态词的同一份列表（`crates/core/src/status_words.rs`）：

| 看板上的行 | 条目的第 2 行 |
|---|---|
| `◐` 代理在干活 | `agent · working` |
| `◑` 评审中 | 评审者条目是 `reviewer · under review`，代理条目是 `agent · under review` |
| `◇` 看板在启动、配置、检查、推送、合并，或在路上等待（该它检查、该它合并、GitHub 的检查） | `board · starting`、`board · setting up`、`board · checking`、`board · pushing`、`board · merging` |
| `◷` 等空位，`Q#2` | `queued`，第 3 行显示 `Q#2`：它在队列里的位置，干活中的任务在这里显示百分比 |
| `◷` 排在其他任务后面（[一个接一个](after.md)） | `queued after SKL-6` |
| `?` 问你 | `asks you` |
| `!` 要你点头 | `asks for your OK` |
| 登录标记 | `needs a login` |
| `◆` 通过，等你点头 | `ready to approve` |
| `!` 评审失败 | `review failed` |
| `×` 出错了（失败、合并或 PR 失败） | `went wrong` |
| 完成了，但分支跟不上 | `behind main` |
| 等你，无类别（评审者的对话框、无总结停止） | `needs you` |
| 停下、退回 | `to do` |
| 完成，空间还开着 | `done` |

- 只有有人干时才点名。等待中的任务说明它在等什么。
- 然后是评审轮次和 PR，按行的写法（`R#1 PR#113`）、任务的标签（`[chat]`、`[research]`、`[auto]`），最后是行的时间：干了多久写 `6m`，等了多久写 `2m ago`。
- 时间按分钟（第一分钟前是 `<1m`）：Herdr 显示它最后被告知的值，看板不是每秒都写。它排在最后，所以窄侧栏先截它。

### 第 3 行：光秃秃的数字

百分比（或队列位置）和剩余时间，放在行上显示它们的位置：`8% 16m`，超过一千分钟后写 `20h`。绝不写 `left` 这个词：侧栏窄，百分比后面的数字只可能是剩余时间。行上没有数字时为空。

### 评审者、协调者和空间

- **评审者的条目**同样方式命名任务，以 `reviewer` 为干活的人。评审期间任务有两个条目，各对应自己的面板：`reviewer · under review` 是评审者的，`agent · under review` 是工作被评审的代理的。其余的词相同，各有自己的圆点。
- **协调者**命名为 `Coordinator`，显示看板输入框上方那行说的话：`Coordinator · idle`、`Coordinator · working`、`Coordinator · asks you`，带等着发出去的内容（`· 1 message waits to be sent`），下面还有它最新的一条消息。Herdr 自己对它的词会是 `sw-coordinator` 和 `done`。
- **空间部分**遵循同样的规则：任务空间用任务的编号和标题命名（`HSW-152 Show automations…`），下面显示行的百分比和剩余时间。等你选一个并行尝试的任务没有这些：它的尝试条目写着 `pick one`。
- **只有任务自己的面板和空间。** herdr 会把关掉的面板或移除的空间的 id 再发出去（重启后，或你自己移除了完成任务的空间之后），但绝不会发终端的。在一个从未见过该任务的终端里、挂着任务 id 的面板不是这个任务的，即使在它自己的文件夹里：看板在那里不命名任何东西、也不拿走任何东西，任务的行会说它的代理不见了。它的空间只在它的面板在里面时才命名，协调者的面板或空间绝不是任务的，无论任务的旧链接怎么说。herdr 重启后，当 herdr 显示任务的代理在新终端里运行时，面板才又是任务的。

## 有插件的侧栏行和没有

Herdr 的默认侧栏显示空间名、标签页名和代理的显示名，不显示自定义值。所以：

| | 有这些行（[安装](install.md)） | Herdr 的默认行 |
|---|---|---|
| 第 1 行 | `HSW-152 Show automations…` | `HSW-152 Show automations… · Agent`：还带标签页名 |
| 第 2 行 | `agent · working · R#1 PR#113` | 相同 |
| 第 3 行 | `8% 16m` | 不显示：Herdr 的默认行不显示自定义值 |
| 空间 | 名字、分支、百分比和剩余时间 | 名字和分支 |

插件的行隐藏任务自己标签页的名字（`Agent`、`Reviewer #1`）：它们说的话下一行不会重复。你自己起名恰好叫 `Agent` 的标签页在第 1 行也会被隐藏。不是看板的代理保留 Herdr 自己的条目：它的空间、它的标签页、它的名字。

## 哪些是 Herdr 自己的

- **圆点是 Herdr 的**，用 Herdr 的颜色：它自己对面板的解读（working、idle、done、blocked）。看板不向 Herdr 写任何状态。告诉 Herdr 一个任务被卡住会让 Herdr 停止观察面板（[checked on Herdr](checked-on-herdr.md)）。所以问你的任务显示 Herdr 的完成或空闲圆点，旁边的词准确说明它干到哪。
- **截断是 Herdr 的。** Herdr 把一行的宽度分给各值，截断的行以 `…` 结尾。默认宽度 26 格时长行会被截断；Herdr 配置里 `[ui]` 下的 `sidebar_width = 36` 能显示更多。Herdr 对任何值最多保留 80 个字符：更长的行由看板自己截断，加 `…`。

## Herdr 0.9.3 在这里做不到的

- **协调者的第一行显示 `~ · Coordinator`**，而不是光 `Coordinator`。它住在主空间的一个标签页里，而侧栏行只能按值本身隐藏：隐藏 `~` 会对你主空间里每个代理都隐藏它。
- **没有看板自己的标记。** 行上的 `?`、`!`、`◑`、`◇`、`◷`、`◆` 画不出来：条目的图标是 Herdr 的状态图标，只有 Herdr 自己的五种状态有。词会把它表达出来。
- **没有按状态着色。** Herdr 可以按你配置里的文本规则给值着色。插件的行只把协调者的消息变暗，就像看板把它变灰那样。
- **没有进度条。** 百分比和剩余时间在，行的进度条不在。
- **折叠侧栏或手机宽度窗口里不行。** 那些保留 Herdr 自己的紧凑布局，侧栏行不改变它们（Herdr 的配置页这么说；这里没试过）。
- **另一台机器的任务**由那台机器上的插件写入那台机器的 Herdr。当这里的 Herdr 连着它、且那台机器跑着这个版本的插件时，它们会显示在这里。

## 它怎么写进去的

- **一个写入者，没有自己的词。** 看板用一个写入者写所有这些（`crates/flow/src/flow/sidebar.rs`）。它用看板自己的代码画任务的行，不带颜色（`crates/flow/src/flow/sidebar_entry.rs`）。
- **何时写。** 谁在 tick（看守或打开的看板）就每秒在条目有词变化时写一次：和看板重画自己行的频率一样。任务等你、出错、被重新接手或结束时，做这件事的那个会立刻写条目，不等 tick。
- **不打架。** 观察者（Watcher）和检查只保留他们在任务上发现的东西（[观察者](watcher.md)、[检查的进度](check-progress.md)）。它们自己不向 Herdr 写任何东西，所以没有两个写入者争一行。
- **Herdr 重启后。** Herdr 的服务器重启后这些全不留。看板在下一次 tick 时发现（按 Herdr 自己对面板的解读：它写的显示名没了），把那个 Herdr 的每个条目重写一遍。
- **不留过期内容。** 写过的内容会被替换，绝不留着等它过期。停止、完成或继续走的任务立刻（或一秒内）拿到新词。条目随面板走（Herdr 会丢掉关于关掉的面板的记录）。任务被归档或移除而面板还活着时，看板的词会被从面板上拿掉。
- **写成什么。** 写成 Herdr 的显示元数据（`herdr pane report-metadata`、`herdr workspace report-metadata` 以及空间名），来源是 `plugin:omh`。第 2 行是代理的显示名，第 3 行是 token `progress`。token `summary` 只属于协调者：在任务的面板和空间上它被清空，所以旧版本写的一行会自己消失。
