# 任务面板

**任务面板显示选中任务的一切。** `Alt+P` 在列表旁打开和关闭它（90 列以上的窗口里），你的选择会被记住。`Alt+T` 全屏显示整个任务；`Esc` 或 `Alt+T` 离开。`Ctrl+E` 打开任务的设置。

关闭时，每行都有整个宽度来显示内容。打开或折叠面板绝不改变你在和谁说话、聊天消息是否显示，`Space` 在任何宽度下都不打开或折叠它。

```
 HSW-84 Max length for task titles

 Status ─────────────────────────────────────────────
 State        ◆ ready to approve · 4th review · 2m ago
 On it        claude
 Change       PR#60 · 19 files, +340 −27

 Description ────────────────────────────────────────
 A task title should have a maximum length.

 Reviews · 4 ────────────────────────────────────────
 History      fail → no verdict → fail → pass
 Review 4     ✓ Pass · 1 minor · 11m ago
              The change meets the task. A title at
              the limit is kept whole.
 Evidence     ↗ Open review

 The agent said ─────────────────────────────────────
 Titles longer than 80 characters are cut between words.

 Settings ───────────────────────────────────────────
 Agent        claude · opus * · default effort
 Plan first   off
 Reviewer     codex · default model · default effort
 Review when  in a repo · with evidence
 Send back    major or worse · 5 rounds max
 Check        npm test · 15m at most
 Merge        after your yes · squash · opens a pull request
              waits for GitHub's checks
 Workspace    from main · removed when done
 Also         one attempt · not research only · no limits
              no fallback
 * this task only
```

## 每个部分一种形式

面板和全屏是同样的部分，同样的顺序：

