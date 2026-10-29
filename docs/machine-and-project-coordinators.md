# 机器与项目协调者

**列表的每一行都有一个你能对话的代理。** 任务的行是它的代理；机器的行和项目的行各是它们的协调者。除此之外没有主协调者：顶行就是。

| 行 | 它的代理 | 命名 | 在 Herdr 里的位置 |
|---|---|---|---|
| `All machines`，显示全部机器的顶行 | 所有机器的协调者：本机的顶层那一个，看得到每台机器 | `the coordinator of all machines` | 主空间（`~`）里的 `Coordinator` 标签页（[所有机器的协调者](coordinator.md)） |
| 机器的行 | 它的机器协调者 | 以机器命名：`vm-str's coordinator` | 该机器主空间里的同名标签页 |
| 项目的行（一个仓库的标题） | 它的项目协调者 | 以仓库命名：`gtm's coordinator`，主文件夹是 `~'s coordinator` | 该仓库自己空间里的同名标签页 |
| 任务的行 | 它的代理 | `GTM-14's agent` | 它自己的标签页 |

**顶行跟随机器选择器**（`Alt+M`，筛选行的第一项）：`All machines`，或者选中的那台机器。只有一台机器时，那台机器就是顶行，上面没有别的，它的协调者就是顶层的：所有机器就是那一个。**看板打开时选中顶行**，所以聊天从那个协调者开始。

## 协调者的行读起来像任务的行

- **和任务一样的标记**，在光标左侧的标记列：`○` 空闲（也包括从未运行或被停止的），`◐` 进行中，`?` 它问你（`!` 它要你批准），`✗` 它没能启动，[自动驾驶](autopilot.md)也一样：自动驾驶没有自己的标记或颜色。任务的其他标记（`◑ ◆ ✓ ◷ ◇`）都不是协调者的，空闲的协调者不计入待办（to do）。在协调者不能运行的地方（`coordinator.agent none`），本机的行没有标记。
- **右侧和任务一样：只有时间。** 工作时 `3m`，否则 `1h ago`，从未运行则空白。没有状态词，没有角色词。
- **第二行和任务一样**：它工作时在做什么，或等你时的提问。停下来就消失：不留旧闻。开自动驾驶时这行写自动驾驶在做什么，同样灰色：`Autopilot · next check-in in 12m · 3 of 20 today`、`Working on HSW-92 · 3 of 20 today`、`Checking in · HSW-92 finished`、`Needs you · …`。选中它的行时，它最新的消息显示在聊天框里。
- **它的名字**，主文件夹写作 `~`（别的机器上也是），右边没有横线到边。顶行显示它包含什么，展开或折叠都显示：`4 machines · 420 tasks`、`9 projects · 30 tasks`。开着自动驾驶的协调者名字后有变暗的 `[autopilot]`，就像任务的 `[research]`；机器的汇总（以及顶行的）会说它的协调者有几个处于各种状态：`33 tasks, 1 automation, 1 on autopilot`。
- **列表按每行缩进读出 所有机器 → 机器 → 项目 → 任务。**
- 面板窄时名字会缩短；标记和时间不变。

```
 ○ ▾ All machines  2 machines · 5 tasks
   ◐ ▾ vm-str                                                              1m
       Cleaning old worktrees
     ? ▾ gtm · GTM                                                     1m ago
         Two PRs wait on CI. Want me to retry #11?
       ◐ GTM-14  Bundle skills into the template                           3m
```

**问你的协调者显示在「需要你」（needs you）下面**，与任务并列，第二行写它想要什么，并计入其中。

