# 自动驾驶

**自动驾驶（autopilot）是协调者上的一个开关。** 开着时，每当计时器或看板上的事态表明可能有活可干，看板就在它已有的聊天里给那个协调者发一条简短的唤醒消息。协调者随后像你一样，在自己的限额内用普通任务朝目标推进。关着时，它就是原来的那个协调者。

任何协调者都可以开自动驾驶：[协调者](coordinator.md)、机器协调者或项目协调者（[机器与项目协调者](machine-and-project-coordinators.md)）。没有第二个代理，也没有第二段对话：就是你平时交谈的那段会话，它记得你告诉它的事。

**`autopilot.approve yes` 覆盖 `merge.by`。** 默认值 `no` 时，自动驾驶新增的任务只按你的 `merge.by` 合并（merge）：在 `merge.by you` 下，它等你点头。设为 `yes`，协调者可以自己点头，任务不经你就合并。它是唯一能做到这一点的设置。除非你真要这样，否则保持 `no`。

## 开关它

最简单的方式是用平实的话告诉协调者：「go on autopilot: sync never loses data」、「put the vm-str coordinator on autopilot, goal: keep the tests green, two tasks a day」、「turn all autopilot off」。它会执行下面的命令。你永远不需要自己敲。

在看板上，在协调者的行上（见下），不输入任何内容：

| 按键 | 作用 |
|---|---|
| `Ctrl+O` | 开启自动驾驶，用保存的目标和设置；再按一次关闭。对显示 `needs you` 或 `could not start` 的，它会重新启动 |

`Ctrl+S` 和 `Ctrl+X` 在这里是启动和停止协调者本身，就像给任务放上代理和停止代理；自动驾驶有自己的按键。

从 shell：

```
omh autopilot                     this machine's coordinators, each with how it stands
omh autopilot show [--machine | --project <repo>]   one coordinator's autopilot whole, as it stands now
omh autopilot on [--machine | --project <repo>] [--goal "<words>"] [--autopilot.<name> <value> …]
omh autopilot off [--machine | --project <repo> | --all]
```

- **不带 flag 就是主协调者**，和 `coordinator` 命令一样。`--machine` 是本机的协调者，`--project <repo>` 是那个仓库的。
- **它需要目标。** `--goal` 把目标保存为该协调者自己的。没给就用保存的；没保存则拒绝：`Tell a coordinator its goal first.`
- **`on` 时附带的其他值只对这一次有效**，持续到关闭（见下）。
- **不能在 `coordinator.access ask` 下**：唤醒会卡在第一个权限提示上。已经是 `ask` 的协调者开上后显示 `needs you`。
- **开着时再 `on`** 会改动它指定的内容（新目标被保存，给定的值替换本次同名值，其余不变）并唤醒它。从 `needs you` 状态会带清零的计数重新开始。
- **开启即唤醒**，等你的任何回合一结束就发。看板上不显示消息：协调者的行会显示（见下）。命令会说明开启了什么，以及它的目标。
- **关闭**丢弃本次的值和它所有的唤醒理由。进行中的回合不会中断（那段会话也是你的），但从那一刻起该回合的每次写入都会被拒绝：`autopilot was turned off`。
- **`off --all` 只作用于本机。** 别的机器的协调者要在那台上关，或在它们的行上用 `Ctrl+O` 一个一个关。
- `omh autopilot` 列出本机的协调者。别的机器的在它们自己的看板上。
- **`coordinator new`、`restart`、`stop` 不改动自动驾驶。** 它的下次唤醒去往新会话。对开着自动驾驶的协调者执行 `coordinator stop` 会有提示：它下次被唤醒时会再次启动，而 `Ctrl+O` 关掉自动驾驶。

## 显示在哪里