- **标题行**，粗体，压在一条延伸到边缘的细横线上。需要你的用它状态的颜色。
- **事实是标签加值**：标签灰色在左，值在右。每个值从同一列开始，让视线竖着走。长值在自己下面折行，在它的事实之间折（`12m ago` 绝不拆开）。
- **一段文字**（描述、代理说的话）是标题行下的朴素行。
- **两个部分之间一个空行**，全屏上两场评审之间也是。别处没有。
- **没有东西装在框里，没有东西压在横条后面，没有事实被 `…` 截断。** 面板太矮时，整行和整个事实让路（[下文](#矮窗口)），最后一行说明其余内容在哪。

## 各部分，从上到下

- **标题**，在最上面，单独一行。任务按它行上的方式命名（`HSW-84`），标题里和整个任务页的标题里都是（`Task details  HSW-84 "Max length for task titles"`）。它的 id，按命令接受的形式，只出现一次，在「整个任务」上，带会话和仓库的 key（`Task id  omh/84 · HSW`）。
- **Status**：干到哪了，一行一个事实，总是这个顺序：

  | 行 | 说明 |
  |---|---|
  | `State` | 它的标记和词、第几轮评审、从什么时候 |
  | `Progress` | 进度条、百分比和剩余分钟 |
  | `Now` | 代理或评审者此刻在做什么，整句折行 |
  | `On it` | 谁在干 |
  | `Took over` | 每次移交给[备用代理](fallback.md)，用朴素的话 |
  | `Made by` | 制造它的[自动化](automations.md)（如果有的话） |
  | `Change` | PR，用它的检查对应的颜色，以及改动的大小。编号是链接；全屏时地址也印出来 |
  | `Next` | 没人干的任务：什么会让一个代理接手 |

- **它需要你什么**：Status 正下方的独立部分，标题行是红的：`Asks you`、`Asks for your OK`、`Needs a login`、`Review failed`（评审发现了什么，两个键各做什么：退回，或照样合并），或 `Went wrong` 加失败内容、谁能修、`Next step` 和 `Details`。标题行绝不在下面第一行重复。`Behind main` 和 `Cut off`（看板自己重试）是同一部分，用各自的颜色，归档的任务在那里说明。
- **描述**：全屏上完整，列表旁约 400 字符后截断。面板太矮、放不下它和设置时，它再被截，然后变成一行（`Description · alt+t shows it`），然后消失：这一切都发生在任何单个设置之前。
- **并行尝试**（attempts），一次做几遍的任务：评审者的比较，然后是每个尝试（[并行尝试](attempts.md)）。任务等你挑选时，它的标题行是 `Pick one of 3`，红色，属于「需要你什么」。
- **评审们**，作为历史（[下文](#评审们)）。
- **代理说的话**：它对话里最新的词，绝不是它程序的屏幕（你和它的代理聊天时，那些在聊天里）。评审期间是评审者的，标题行写 `The reviewer says`。评审结束后又变回代理的。只出现在程序屏幕上的对话框会用一行提到，附去那里的方法。聊天显示什么、从哪显示：[看板](board.md)。
- **设置**：任务运行用的每个设置（[下文](#设置部分)）。
- **Session**，仅全屏：代理的上下文、它的 tokens、它在 Herdr 里跑在哪。

全屏时文字控制在 100 列内。列表旁，任何内容比全屏短时，最后一行写 `alt+t shows the whole task`。

## 设置部分

任务运行用的每个设置和它的值，一行几个，用朴素的话。绝不写一行说这些是默认值。

| 行 | 说明 | 设置 |
|---|---|---|
| `Agent` | 谁干活：程序（和版本）、模型、effort；`fast`、`asks before acting`、`words of your own` 在成立时显示 | `worker.*` |
| `Takes over` | 代理或评审者干不下去时谁接手，排在那一位的行下面。两者中一旦一个有列表，两行都有，没列的那个写 `nobody` | `worker.fallback`、`reviewer.fallback` |
| `Plan first` | `off`，或谁批准计划、谁来写（就算关着，有人选过的 planner 也显示） | `plan.approve`、`plan.agent` |
| `Attempts` | 几个、由谁做，谁来挑；只在多于一个时显示 | `attempts.by`、`attempts.pick` |
| `Reviewer` | 谁评审，格式同 `Agent`；不评审的地方写 `off`，附有人为它选过的东西 | `reviewer.*` |
| `Review when` | `in a repo`、`always` 或 `never`，以及评审者是否带证据 | `review.when`、`review.evidence` |
| `Send back` | 多严重的发现才把工作退回，几轮之内自己重来 | `review.mustFix`、`review.rounds` |
| `Check` | 检查的命令和最长运行时间 | `check.command`、`check.timeout` |
| `Merge` | 你点头后还是自己来，怎么落地，是否开 PR。只研究（research-only）的任务这里是 `Research`：它什么都不合并。[只聊天](chat.md)的是 `Chat`（`no change to the repo · ends when you say`），且没有 `Reviewer`、`Review when`、`Send back`、`Check` 行：它们都不运行 | `merge.by`、`merge.method`、`merge.pr`、`research.only`、`chat.only` |
| `Limits` | 最长运行时间和最多 tokens，两者都写（`2h a run · no token limit`）；只在设了其中之一、或有人说了 `none` 时显示 | `limit.time`、`limit.tokens` |
| `Workspace` | 工作树从什么开始、什么配置它、拷入什么、拆掉什么，完成后是否移除 | `workspace.*` |
| `Also` | 各用一个词，写那些没有自己行、又什么都没做的：`one attempt`、`not research only`、`no limits`、`no fallback` | |

- **只为这个任务设的值才被标记**：后面一个 `*`，用强调色，行下面一行暗淡的 `* this task only`（空间不够时最先消失的一行）。其他每个值看起来都一样，无论来自你的设置、仓库的还是内置。面板绝不说明值从哪来：`omh show <id>` 会（[设置](settings.md)）。
- 有人选过的每个设置都显示，包括写着 `none` 的，或现在不起作用的。
- 不在仓库里的任务没有 `Check` 和 `Workspace` 行。
- 「整个任务」多出任务在哪（`Project`、带文件夹的 `Worktree`、`Branch`）。列表旁，只有不属于本看板的项目、以及另一台机器的名字（`Machine`）才有行。
- `Note` 行说明看板用不了的东西：没装的程序、仓库自己的文件被拒绝、你设置文件里的笔误。
- 另一台机器上的任务有同样的行，来自那台机器发来的设置。
- web 看板的任务面板也有这些，就在 `Status` 和「需要你什么」下面，在描述、评审和代理说的话上面：它的页面滚动，所以有空间的排在上面（[web UI 里的看板](web-ui.md)）。

## 设置页

任务上按 `Ctrl+E` 全屏打开它的设置：每个工作[设置](settings.md)按朴素标签排列，值，以及命令接受的名字，只为本任务设的带 `*`。其他值从哪来这里同样不说：`omh show <id>` 说。

```
 Settings  HSW-12 "Tidy the first-run questions"
 ────────────────────────────────────────────────────────────────────────────────
 Send back from: critical · [major] · minor · trivial

     Agent's program       claude    worker.agent
   * Agent's effort        high      worker.effort
     …
▌›   Send back from        major     review.mustFix
     Rounds by itself      5         review.rounds
     Merged by             itself    merge.by
     Check command         npm test  check.command

 * this task only
```

| 按键 | 作用 |
|---|---|
| `↑` `↓` | 移到某行（这里不在任务间移动） |
| `Enter` | 改这一行，只对这个任务（下文） |
| `Ctrl+U` | 把选中的行放回没有这个任务自己答案时的样子 |
| `Esc`，或再按 `Ctrl+E` | 回到任务 |

- **`Enter`**：开关或选项走到下一个值并立刻保存。其他内容在底部输入框里打（开始是空的；页面显示当前值），用 `Enter` 保存（`Esc` 保持原样）。被拒绝的值说明原因，附最接近的合法值，且什么都不改。
- **顶上的那行**是选中行的：它可以是什么。选项列出它的词、当前的标出；开关读作 `on · [off]`；要打字的说明打什么（`a time, such as 90s or 15m`）。
- `*` 标记只为本任务设的行。其他行都是朴素的，无论值从哪来。对本任务不起作用、或从下次启动才算数的行，在名字前说明。
- **用不上的设置不提供**：fast 模式，对没有它的程序（只有 Codex 有）；仓库设置，对不在仓库里的任务。就算如此仍属于本任务自己的设置会显示，好让它能被放回去。
- 页面打开时只有它的按键起作用，提示行也是它自己的（[按键](keys.md)）。
- 看板设置只属于你，不在这一页：在你的那页（下文）。另一台机器上的任务在那里改。归档任务的设置可读不可改。

### 你自己的设置

在本机的标题（或状态组的）上按 `Ctrl+E`，或 `?` 菜单里的 `your own settings`，为你的设置打开同样的页面：每个任务从什么开始，看板设置也在（`Settings  yours: what every task starts from`）。在仓库的标题上是你对该仓库的条目。按键就是本页的；`Enter` 像 `omh set` 一样保存，`Ctrl+U` 像 `unset`。`*` 标记你设的，其他行说明值从哪来（`built in`，仓库条目上是 `you`）。`Esc` 回列表。在另一台机器的标题上，看板说去那边改（[设置](settings.md)）。

## 面板里的协调者

在协调者的行上（机器的或项目的行），面板显示那个协调者，形式相同：

- **Status**：`State`（它的聊天，`idle` 或 `working`；[自动驾驶](autopilot.md)开着时是自动驾驶的词）、`Last`，它最新的消息按聊天的显示方式，开着时还有 `Goal` 和 `Today`（`2 of 5 added · 1 of 2 under way`）。
- **设置**：`Agent`（它的程序、模型和 effort），开着时还有 `Wakes`、`Limits`、`May set`、`May touch` 等，用朴素的话。`*` 标记这一次的值，下面一行 `* this time only`。
- 自动驾驶关闭时，它的一概不显示。
- 它的聊天运行中改动的 `coordinator.*` 值显示 `from its next fresh chat`，这里和它的设置页上都是。
- `Alt+T` 显示完整的它，开着时还有最近 20 次唤醒：何时、为何、是否应答了。

那里的 `Ctrl+E` 打开它的设置页，用本页的形式和按键：先是它自己的行，在 `This coordinator` 下，然后在本机的行上是你自己的设置，在项目的行上是你对该仓库的条目。自动驾驶开着时，在目标以外的自动驾驶行上按 `Enter` 只保存这一次，行下面一行说明。另一台机器协调者的页面完整显示，只读；改要到那台机器上。

## 评审们

**每场评审都保留，读起来是历史。**

- `History` 说明每场怎么样，从最旧开始（`fail → no verdict → pass`）。
- 一场评审是它名字下的行：标记和结果（`✓ Pass · 2 minor`、`✗ Fail · 1 critical`、`– No verdict`；颜色只是重复标记），时间，下面它的总结。
- **列表旁**显示最近一场，带第一份 `Evidence`（它的名字，整句折行）。全屏有更多内容时（更长的总结、发现、它检查了什么、更多证据、或地址），最后一行说明。
- **全屏**有全部，从最新开始，各带它的提交，以及作为独立事实的：
  - 它的 `Findings`（每条多严重，单独一列：`critical`、`major`、`minor`；没有拦住工作的标 `not required` 或 `pre-existing`）；
  - 它 `Checked` 了什么，留下什么 `Unverified`；
  - 每份**证据**在自己的 `Evidence` 标题下：名字、是什么、完整印出的地址，各自在下面折行，绝不截断。
- 没给结论的评审也有行，附原因。
- 评审门（gate）出现之前的评审保留它的评分和建议原样（`4/5, says merge`），以及它的 `Problems`。
- 合入 main 后仍然作数的评审，在结果下面说明。

`omh details <id>` 打印同样的列表，名为 `reviews`。

## 窄面板

面板太窄、值放不下标签旁边时（约 30 列以下），标签单独一行，值在它下面缩进。那里同样什么都不截断。

## 矮窗口

整行让路，最不需要的先走。状态和 `Now` 文字保持完整，**设置比任何文字都持久**：只要能显示就完整显示。

1. 顶上的空行。然后代理说的话和评审让出行数（评审先让总结，再让 `History` 和 `Evidence`），各让到几行。丢了行的文字以 ` …` 结尾。
2. 代理说的话、并行尝试、计划、报告消失。评审保留它的那一行（`Review 3  ✓ Pass · 2 minor`）。
3. 描述让出行数，直到一行说明去哪读（`Description · alt+t shows it`）。「需要你什么」剩头两行。然后描述那一行也走。
4. 现在才轮到设置。先走不是设置的行（`Project`、`Machine`、`Note`）。然后一次一行，每行整行走。留得最久的：带有人选过的值的行（点明它的位置），然后 `Agent`、`Reviewer`、`Review when`、`Send back`、`Merge`，然后其余。它们下面最后一行说明多少个设置没地方、按什么键看（`13 more settings · ctrl+e shows them`）。
5. 一个设置都放不下时，它们的标题行也走，其他部分拿回自己的行数：像 1 和 2 那样让路，文字里描述最后走。
6. 别处说过的事（`Task`、`On it`、`Made by`）、评审让到只剩一行、部分之间的空行、`Status` 标题行、标题的第二行；然后是 `Change`、评审的行、标题。

剩下的就是状态、进度和 `Now` 文字。连这些和最后一行（`alt+t shows the whole task`）都放不下时，那行走，然后进度，然后状态：文字本身是最后留下的。只有比整个面板还长的文字才被截断，最后一行以 ` …` 结尾。

## 一个画不出来的部分只影响它自己

它的标题行留下，下面一行红字（`✗ This part could not be shown: …` 加错误）。面板的其他部分都在，所有按键都工作。屏幕的每个区块同理：列表、面板、任务详情、菜单、聊天、输入框、提示。画不出来的就是在它位置上的那行红字，其他的留下。只有这些之外的失败才会用 `view error` 替换整个看板。

## 为什么长这样

面板显示一个议题和它下面的历史，正是议题或 PR 页面显示的东西。形式取自对那件事已经行得通的做法，适配到终端的文本格：

| 来源 | 取了什么 | 没取什么，为什么 |
|---|---|---|
| GitHub 的 PR 页面 | 先状态，再把检查和评审总结成标记、名字、结果的行；挡住合并的就在状态下面点名；时间线按最相关最新排。 | 头像、彩色药丸徽章、侧边一列：没地方也没像素。 |
| `gh pr view`、`gh pr status`、`gh pr checks` | 事实用标签加值的行；每次运行一行，带标记、结果和时间；结果用词说出来，颜色只是重复。 | 它用点连起来的一行（`Checks passing - Approved`）：那就是面板旧的顶部。 |
| lazygit、gh-dash | 每种状态一个强调色、安静的标签、选中项的详情在列表旁、全屏时独占一屏。 | 每个面板都套框、详情里再放标签页：一个框要花掉 38 列面板的两列两行。 |
| CodeRabbit、Greptile、Cursor Bugbot | 发现按严重程度单独一列；详情前一行结论；相对上一轮的改动作为历史行。 | Emoji 徽章、新评审的五分制评分（评审门的过与不过说得更多）、折叠区块。 |
| `/ui` 技能的准则 | 事实列表里标签比值更轻；最轻的有效分隔（空格，然后横线，默认绝不用卡片）；数字对齐。 | 一切关于像素、圆角和阴影的东西。 |
| skills.sh 上的 `tui-design` 技能 | 同一个事实在每种状态里位置稳定；每个颜色旁配标记或词；窄、常、宽三种宽度都试过；按优先级让路而不是截断。 | 用来强调的边框，以及它们的配色：看板沿用 Herdr 的。 |
