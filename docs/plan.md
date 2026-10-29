# 先计划

**开启 `plan.approve` 后，代理先写计划再动手，计划获批之前仓库里不会有任何改动。**两分钟就能抓住的误解，胜过四十分钟后才发现的错误构建。

```
omh delegate 12 --plan.approve you                                    # show me the plan first
omh delegate 12 --plan.approve you --plan.agent claude/claude-opus-5-5 # a strong model plans, the usual one builds
omh set plan.approve reviewer --repo my-app                           # in this repo the reviewer checks every plan
```

或者告诉[协调者](coordinator.md)：「先给我看计划」「让 Opus 来写」。

```
task ─► the plan is written (nothing changes) ─► yes: you, or the reviewer ─► the agent builds that plan ─► check, review, merge
                     ▲                                │
                     └──── sent back with words ◄─────┘
```

## 两项设置

| 设置 | 取值 | 含义 |
|---|---|---|
| `plan.approve` | `none`（默认）、`you`、`reviewer` | `none`：不写计划，代理立即构建。`you`：代理写出计划，任务以 **计划待批准（plan ready）** 状态等你批准。`reviewer`：评审者对照任务描述检查计划，计划通过后构建自动继续 |
| `plan.agent` | `none`（默认），或一个程序，斜杠后可带模型（`claude/claude-opus-5-5`） | 谁写计划。`none`：工作代理。指定程序：独立的计划编写者 |

- 两项都和其它[设置](settings.md)一样：查找顺序 任务 → 你在该仓库的条目 → 你 → 默认；用 `set` 设置；`add --delegate` 和 `delegate` 以 `--plan.approve you` 传入；可由[自动化](automations.md)持有；`show` 会列出。
- **仓库自己的文件不能设置它们**，所以代理写的代码无法开关计划批准。
- **读取时机。**两项都在代理首次开始任务时读取：对代理已经在构建的任务设置它们，不会改变那一次运行。`plan.approve` 在计划写好后还再读一次，决定谁来批准：如果中途被关掉，就不问任何人，构建直接继续。
- 任务落地后重新开始的新一轮（清理后再次派发）会从一份新计划开始。

## 计划待批准：批准，或者发回

开启 `plan.approve: you` 时，任务停在自己的状态 **计划待批准（plan ready）**。它挂在 **需要你（needs you）** 下：行上显示 `Plan ready · <the plan's first words>`，State 行和 Herdr 的侧栏显示 `plan ready`，`board.notify` 像对待任何需要你的事项一样触发。

任务面板在 **Plan** 部分显示计划：当前状态、谁写的、计划原文（列表旁显示开头几行，全屏显示全文），以及 **`Open plan`**——一个私密链接，服务方式与报告相同。网页看板显示同样的内容。你二选一：

- **批准**：批准键（`ctrl+l`），或 `omh approve <id>`。计划按原样批准，构建开始。
- **回复**：`space`，然后输入你的话，或 `omh tell <id> <text>`。计划带着你的话退回编写者重写。新计划就位后，任务再次成为计划待批准。

## 评审者的审阅

开启 `plan.approve: reviewer` 时，评审者（`reviewer.agent`，标签页名为 `Plan review`）阅读计划及其提到的代码，像评审工作成果一样作答：照这份计划构建能否完成任务要求，外加按严重程度列出的发现。[评审门](gate.md)按同一规则判定（`review.mustFix`）：

- 计划通过：构建继续，不问任何人；
- 不通过：计划带着评审者的意见自动退回编写者，最多 `review.rounds` 次；
- 仍然不通过：任务成为计划待批准，评审者的发现放在计划部分，由你批准；
- **评审无法运行**（评审者起不来，或没有结论就退出）：任务照样成为计划待批准，行上说明原因（`The plan's review could not run (…): yours to judge`）。你的批准（`ctrl+l`）按原样构建计划；`omh retry <id>` 让评审者再看一次。

评审者在自己的只读挂载里运行，制作方式与编写者相同。它总是由看板从编写者的挂载外启动：编写者的程序在挂载内报告回合结束，其余由看板在挂载外完成（macOS 无法在沙箱里再建沙箱）。

审阅计划不算任务的评审轮次：工作成果的第一次评审仍然是 `Reviewer #1`。

## 批准之后

代理照该计划构建。