- **机器的行和项目的行代表它们的协调者**，顶行（`All machines`，或唯一的机器）代表顶层那个；筛选器 Coordinators（`Tab`）只留下它们（[机器与项目协调者](machine-and-project-coordinators.md)）。
- **开着时，它的行保留协调者自己的标记**（`○` 空闲，`◐` 进行中，`?` 它问你，`✗` 未能启动），用它自己的颜色：自动驾驶没有自己的图标和颜色。它只在看板自己的样式里显示：

  ```
   ○ ▾ vm-str [autopilot]  33 tasks, 1 automation, 1 on autopilot
       Autopilot · next check-in in 12m · 3 of 20 today
     ○ ▾ omh · HSW [autopilot]
         Working on HSW-92 · 3 of 20 today
  ```

  - **名字后有标签**，`[autopilot]`，像任务的 `[research]` 一样变暗。
  - **下面有状态行**，用任务的灰色，取第一个成立的：

    | 行 | 何时 |
    |---|---|
    | `Needs you · …` | 它停止唤醒（见下），附原因 |
    | `Could not start · <why>` | 它的程序没能启动 |
    | `Checking in · HSW-84 finished` | 一次唤醒的回合进行中，附唤醒原因 |
    | `Working on HSW-92 · 3 of 20 today` | 它的一个任务进行中（外加今天 `tasksPerDay` 的新增数） |
    | `Autopilot · asleep outside mon-fri 09:00-18:00 · 3 of 20 today` | 在它的时段之外 |
    | `Autopilot · at today's limit · 5 of 5 today` | 没有余量再加任务 |
    | `Autopilot stalled: no check-in for 3h · 0 of 5 today` | 有余量却闲置，且超过 `autopilot.every`（见下）两倍时间没有任何运行 |
    | `Autopilot idle: last 3 wakes added nothing · next check-in in 2h · 0 of 5 today` | 闲置，计时器已退避（见下） |
    | `Autopilot · next check-in in 12m · 3 of 20 today` | 等待下一个唤醒理由 |

  - **它的机器汇总**（折叠的机器行，以及顶行）会说有几个协调者开着自动驾驶：`33 tasks, 1 automation, 1 on autopilot`。
  - web 看板显示同样的内容，在它自己的行里。
  - 关闭后，不显示任何自动驾驶内容。
- **状态词**，在它的面板和 `omh autopilot` 输出里，取第一个成立的：

  | 词 | 含义 |
  |---|---|
  | `needs you` | 它停止了唤醒：唤醒无人回应，或 `coordinator.access` 是 `ask` |
  | `could not start` | 它的程序没能启动，附原因 |
  | `working` | 一次唤醒的回合进行中 |
  | `asleep` | 在它的时段之外：`next in` 的位置写 `waits for its hours` |
  | `at its limit` | 没有余量再加任务，且它的任务都不需要照看 |
  | `on` | 等待下一个唤醒理由 |

- **它的面板。** `Alt+P` 以[任务面板](task-panel.md)的形式在列表旁显示协调者，`Alt+T` 显示完整的，带最近 20 次唤醒（时间、原因、是否回应）。开启时还显示：它的目标、今天的计数（`2 of 5 added · 1 of 2 under way`）和它的设置，用平实的话。
- **它的唤醒显示在它的聊天里**，署名 `Autopilot`，就像看板自己的话署名 `Board` 一样，绝不会显示成你的话：只写唤醒原因（`Regular check-in`、`HSW-84 finished`），用聊天惯用的灰色。
- **没有自己的颜色**（[颜色](colours.md)）。

## 何时唤醒

它有三类理由：

- **它的计时器**，`autopilot.every`，从它上一次唤醒结束（或上次查看没发现新东西）起算。错过的节拍不会补。
- **它范围内的看板事件**，`autopilot.when`：`done`（任务被合并或完成）、`stuck`（它照看中的任务开始需要照看：只触发一次，状态不变就不再触发）、`added`（除这个协调者外任何人添加了任务）。只有开启之后发生的事才算数，且每种只触发一次。
- **被开启。**

**免费查看。** 计时器唤醒之前，看板自己先看一眼，零成本：不调模型，不花 token。只有自上次唤醒结束以来有变化，计时器才唤醒它：它范围内的任务及其阶段、它的目标和设置、或今天的限额是否还有余量。**新的一天有了余量也算变化**，所以开着的协调者在有余量时至少每天唤醒一次。它自己那个回合做的事不触发唤醒：查看是在它的回合结束时进行的。

