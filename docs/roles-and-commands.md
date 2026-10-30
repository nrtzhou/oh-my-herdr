# 角色与命令

## 四个角色

| 角色 | 做什么 |
|---|---|
| **工作代理（Agent）** | 干活 |
| **评审者（reviewer）** | 攻击做完的工作，专找哪里不对 |
| **观察者（Watcher）** | 一个小模型，写「正在做」那行，并猜工作跑到多远了 |
| **协调者（Coordinator）** | 你在聊天里说话的那个代理；把你敲的字变成任务 |

### 观察者的猜测

大约每分钟一次（屏幕没变就跳过一次），观察者会看到任务、它的描述、代理屏幕上的最后几行、工作跑了多久，以及它自己上一次的猜测。它猜一个数字：工作完成了百分之几。除非屏幕给了改变的理由，它就保持上次的猜测。猜测可以往下掉：测试失败、合并冲突或评审发现都意味着还有活。更多见[观察者](watcher.md)。

看板把那个百分比换算成剩余分钟：

```
time spent × (100 − percent) ÷ percent        40 minutes in at 80% → 10 minutes left
```

上面式子的意思是：已用时间乘以（100 减百分比）除以百分比，比如 80% 时用了 40 分钟，还剩 10 分钟。

- 已用时间从代理开始做你最后一次交代的事算起。评审者工作期间，从那一轮评审开始算。
- 分钟每秒重算一次，所以在两次看之间也会一直动。
- 低于 5% 时依据太少，不显示分钟。
- 你自己的任务不同：它们显示你给 `report` 的 `--progress` 和 `--eta`，原样显示。

### 协调者

一个长期存在的聊天，把你敲的字变成看板命令，也能做代理聊天能做的任何事。

- 默认是 Claude Code 的默认模型和默认 effort，除非你另说；不管工作代理设成了什么。
- 用 `coordinator.agent`（`claude`、`codex`、`pi` 或 Herdr 能启动的其他程序，模型跟在斜杠后：[代理](agents.md)）、`coordinator.effort`、`coordinator.fast`、`coordinator.access` 和 `coordinator.instructions` 来改。或者用 `coordinator new --agent …` 为单次使用挑一个。
- 改过的默认值从下一个新协调者开始生效。聊天里会说明。
- `coordinator.agent` 设成 `none`、程序没装、或没有 Herdr 在跑时，聊天显示「New task」，你敲的字逐字变成任务。

## 每个聊天都懂看板

插件会在每个装好的代理程序自己的 skills 文件夹里放一份小型 how-to：一个名为 `omh` 的文件夹，内含 `SKILL.md`，位置在 `~/.claude/skills`、`~/.codex/skills`、`~/.pi/agent/skills` 和 `~/.omp/agent/skills`。所以任何普通聊天都能「add this to the board」（把这条加到看板），不用人教。

- 它给出的是命令的完整路径，所以 PATH 里没有 `omh` 也能用。
- Herdr 的其他程序拿不到它：看板不知道它们的 skills 文件夹在哪。
- `omh remove-extras` 只删当初写进去的东西，也不会再写回去。插件没写过的同名文件夹不会动。

## 命令

`add`、`list`、`report`、`describe`、`delegate`、`after`（[一个任务接一个](after.md)）、`tell`、`stop`、`approve`、`review`、`retry`、`fix`、`finish`、`details`、`pin`、`unpin`（[置顶](pinned.md)）、`archive`、`restore`、`clear`、`set`、`unset`、`show`、`automations`（[自动化](automations.md)）、`machines`（[其他机器](other-machines.md)）、`install`、`update`（[安装](install.md)）、`remove-extras`。

`omh help` 会列出它们。

### 命令回答什么

一行以任务 id 开头，在哪儿都是那几个词（看板自己的消息和协调者的收尾小结也用它们）：