- 计划的编写者会在其对话里听到 `Your plan is approved`：它的程序被结束，并在同一段对话里、写计划用的只读挂载之外重新启动。
- 用过计划编写者之后，工作代理带着任务和计划文件开始。
- 对完成工作的评审会被告知已批准的计划。它对照任务**和**计划检查工作，说明构建在哪些地方偏离了计划（任务面板评审部分下的 `Left the plan`）。偏离计划只有在结果因此变差时才算一项发现。

## 谁来写计划

### 工作代理（`plan.agent: none`）

它被要求不改任何东西；写下它的理解、要改什么和在哪里、按什么顺序、怎么检查、哪些没把握；然后停下。

- **每个程序都用平常的权限写计划**（Claude Code、Codex、pi、Herdr 的其它程序：[agents](agents.md)），并把计划存成任务自己文件夹里的文件。[只读挂载](#批准之前仓库里不会有任何改动)让仓库保持原样，所以编写者仍能存笔记、发布页面、写收尾小结，不会卡在面板前等你批准。
- **不使用 Claude Code 自带的计划模式**：它会对计划文件之外的每次写入逐个在面板前询问。旧版看板以该模式启动的 Claude Code，在它呈现计划时照样会被取走计划，直到下一个版本。

### 独立的计划编写者（`plan.agent: <program>`）

- 它在任务的工作树里工作，在名为 **`Planner`** 的专属标签页中（[任务的标签页](tabs.md)），用自己的程序和模型；工作代理的 `Agent` 标签页在旁边等待。
- 写计划期间，它在各方面都是任务的代理：行、观察者的那一行、聊天、回复和停止键都归它。
- 计划编写者没装在这里：由工作代理写计划，任务的设置里用 `Note` 说明这一点。
- 批准后它的标签页关闭，**它花的时间和 token 记在任务上**（任务面板的计划部分：`Planner used 51s · 18k in · 29 out`）。

### 计划文件

一个文件 `plan.md`，放在看板上任务自己的文件夹里（`~/.omh/data/tasks/<repo>/<n>/artifacts/plan/`），绝不在仓库里。

- 代理停下却没存计划：它最后一条消息就是计划。
- 什么也没说：`No plan was written`，像任何失败一样退回给代理。
- 退回后没重新保存的计划还是旧的。此时编写者最后说的话就是新计划；什么都没说就没有新计划。更早的文件永远不会获批。

## 批准之前仓库里不会有任何改动

看板不只是口头要求。从计划步骤开始到构建启动，有四重保障：

1. **计划的编写者运行在工作树不可写的地方。**看板把它的程序放进专属挂载里启动，挂载由平台自己的工具制作（[哪一个](#linux-与-macos-上的挂载)），任务的工作树和它的 git 目录在里面不可写。写入、删除、提交，以及想把写权限改回来的 `chmod`，一律被拒（Linux 上是 `Read-only file system`，macOS 上是 `Operation not permitted`），不论程序自身权限如何。
   - 对每个程序都成立（Claude Code、Codex、pi 和 Herdr 的其它程序），对独立计划编写者、对检查计划的评审者也成立。
   - 其它一切与机器上无异，所以计划可以存进看板的文件夹，程序保留自己的会话文件。
   - 批准后，程序被结束并以平常方式在同一段对话里、该挂载之外重新启动。
   - **在做不出这种挂载的地方**，**根本不在仓库里写计划**。任务不启动，会说明缺什么和两条出路：让挂载可用（Linux：安装 bubblewrap；macOS：在任何沙箱之外启动 Herdr），或 `set plan.approve none --task <id>`。绝不用较弱的防护顶替。`omh doctor` 会说明这台机器能不能跑计划批准，不能是为什么。
2. **工作树的文件也变只读。**里面每个文件和文件夹都去掉写权限，被忽略的文件也一样，工作树自己的 git 目录（索引、`HEAD`）也是。挂载外跑的任何东西得到的回答都是 `Permission denied`。权限在构建启动前、其代理启动前恢复（所以启动失败会留下可写的工作树和 false 的 `plan.locked`），你停止任务时恢复，工作树被移除前也恢复。计划步骤还开着时再次派发，编写者重新启动，工作树再次只读。

   离开计划步骤的每条路都让工作树恢复可写：你的批准（也包括对写完后失败的计划的批准）、停止，以及去掉计划步骤的派发（`set plan.approve none --task <id>`）——后者按原样构建已写出的计划，没有计划就直接开工。
3. **看板自查**，计划的回合结束时一次，构建启动前再一次。要是还是变了（有人去掉了只读位），它就在任务自己的工作树里撤销：分支回到原来的提交，新增文件消失，步骤开始时不存在的被忽略文件也清掉。撤销的内容存成 `changed-too-early.patch` 放在计划旁边，任务说明还原了多少（`Put back  2 changes made in the repo before the yes`）。
4. **看板不能或不该撤销的改动之上，什么也不构建。**计划不显示，构建不启动：任务显示 `The repo was changed before the plan was approved`，附上改了什么。发给代理后它撤销这些改动。已给出的批准依然有效，改动消失后再试一次启动。

计划步骤开着期间，任何回合结束都不算完成的工作：不提交、不检查、不推送、不开 PR、不评审。

挂载防不住蓄意绕过的程序（比如让 Herdr 在别的面板里跑命令）。它拦住一切按常规方式做出的写入，无论写权限是否被改回。看板自己的自查和冻结构建负责其余部分。

### Linux 与 macOS 上的挂载

| | Linux | macOS |
|---|---|---|
| 工具 | bubblewrap：`bwrap --dev-bind / / --ro-bind <worktree> <worktree> --ro-bind <git folder> <git folder>` | macOS 自带沙箱：`sandbox-exec -p '<profile>'` |
| 防护内容 | 工作树和它的 git 目录以只读方式挂载 | 一个 Seatbelt 配置允许一切，只拒绝工作树和 git 目录下的写入：`(version 1)(allow default)(deny file-write* (subpath "<worktree>") (subpath "<git folder>"))`，各用真实路径（`/var/…` 是 `/private/var/…`） |
| 那里的写入 | `Read-only file system` | `Operation not permitted` |
| 需要 | 已装 bubblewrap，且允许用户命名空间 | `/usr/bin/sandbox-exec`（macOS 自带），且 Herdr 自身不在沙箱里启动 |

- 两者保证相同：程序无法在这两个文件夹下写入，也无法解除防护。
- **macOS 不允许沙箱套沙箱。**在自己沙箱里跑命令的程序，在我们的沙箱内每条命令都会失败，所以计划步骤中看板关掉它那个沙箱，由我们的防护负责：
  - Codex：`-c sandbox_mode="danger-full-access"`（完全访问下 `--dangerously-bypass-approvals-and-sandbox` 已经把它关了）。它的批准行为保持原样。
  - Claude Code：看板该次启动传给它的设置（`--settings`）里写 `"sandbox": {"enabled": false}`，以防你自己的设置开了它的沙箱。
  - pi 没有自己的沙箱。
  批准后程序以平常方式重新启动，它自己的沙箱按你的设置来。
- `OMH_SANDBOX` 直接指定工具：`bwrap`、`sandbox-exec` 或 `none`（不挂载：不在仓库里写计划）。未设置时，看板试平台自带的，并记住十分钟内它是否可用。从沙箱内部做的尝试（macOS 上在沙箱里总是失败）永远不会被记住。

### 在你自己的检出里跑的任务

没有自己工作树的任务只有一点不同：看板绝不摘掉你文件的写权限，也绝不在那里撤销任何东西，那里的东西可能是你的。

- 它的计划编写者照样运行在只读挂载里：你的检出只对那个程序是只读文件系统，对你从来不是。
- 规则 4 对期间发生的任何改动都成立：退回给代理（`review.rounds` 次，然后等你），改动消失之前什么都不构建。
- 仓库里的任务在首次派发时总会得到自己的工作树。

压根不是仓库的文件夹没什么可防护：计划照样写、照样批准，没有挂载。

## 一个任务上的多次并行尝试

计划属于任务，不属于某个代理或某个工作树：一个文件，写一次，批准一次。有[多次并行尝试](attempts.md)（`attempts.by`）时：

1. 任务启动时不拆分。一个编写者写计划（工作代理或计划编写者），批准一次。
2. 然后才拆分任务。每次尝试在第一条提示词里拿到同一份计划，照着构建。
3. 写计划用的工作树和标签页归尝试 A。其余每次尝试在旁边各有自己的。
4. 此后一切与做多次的任何任务相同：检查、挑选、对选中的尝试做一次评审，对照任务和计划检查。

`attempts.by` 在构建开始时读取，所以计划等待期间改动的数量也生效。