**闲置的自动驾驶不会停摆。** 只要有它自己的事在进行（工作中、排队中、或等你），免费查看就按住计时器。闲置时——没有自己的事在进行、今天的限额还有余量——即使没有任何变化计时器也会唤醒它：曾拒绝过或误解过唤醒的回合，还有一次机会。所以闲置的它不会每小时白白烧掉一个回合，它的唤醒中回合结束却毫无变化的会被连续计数：从第三次起，计时器间隔每次翻倍，最多到 4 小时（比这更长的 `autopilot.every` 保持不变），它的行显示 `Autopilot idle: last 3 wakes added nothing`。它范围内有变化仍照常唤醒，而改变了什么的回合（它添加、启动或重试了任务，或者你做了这些）会把计数清零，`autopilot on` 也一样。当它有余量、今天还有唤醒次数闲置着，却超过 `autopilot.every` 两倍时间没有任何运行（看守没跑，或计时器退避了），它的行显示 `Autopilot stalled: no check-in for 3h`，并附上退避原因：`Autopilot stalled: no check-in for 3h, last 4 wakes added nothing`。

**不唤醒的情形**：它关着；它显示 `needs you`；在 `autopilot.hours` 时段之外；距它上一次唤醒的回合结束或你的某个回合结束不到 `autopilot.gap`；今天 `autopilot.wakesPerDay` 已用完；它在回合中，或有你的消息在等它；或它没有余量且它的任务都不需要照看。这些理由会被保留，等条件解除后合成一次唤醒发出。只是计时器的理由，如果随后的查看发现没有新东西，就会被丢弃。

**你优先。** 队列里最多等一条唤醒。当你的消息入队时，还没打入的唤醒会被撤回，它的理由继续等。只有开启那一次唤醒不受你回合后的间隔限制。

**唤醒消息**总是以 `Autopilot:` 开头，只用平实的话说唤醒原因：

```
Autopilot: HSW-84 finished, HSW-88 has a question
```

| 理由 | 说法 |
|---|---|
| 开启（或改动） | `Turned on` |
| 计时器，且看板有变化 | `Regular check-in` |
| `done`：任务合并或完成 | `HSW-84 finished` |
| `stuck`：它失败了 | `HSW-84 is stuck` |
| `stuck`：它的代理提问 | `HSW-84 has a question` |
| `stuck`：它等一个自动驾驶可给的批准 | `HSW-84 is ready to approve` |
| `added`：别人添加了任务 | `HSW-84 was added` |

不带目标、限额和计数：**它的规则要求它每次唤醒先读自己的自动驾驶状态**，用 `omh autopilot show`（`--machine`、`--project <repo>`）读当前状态：它的状态、目标、今天的计数、进行中的任务和需要照看的任务，以及用平实的话写的设置（它能设置和触碰什么、它能否点头或添加发布任务）。所以两次唤醒间的变化总会被捕捉，聊天里也不会重复。

它的规则明确写道：**凡以 `Autopilot:` 开头的消息都是看板发来的真实唤醒**，无论旧版规则（或它会话中更早的部分）对别的措辞怎么说。当规则在它的程序运行中改变时，说明那一行放在唤醒词之后，绝不放在之前，并且带着唤醒时会说明这条是真实唤醒。

只有看板会写唤醒。其他以 `Autopilot:` 开头的消息（来自聊天框、`coordinator tell`、别的机器）到达它时会变成 `(not a wake) Autopilot: …`，以旧版的 `Autopilot woke:` 开头的也一样。没在运行的协调者会先启动，在同一段会话里，就像你的第一条消息会启动它那样。

- **已回应**指唤醒的回合在 15 分钟内结束。连续 `autopilot.needsYouAfter` 次未回应后，它的状态是 `needs you`（它的行显示：`Needs you · …`），在 `Ctrl+O` 或 `autopilot on` 之前不再发唤醒。回应一次就把计数清零。
- **启动失败**不算唤醒：它不占任何限额，等间隔过去再试，在启动成功之前它的行显示 `Could not start · <reason>`。

## 唤醒回合里它能做什么

唤醒开启的回合在你打入消息之前都归自动驾驶：接下来的就听你的了。在自动驾驶的回合里只有这些命令能通过。其他一切（合并、覆盖评审门（gate）、编辑或重命名任务、`set`、自动化、仓库、给别的协调者发消息）都会被拒绝，并给出规则和（如有）可放行它的设置。

