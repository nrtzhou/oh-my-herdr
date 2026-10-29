# 一个接一个：`--after`

**任务可以等其他任务完成。**适合链式工作：先改 `gtm-skills`，再把成果带进 `gtm-agent`，再进下游各副本。每个都是自己的任务，有自己的评审（review）和合并（merge）。

```sh
omh add "Take the new skill in" --repo gtm-agent --after gtm-skills/12             # only the condition
omh add "Take the new skill in" --repo gtm-agent --after gtm-skills/12 --delegate  # queued after it
omh delegate gtm-agent/4 --after gtm-skills/12,gtm-skills/13
```

也可以告诉协调者（Coordinator）：“then”、“once that is merged”、“after HSW-12”、“after ELA-3 on vm-str” 会变成它添加或派发（delegate）的任务上的 `--after`。只有你要求任务开始时，它才加 `--delegate`（把任务排队）。

`--after` 只是个条件：它点名的任务没完成之前，该任务不能启动。启动仍然是 `Ctrl+S`（`delegate`），和任何任务一样。

| 你做了 | 任务 |
|---|---|
| 只 `--after`，没按 `Ctrl+S` | 留在**待办**下，**绝不自己启动** |
| `--after` 加 `Ctrl+S`（`delegate`） | **排队**，它们全部完成后自己启动 |

**完成**就是看板叫完成的那些：工作合并了、研究任务交了报告、或任务标了完成（`report <id> --state done`，无东西可合并）。完成后归档的任务也算完成。

## 没派发：只是个条件

带 `--after` 但没人派发的任务（`add --after` 没有 `--delegate`，或对一个待办任务用 `after <id> <id>`）带着 `○` 留在**待办**下。它绝不自己启动，哪怕那些任务都完成了。

- `Ctrl+S` 随时启动它。那些任务还没全完成就排队（见下）。全完成了就立刻启动。
- 它的行不提等待。Web 看板和 Herdr 侧栏（sidebar）的行也不提。
- 任务面板里有一行 `To wait for`（`To wait for  SKL-6`），在那里看和改。
- `list` 显示在标题旁边：`after SKL-6`，或 `after SKL-4, SKL-5`，点名还没完成的（全完成后不点名）。

## 已派发：排在它们之后

带 `--after` 派发、而那些任务还没全完成的任务会排队，和等空闲名额的任务一模一样（[看板](board.md)）：

- 队列时钟 `◷`，洋红色，在**进行中**下面的排队任务里（排在等名额的之后）；
- 它的行显示 `Queued after AGT-2`（或 `Queued after SKL-4, SKL-5`），按键名点名还没完成的，别处也一样（别的机器的任务用 `vm-str:ELA-3`），和任何任务行一样的静灰，在同样的地方截断；
- 它还没有队列位置（`Q#2`）：它在等任务，不是在等名额；
- 任务面板的 State 显示 `queued after AGT-2`，`Starts after` 行逐个列出任务（已完成的后面标 `(done)`）。`details` 里有 `after` 部分，`list` 显示在标题旁边；
- Web 看板、别的机器的看板和 Herdr 侧栏（`queued after AGT-2`）显示同样的内容。别的机器的看板按任务自己那台机器的说法显示行和名字（它的键名，那块看板可能不认识），绝不显示裸 id（`gtm-agent/2`）。

这个功能出现之前用 `--after … --delegate` 加的任务算作已派发，显示为排队。

等待期间：

- 它不占代理名额（`board.agents`），也不为它跑任何东西：没有工作树（worktree），没有空间（space），没有代理。
- id 可以是**其他仓库**（本机任何仓库）和看板认识的**其他机器**（见下）的任务。
- 不存在的 id 会被拒绝。循环等待（`a` 等 `b`，`b` 等 `a`）也拒绝：回答显示循环，`proj/1 → proj/3 → proj/2 → proj/1`，什么也不改。`add` 时被拒的 `--after` 不会加上任务。

## 等待结束时