| 命令 | 回答 |
|---|---|
| `add` | `proj/12: added`；带 `--delegate` 时是 `proj/12: added, an agent is on it` |
| `describe`、改标题、`report` | `updated`，附上改了什么：`updated (description)`、`updated (title)`、`updated (done)` |
| `delegate` | `started` |
| `clear` | `removed`，以及它先结束了什么：`proj/12: removed. Ended what it started: node …/vite.js (pid 812).`（[它留下了什么在跑](disk-space.md#它留下还在跑的东西)） |
| `archive`、`restore` | `archived`、`restored` |
| `archive --done [--repo NAME]` | `Archived 50 done tasks of mstars.`，以及跳过了什么：`Left out: 1 done but pinned, 2 working.` |
| `archive --repo NAME`、`restore --repo NAME`（`--on MACHINE`：别的机器上的） | `Archived mstars and its 6 tasks.`、`Restored mstars and its 6 tasks.`，以及它先停掉的任务（[归档项目](board.md#归档一个项目)） |
| `approve` | `approved` |
| `pin`、`unpin`（任务、机器或项目；`--on MACHINE`：别的机器上的项目） | `proj/12: pinned`、`vm-str: pinned`、`mstars on vm-b: unpinned`（[置顶](pinned.md)） |
| `stop` | `stopped`，以及除了代理它还结束了什么：`proj/12: stopped. Ended what it started: bun …/check (pid 812).` |

上表回答里的英文是命令输出原文，意思是「已添加（且已有代理接手）」「已更新（描述/标题/完成）」「已启动」「已移除，并结束了它启动的东西：node …/vite.js（pid 812）」「已归档/已恢复」「已归档 mstars 的 50 个完成任务；跳过：1 个完成但置顶，2 个进行中」「已归档 mstars 和它的 6 个任务」「已批准」「已置顶/已取消置顶」「已停止，并结束了它启动的东西」。

`stop` 会打断代理并留住它，所以回复还能继续，同时结束为这个任务启动的东西：终端里 shell 之下除代理及其助手（接给它的 MCP 服务器）外的每个进程、整棵进程树，以及从任务的工作树（worktree）或临时文件夹跑起来的你的进程。绝不会碰你自己的 checkout、别的任务的工作树、或任务在其他面板里跑的东西（你的面板、它评审者的面板）。为这个任务启动、但现在在别处干活的东西，只会列出来让你自己结束，不会直接结束：`Still running, end it yourself: node …/x.js (pid 9), in /srv/app.`。当 herdr 不说它的面板里跑着什么时，从它的文件夹跑起来的东西也一样处理：那时没法区分代理和你在那里开的 shell。

`approve` 会拒绝最后一次评审没过的工作，并说出评审发现了什么。`approve <id> --anyway` 照样合并。这是给人明确拍板时用的。

### 命令不认识的 flag 会被拒绝

每个命令只认自己的 flag，外加 `--json` 和 `--help`。`add`、`delegate` 和 `automations` 还接受任何工作设置的名字。其他东西会在命令做事之前停下，并指出最接近的正确 flag：

```
$ omh list --archvied
list does not know --archvied. Did you mean --archived?
```

上面报错的意思是：`list` 不认识 `--archvied`，问你是不是想写 `--archived`。（以前打错的 flag 会被忽略，于是命令做的事和你要的不一样。）

给 `add` 的设置从一开始就属于这个任务，带不带 `--delegate` 都一样。flag 清单在 `src/cli/flags.rs`。

## 看板启动的代理：有权，但不受指使

看板给任务启动的工作代理或评审者可以跑上面所有命令，和协调者、或你自己 shell 里的代理一样：`add`（也可以带 `--delegate`）、`delegate`、`tell`、`stop`、`approve`、`archive`、`restore`、`clear`、`set`、`unset`、`describe` 等等。没有任何命令被拒绝或改动。

但它们的规则里没提这些命令，而且写得明白：**除了自己的任务（报告自己的状态），不要动看板，除非人明确吩咐过。**

- 任务自己的描述里、或你发来的消息里的明确指示算数。所以一个写着「add a task for each failing page and delegate it」（给每个失败的页面加任务并派发）的任务，它的代理就会照做。
- 它绝不会自己往看板的文件夹（`~/.omh`）里写东西，手写或用它自己的程序都不行；其规则点名的文件除外（它的收尾小结、评审者的结论和证据）：看板只能通过它的命令改变。
- 顺手看到的活绝不会变成任务。它写进收尾小结或报告末尾一小段**Also noticed**（顺带发现）清单，别的什么都不做。评审者的 `preexisting` 发现是评审里的同一种东西：一条记下就完（[评审门](gate.md)）。
- 对自己的任务，它的规则让它把自己报告为 `blocked`，别的都不做（收尾流水线会把工作标为完成）。但它报来的 `done`、`idle` 或 `error` 和任何人的一样对待。