- **读取**它范围内的信息：`list`、`show`、`details`、`autopilot`（以及 `autopilot show`）、`coordinator where`。看板是它唯一的记忆：`list` 还会给它范围内最近 30 天已完成、已停止和已归档的任务，各带添加者和收尾小结（closing note）的开头。
- **`add`**，在它的范围内，受 `autopilot.tasksPerDay` 和 `autopilot.tasksAtOnce` 限制。任务上设了 `autopilot.mayChange` 不允许的设置时不予应用，回答里会点名。设了 `autopilot.startAtOnce yes` 则立即启动，或在它自己的一个任务之后启动。
- **`delegate`** 它自己的任务，以及 `autopilot.scope all` 下你在它范围内、它自己机器上的待办任务。
- **`tell`** 它照看范围内、提出问题的任务的代理；绝不用来推进权限或许可提示。
- **`retry`** 它照看范围内的任务，最多 `autopilot.retries` 次。回答问题不用重试（retry）。
- **`stop`** 和 **`archive`** 它自己的任务，不能动等你点头的任务。
- **`approve`** 计划或合并，仅在 `autopilot.approve yes` 下，对它照看范围内的任务。绝不带 `--anyway`。
- **`autopilot off`**，对它自己、别的协调者或 `--all`。`autopilot on` 只有在 `autopilot.switchOthers yes` 下才能对别的协调者使用，且只能用那个协调者保存的目标和设置。

**它不能扩大自己的权限。** 在自动驾驶的回合里它不能提高自己的限额、写设置或用别的值开启自己。

**计数方式，处处一致：** 今天**新增**的任务，计入 `tasksPerDay`；**进行中**的任务，计入 `tasksAtOnce`：从启动（或链式启动时从新增）到合并、完成、停止或归档。等你也计入。天按本机算。

**它能照看什么**：它添加的任务，以及 `autopilot.scope all` 下你那些由它启动的待办任务。绝不是你那些它没启动的任务。其中某个任务**需要照看**的情形：它失败了（检查（check）、几轮后的评审（review）、某个限额、它代理的程序、它的搭建），或它的代理提问；`autopilot.approve yes` 下，还包括它等一个自动驾驶可给的批准。重试用完后仍失败或再次提问的任务**留给你**：不再触发唤醒，自动驾驶不再碰它。

**它的任务是普通任务**，检查、评审、评审门和合并都一样，按仓库的设置加上 `mayChange` 允许的来做。三点不同：

- 它们的代理和评审者可以读看板，并对自己那个任务 `report`。其他看板命令一律拒绝，这样它的任务做什么都不会在限额之外添活。一个命令属于它的代理（或评审者），当它在那个面板里运行、且该终端里曾被看到过那个代理：Herdr 会把已关闭面板的 id 交给同空间的下一个面板，所以待在某个已完成任务旧面板里的协调者就是那个协调者，不是那个已完成任务的代理。协调者自己的面板，在与它一起保存的那个终端里，永远不属于任何任务，无论任务记录里怎么写；工作树（worktree）已被清理的任务不会再与仓库文件夹里的代理建立关联。拒绝时会点名调用者被当成了谁（「You are taken as AFS-6's agent」）。
- 在 `autopilot.repoFileWaits yes` 下，触及仓库自己 `.omh.json` 的改动绝不自动合并，无论 `merge.by` 或 `autopilot.approve` 说什么。它的行会说它在等你亲自点头。
- 自动驾驶回合里的 approve，对任何改动触及该文件的任务一律拒绝，不管是谁添加的。

**只有你能开启它。** 任务的代理或评审者永远不能开或关自动驾驶，也不能写 `coordinator.*` 或 `autopilot.*` 设置，哪怕通过请求协调者也不行。关闭对你和任何协调者的回合都开放。

**什么都不做也没关系。** 它的规则写道：目标看起来达成了，或没什么值得做的，就用一行说明然后停下。绝不编造工作。没有发布任务（版本升级、打 tag、发布），除非 `autopilot.releases yes`；为 `no` 时，它会在认为该发布时说一声。

## 它的设置