它等的任务全部完成后，已派发的任务自己去拿空闲名额，和任何派发的任务一样。没有空闲名额就等名额：`waiting for a free slot, Q#2`。

它的代理的首个提示词（prompt）会说明前面的来龙去脉，逐个任务：id、标题和仓库，它的收尾小结（closing note），以及它的工作合并到的 PR 和合并提交，研究任务则是报告链接。这些在等待结束时取来存到任务上（`details`：`after.handedOver`），所以另一个任务的清理不会弄丢。同时做多次的任务（`attempts.by`）把同样的内容交给每次并行尝试（attempts）的代理。

## 自己走不下去时

它等的任务失败、被停、被移除、或没完成就被归档时，已派发的任务不启动。它带着原因（`gtm-skills/12 was archived without being done`）在**需要你**下等你，像任何需要你的任务一样通知你（`board.notify`）。

这些都不是终局：让那个任务重新跑起来，它一完成，等待自动继续。

## 修改它，提前启动

- `omh after <id> <id>[,<id>…]` 设置任务在等什么。对等待中的任务，新列表从现在起算（全已完成：立刻启动）。对还没派发的任务，留到派发时生效。
- `after <id>` 查看它在等什么。
- `after <id> --clear` 去掉等待；在等的任务立刻启动。派发时用 `delegate <id> --after none` 一样。
- 对排队等待其他任务的任务用 `delegate`，会说明它还在等、怎么继续：`delegate <id> --now` 照样启动它。它的代理会被告知任务没全完成、以及它们各自的进展。在看板（和 Web 看板）上，对这种任务按 `Ctrl+S` 会先问：`Start HSW-4 "…" now?`。
- 对排队等待其他任务的任务用 `stop <id>` 停掉等待（看板上用停止键，和任何排队任务一样）。它回到待办，它等的东西作为下次 `Ctrl+S` 的条件保留。等这个任务的任务会被告知它被停了，和任何被停任务一样，直到它再次派发。
- 代理已经在做的任务不能改成等待：等待发生在启动之前。

## 等另一台机器上的任务

任务可以等看板认识的任何其他机器上的任务（`board.machines`，或 Herdr 自己的机器列表：[其他机器](other-machines.md)）。键名可能跨机器撞车，所以这种任务先写机器名：`vm-str:ELA-3`、`vm-str:ela/3` 或 `vm-str#3`。保存为 `vm-str:ela/3`，在看板说任务在等什么的地方（它的行、它的面板、`details`、`list`、别的机器的看板）都显示为 `vm-str:ELA-3`。

```sh
omh delegate notably/3 --after vm-str:ELA-3
omh add "Use the new API" --after vm-str:ELA-3,HSW-12 --delegate
```

- **设置等待时**，看板会问那台机器任务在不在。不认识的机器、那台机器没有的任务、当时不响应的机器，都会被拒绝，什么也不改。
- **跨机器的循环也拒绝。**看板会顺着那边任务等的东西往下追，在那台机器和它点名的任何其他机器上，一直追回本机。追不到底（路上有机器不响应、或点了本看板不认识的机器）同样拒绝：看不见的循环会永远等下去。
- **等待期间**，本机每 15 秒问一次对方，在自己的作业里问（慢机器不会挡住别的事）。对方只要有这个版本的 omh 就行，不用开着看板。旧版本答不了，看板会说明，并提示「update it there」。
- **机器不响应不会结束等待。**任务继续等，行上说明：`queued after vm-str:ELA-3 · waiting for vm-str: not reachable since 10:42`（`details`：`after.notReachable`）。它恢复响应后，行恢复正常。
- **完成、失败、被停、被移除、或没完成就被归档**以那台机器对它任务的说法为准，在这里的效果和本机任务一样：完成算数，其余让任务带着原因等你（`vm-str:ELA-3 was stopped`）。
- **等待结束时**，代理会被告知那个任务做了什么，和本机任务一样（它的收尾小结、PR、合并提交或报告，以及它在哪台机器上），依据是对方机器最后一次的答复。