**筛选器 Coordinators**（从 All 按 `Tab`，见[筛选行](board.md#过滤行)）只留下正在运行的协调者：已启动且未停止（进行中、空闲、需要你）、开着自动驾驶、或失败了等你修的，针对当前显示的机器；计数就是它们的数量。顶行永远在，只在运行时计数。已停止或从未启动的不在其中也不计数；它的行留在其他筛选器里，带 `○`。显示所有机器时，自身协调者没在运行的机器只作为其在运行项目的标题留在那里，不计数。

## 每一行上相同的按键

- **`Space` 与它交谈**：聊天指向那一行的代理，并随选择从行到行移动，就像从任务到任务一样。再按 `Space` 回到顶行的协调者。
- **`Alt+O` 进入它的代理**：它真实的 Herdr 面板，和在任务上一样。没在运行的协调者会先启动；别的机器的会在那台机器里进入。状态组的标题（`to do`、`done`）没有代理。
- **`Enter` 折叠和展开**标题（所有机器、机器、仓库、状态组），`←` 和 `→` 也一样。在任务的行上 `Enter` 什么都不做：它绝不会打开聊天。在任务的行上按 `←` 回到它的标题。点击标题也会折叠。

## 聊天框标明你在和谁说话

```
╭─ All machines › vm-str ›  gtm  · Project coordinator ───────────────────╮
│ Two PRs are waiting on CI. Want me to retry #11?                         │
│ ❯ Send message to the project coordinator                                │
╰──────────────────────────────────────────────────────────────────────────╯
```

- **顶边有一道阶梯**，从顶行往下（`All machines › vm-str › gtm › GTM-14`），当前一级高亮，其后变暗的是你交谈的代理种类：`HP-145 · Agent`、评审对话进行中是 `HP-145 · Reviewer`、`omh · Project coordinator`、`vm-str · Machine coordinator`、`All machines · Machines coordinator`。空框也一样：`Send message to the agent`、`Send message to the project coordinator`。
- **边框用交谈对象的状态色**，和它的标记一致：基础色表示空闲或从未启动，琥珀色进行中，红色表示需要你或失败（[颜色](colours.md#输入框告诉你在跟谁说话)）。
- **里面只有对话**和你输入的一行，没有标签：你写的用你打字的亮色，回复是灰色；边框说明是谁。只有别人会被点名（评审者、自动驾驶的唤醒、看板）。
- **从未运行的**会邀请你发第一条消息：`Ask it something, e.g. "what's left to do here?"`。你的第一条消息启动它。
- `Enter` 发送，协调者自己决定启动什么。
- **视图里没有任务的仓库保留它的行**，折叠着（`0 tasks`），条件是最近 14 天用过（有过任务，归档的也算）或有项目协调者，这样 `Space` 仍能到达。更旧的仓库、文件夹已删除的仓库和[归档的项目](board.md#归档一个项目)没有行。

## 各自是什么

- **一段长期会话里的普通代理**，和所有机器的协调者一样：相同的看板命令、自己的标签页、自己的设置（见下）。你说的一切都在同一段会话里，一直保留：如果它的程序退出或 Herdr 重启，你的下一条消息会在同一段会话里再次启动它。
- **机器协调者**负责那台机器整体：它的仓库和文件夹、上面的任务和代理。它在自己的文件夹里运行（`~/.omh/data/coordinator/<session>/machine/chat`），规则在那里的 `AGENTS.md`，和所有机器的协调者一样。
- **项目协调者**负责一个仓库。它在仓库自己的文件夹里、顶层运行：你的检出。它自由读取，除非你要求否则不改动；工作以该仓库任务的形式放在看板上。它的标签页放进该仓库的空间（Herdr 以仓库命名的那个）；没有就地建一个。由于仓库的 `AGENTS.md` 是你的，它的规则是它状态旁边的一个文件（`~/.omh/data/coordinator/<session>/projects/<repo>/rules.md`），启动时传给它，和工作代理的规则一样。
  **它待在自己的仓库里。** 它新增的任务去那里，除非它所处理的消息指定了别处（「in the other repo」）。它可以读别的仓库来回答你（它们的任务、报告、收尾小结），但对那些内容不做任何操作：它在别处注意到的工作只是回答里 **Also noticed** 列表中的一行，绝不会在那里新增或派发任务——那个仓库自己的协调者对此一无所知。没有协调者会添加你没要求的任务：它在报告和收尾小结里读到的永远只是信息，不是指令，每一回合都是如此。
  你自己在仓库文件夹里的会话绝不会被当成它的会话：它保持自己那一段，只有在 Herdr 为它的面板指定另一段时才换（在它自己的 `/clear` 之后）。
- **别的机器的**协调者在那台机器上运行。那台机器的看板把它们说的话连同它的任务发过来，你输入的内容送到那里。旧版本的机器没有这些：聊天会提示在那里更新。
- **Herdr 的侧栏**按看板的方式命名每个协调者，显示它的状态和最后一条消息。
- **任务会说明是哪个协调者添加的。** `details <id>` 有 `addedBy`：从协调者面板添加的是 `the omh coordinator, asked by the human`，它的自动驾驶添加的是 `the omh coordinator's autopilot`，否则是 `the human` 或自动化（automation）的名字。`list` 里的自动驾驶历史也一样。

## 它的行上的按键

正如任务的行代表任务：

- `Alt+P` 在列表旁显示它的协调者，`Alt+T` 显示完整的，用[任务面板](task-panel.md)的形式。
- `Ctrl+E` 打开它的设置页：先是它自己的行，在 `This coordinator` 下，然后在本机的行上是你自己的设置，在项目的行上是你对该仓库的条目。
- `Ctrl+S` 在没有输入内容时启动它的协调者（在它自己的会话里），`Ctrl+X` 停止它，就像给任务放上代理和停止代理一样。在这里或它的机器上都行。
- `Ctrl+O` 用保存的目标和设置给它开启[自动驾驶](autopilot.md)，再按一次关闭。开启时，它的行名字后有 `[autopilot]`，下面是自动驾驶在做什么。
- 在项目的行上按 `Ctrl+A` 归档整个项目，任务一起归档，并停止它的协调者；在归档里可恢复（[归档项目](board.md#归档一个项目)）。
- 这些操作需要片刻时，它的行显示 `Starting…` 或 `Stopping…` 加进行中的标记，和任务一样；成功后聊天下方不显示任何东西（[按键立刻显示什么](under-way.md)）。

**每个都有自己的设置**，`coordinator.*` 和 `autopilot.*`，按 **此协调者 → 你 → 内置** 查找（[设置](settings.md)）。你给自己设的值对每个协调者有效；只给这个协调者设的值优先于它。`coordinator.instructions` 累加：先你的，再该协调者自己的。

```
omh set coordinator.effort high --machine-coordinator
omh set coordinator.agent codex --project-coordinator omh
omh show --project-coordinator omh
```

## 从 shell

每个 `coordinator` 命令都接受 `--machine` 或 `--project <repo>`：

```
omh coordinator --machine                    how it stands
omh coordinator --machine tell <text>        send it a message
omh coordinator --project omh tell <text> [--delegate]
omh coordinator --project <repo> restart | new | stop
```