协调者有设置，像任务一样，按同样的规则查找，就近者胜：**本次 → 此协调者 → 你 → 内置**（[设置](settings.md)）。包括 `coordinator.*`（它的程序、模型、effort、快速模式、访问权限、你的话）和 `autopilot.*`。

```
omh set autopilot.every 2h --project-coordinator omh
omh set coordinator.agent codex --machine-coordinator
omh show --main-coordinator
```

- **你**是你的 `config.json`，和所有设置一样：你在里面设的 `coordinator.agent` 对每个协调者有效。
- **此协调者**是同一个文件的 `coordinators` 部分：`{"coordinators": {"main": {…}, "machine": {…}, "projects": {"omh": {…}}}}`。`set`、`unset` 和 `show` 接受 `--main-coordinator`、`--machine-coordinator` 或 `--project-coordinator <repo>`。
- **本次**是 `autopilot on` 时给的值，只限 `autopilot.*`，保留到关闭。
- **`autopilot.goal` 只属于一个协调者**：绝不是你给所有协调者的，也不属于本次。
- **这些都不从仓库的 `.omh.json` 读取**，所以代理写的代码永远改不了它们。
- `coordinator.instructions` 累加：先你的，再该协调者自己的。
- 改动的 `coordinator.*` 从它的下一次全新会话起生效，面板会这么说。`coordinator new --agent` 的选择对它启动的那次会话仍然优先。

在看板上，在协调者的行上按 `Ctrl+E` 打开它的设置页，形式和按键与任务设置页相同。它自己的行在最前，在 `This coordinator` 下；然后在本机的行上是你自己的设置，在项目的行上是你对该仓库的条目。自动驾驶开着时，在目标之外的自动驾驶行上按 `Enter` 把它保存为本次值（`*`），行下面有一行说明。目标始终保存为该协调者自己的。`Ctrl+U` 还原一行。

| 设置 | 内置 | 简述 |
|---|---|---|
| `autopilot.goal` | `none` | 它朝什么努力 |
| `autopilot.every` | `6h` | 它的计时器 |
| `autopilot.gap` | `10m` | 两次唤醒之间的最短间隔 |
| `autopilot.wakesPerDay` | `24` | 一天最多几次唤醒 |
| `autopilot.when` | `done stuck added` | 哪些事件唤醒它 |
| `autopilot.hours` | `any` | 它可以何时唤醒（`mon-fri 09:00-18:00`） |
| `autopilot.tasksPerDay` | `5` | 它一天最多加几个任务 |
| `autopilot.tasksAtOnce` | `2` | 它最多几个任务同时进行 |
| `autopilot.mayChange` | `none` | 它可以给自己的任务设哪些任务设置 |
| `autopilot.scope` | `own` | `all`：它也可以启动你的待办任务 |
| `autopilot.approve` | `no` | 它可以替自己的任务点头：覆盖 `merge.by` |
| `autopilot.switchOthers` | `no` | 它可以自己开启别的协调者 |
| `autopilot.releases` | `no` | 它可以添加发布任务 |
| `autopilot.retries` | `1` | 一个任务留给你之前重试几次 |
| `autopilot.startAtOnce` | `yes` | 它的任务立即启动 |
| `autopilot.needsYouAfter` | `3` | 多少次唤醒无回应后转 `needs you` |
| `autopilot.repoFileWaits` | `yes` | 对 `.omh.json` 的改动等你 |

每项的完整含义见[设置清单](settings.md#设置清单)。

## 层级

| 协调者 | 它的范围 |
|---|---|
| 主协调者 | 看板显示的一切，每台机器 |
| 机器协调者 | 那台机器的仓库 |
| 项目协调者 | 它的仓库 |

- 两个照看同一个仓库（主协调者和该仓库的）各自照看自己的任务、各自计算自己的限额。协调者之间没有共同限额。
- **它们能互相唤醒**：一个添加的任务在另一个的范围内就是 `added`。各自仍受自己的间隔、每日唤醒数和每日任务数约束。
- 在 `autopilot.scope all` 下，你的待办任务交给任务所在机器上开着 `scope all`、离它最近的自动驾驶协调者：先是它的项目协调者，然后机器协调者，然后那台机器的主协调者。

## 其他机器

**每台机器运行自己协调者的自动驾驶**，有自己的设置和限额。本看板按那台机器随任务发来的内容显示它们的行和面板，把 `Ctrl+O`（以及 `Ctrl+S`、`Ctrl+X`）发到那边。在其中一个上按 `Ctrl+E` 显示它的完整设置（只读），并提示去那边改。每个有行的协调者都会发来：那台机器的和那台机器知道的每个仓库的，从未运行的按关闭处理：在那里按 `Ctrl+O` 启动它，就像发第一条消息那样。旧版本的机器不发任何自动驾驶内容，按键会提示去那边更新。

**主协调者只在看板打开时才看到其他机器。** 看守不会向其他机器要任务。所以它们的任务只有在某个看板（终端里或 web UI 里）最后听到它们时才进入主协调者的自动驾驶，3 分钟没有新回答就丢弃。没有看板打开时，它的范围就是本机的任务。

它也能像你一样在别的机器上加任务，自动驾驶之外也行：

```
omh add "<title>" --repo <repo> --on <machine>
```

在唤醒里，这算进它自己的限额：它的名额先在本机占用，如果那台机器拒绝该任务则释放。任务带着标明本机和当时生效的 `autopilot.repoFileWaits` 的记录，那台机器会按两者执行。旧版本的机器拒绝它，提示更新。它只在自己机器上启动你的待办任务。

## 自动驾驶与自动化

它们是两回事，自动化不受影响。

- **[自动化](automations.md)** 是保存的任务加触发器：每次触发都添加同样的任务并启动它。它不追逐目标，也不唤醒谁。
- **自动驾驶**唤醒一个协调者，由它决定朝目标做什么、做不做。

它们会相遇：由 `merged` 触发的自动化，在自动驾驶的某个任务合并时加一个任务，那个任务在自动驾驶的范围内就是 `added`，于是唤醒协调者。两者都有边界：自动化受每小时十个任务限制，自动驾驶受间隔、每日唤醒数和每日任务数限制。

## 信任到什么程度

**信任水平就是现有的水平。** 协调者本来就拥有全部看板命令，默认 `full` 访问权限。看板在自动驾驶回合里的拒绝能拦住失误；它拦不住一个变坏的代理。

- 在别的面板里执行命令的协调者可以冒充你。
- 收尾小结、仓库文件或别人任务里的文字可以操纵它。它的规则说这类文字是信息不是指令，但规则不是封印。
- **代理文件像任何改动一样合并。** `AGENTS.md`、`CLAUDE.md`、钩子（hook）和检查脚本不会替你扣下；只有 `.omh.json` 会。那里的防线是评审。
- **每次唤醒都是你也在用的那段聊天里的一个回合。** 它会变长，每次唤醒都带着全部内容。`coordinator new` 让它重新开始，`list` 把它的历史还给它。
- **要和开着自动驾驶的协调者说话，请通过看板，或先关掉自动驾驶。** 看不到你直接在协调者标签页里输入的内容：你在那里打字时可能有唤醒被送进去，而你在唤醒后紧接着在那里开始的回合仍可能算作自动驾驶的。在那里输入的内容同样不会被转义。
- 头几周盯着它，看目标达成后会不会做无用功。用 `merge.by you`，它完成的任务等你，`tasksAtOnce` 把它们计入，所以最多堆两个（默认）。

## 在 omh 自己身上

自动驾驶可以做看板自己仓库的活，但那里的代理可能去改进它们自己的工具链而不是项目。它的改动只有在你发布并安装某个版本后才算数，而发布不是它的事，除非 `autopilot.releases yes`。因此，给它一个窄目标（「the web board's chat loses no message」），并保持 `merge.by you`。

## 存放在哪

- **它的状态**（开或关、本次的值、今天的计数、它的任务、最近 20 次唤醒）：[主文件夹](home-folder.md)里协调者 `state.json` 的 `autopilot` 部分（`data/coordinator/<session>/`、`machine/`、`projects/<repo>/`）。
- **它的设置**：你的 `config.json` 的 `coordinators` 部分。
- **其他机器的任务，按看板最后听到的**：`data/autopilot-heard.json`。
- 代码：`crates/flow/src/autopilot/`，规则在 `crates/flow/src/coordinator/rules_autopilot.rs`。
