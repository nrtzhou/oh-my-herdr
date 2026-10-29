# 并行尝试与择一保留

**一个任务可以同时做多次，保留最好的结果。**「三选一」（一个代理试三次）和「委员会」（不同代理各试一次，再由裁判评判）本质相同：若干次尝试、谁来尝试、谁来挑选。

```
# best of three, by the usual agent; you pick
omh delegate 12 --attempts.by 3

# a council of claude, codex and pi, with a named judge
omh delegate 12 --attempts.by "claude, codex, pi" --attempts.pick reviewer --reviewer.agent claude/claude-opus-5-5

# two models for every task in one repo
omh set attempts.by "claude, codex/gpt-6.1-sol" --repo my-app

# which attempt to keep, when the task waits for you
omh pick 12 B
```

或者用大白话告诉[协调者](coordinator.md)：「这个做三遍让我挑」「让 Claude、Codex 和 pi 各试一次，让 Opus 来评判」。

## 两项设置

| 设置 | 取值 | 默认 | 含义 |
|---|---|---|---|
| `attempts.by` | 数量（`3`），或代理列表（`claude, codex, pi`；斜杠后可带模型；名字可重复） | `1` | 同时做几次尝试、由谁做。数量表示那么多 `worker.agent` |
| `attempts.pick` | `you`、`reviewer` | `you` | 在通过检查的尝试里谁来挑 |

- **最多 5 次。**每次尝试都是完整代理，有自己的工作树，占一个 `board.agents` 槽位，成本等于任务做一次。超过 5 会被拒绝，拒绝信息里写明 5。
- 它们和其它[设置](settings.md)一样：查找顺序 任务 → 你在该仓库的条目 → 你 → 默认；用 `set`（带 `--task`、`--repo` 或都不带）和 `unset`；传给 `add` 和 `delegate`；可由[自动化](automations.md)持有（`--automation`）；`show` 和任务设置页会列出。
- **仓库自己的文件不能设置它们**，代理写的代码开不了五次尝试。
- `attempts.by` 在任务启动时读取，`attempts.pick` 在尝试做完后读取。所以它们干活时你还能换挑选者。
- 不在仓库里的任务只做一次：没有工作树分给每次尝试。

## 流程

```
attempts fan out (attempts.by)
  each attempt: build ─► check
─► pick (attempts.pick; the reviewer's comparison)
─► the chosen one: review loop ─► push, pull request ─► merge gate (merge.by)
```

1. **每次尝试各干各的。**它有自己的 git 工作树、自己的分支（`work/12a-…`、`work/12b-…`）和任务 Herdr 空间里自己的标签页（`Agent A`、`Agent B`；[任务的标签页](tabs.md)）。代理只收到任务本身，不知道其它尝试。每次尝试占一个 `board.agents` 槽位：三次尝试只有两个空槽时，第三次等待，行上会说明（`C claude waits for a slot`）。
2. **检查负责筛选。**每次尝试走常规的前几步：提交工作成果、合入 main、跑仓库的检查，失败时退回代理，最多 `review.rounds` 次。检查之后仍失败、没留小结就停下、被停止、或代理没了，这次尝试出局。向你提问的尝试像任何代理一样在自己的标签页里等你回答。
3. **挑选者**在通过的尝试里选择：
   - **一个不剩：**任务失败（`No attempt passed`）。每次尝试的工作树、分支和标签页保留供查看。重新派发会用全新的尝试重来，旧的移除。
   - **只剩一个：**它胜出，不问任何人。
   - **多个且 `attempts.pick you`：**任务停在 **挑选一个（pick one）** 状态。有评审者时（`review.when` 不是 `never`），它先在名为 `Picker` 的标签页里比较各次尝试：读每个改动、逐个试用、说各自的优劣、推荐一个。由你选。
   - **多个且 `attempts.pick reviewer`：**评审者以同样方式比较，它点名的那个自动继续。它没点名（失败或超时）、或没有评审者时，任务等你挑选并说明原因。
4. **只有选中的那个被评审。**它成为任务：从那一刻起，它的工作树、分支、代理和对话都是任务的。只有它走完整的评审循环、被推送、开 PR 并合并，与只做一次的任务完全一样（[工作如何完成](how-work-gets-done.md)）。同一提交上它的检查不重跑，除非期间 `check.command` 变了。对每次尝试都做完整评审会把最贵的步骤乘上倍数，而那些工作是要扔的。
5. **挑选之后**，其它尝试的标签页关闭。它们的工作树和分支保留到任务完成，随后一并处理（`workspace.cleanup`）。落败尝试提交的东西随之消失。有任何从未提交文件的工作树会连同分支保留，任务说明在哪。被忽略的文件也算（草稿，也包括 `node_modules` 或构建产物）：git 分不清它们，所以这类东西一律不删。

`merge.by itself` 加 `attempts.pick you` 时，任务仍停下等你挑，然后自行合并。`attempts.pick reviewer` 时全程无人打扰。

两个尝试的成果绝不会拼在一起进仓库，也没人抢跑：每次尝试都做完，保留最好的，不是最先完成的。

## 在看板上

- **一行。**任务名后的标签注明尝试次数（`×3`）。下面一行每次尝试一个标记：`A claude ✓ · B codex working 60% · C pi ✗ check`。[观察者](watcher.md)跟踪每次尝试的代理；行上的进度条是它们的平均值。
- **挑选一个。**等你选择的任务挂在 **需要你** 下，显示 `Pick one of 3`，有评审者推荐时一并显示。Herdr 侧栏里任务的空间同样说明，每个通过的尝试显示 `pick one`。
- **任务面板**有 **Attempts** 部分（等待时标题为 **Pick one of 3**）：顶部是评审者的比较和推荐，然后是每次尝试的代理、状态、耗时、token 用量、改动大小、评审者对它的评价，以及 diff 链接（只研究任务则是报告链接）。`›` 标出选中的尝试，`★` 标出评审者的选择或最终保留的那个。
- **按键。**
  - `Ctrl+N` 切到下一次尝试。
  - `Ctrl+L`，批准键，保留选中的那个。会先询问。
  - `Alt+O` 进入选中尝试的标签页。
  - 尝试进行中给任务的回复会发给仍在进行的每次尝试。`Ctrl+X` 全部停止。
- **网页看板**显示同样的行和部分，同样的按键以按钮形式给出（`Next attempt`、`Yes`）。
- `omh show <id>` 列出每次尝试的代理和选中的那个。`details <id>` 有各次尝试和比较结果。

## 只研究任务

[只研究任务](research.md)也用并行尝试：每次尝试一份报告，各自放在报告区里该尝试自己的文件夹。挑选者读报告而不是 diff。

- 评审者挑选时（`attempts.pick reviewer`），还会被要求把**所有报告合成一份**写进任务自己的文件夹：各取精华、共同点、分歧所在。那就是任务的报告。
- 你挑选时，选中尝试的报告就是任务的报告。

## 与其它设置的关系

并行尝试位于启动与评审之间：

- [计划](plan.md)只写一次、批准一次，在尝试展开之前。
- [备用代理](fallback.md)在单次尝试内部接管，一个接一个，且仅在失败时。尝试是并排的、始终并行。
- [限额](limits.md)：所有尝试是一个任务。`limit.time` 对每次尝试自己的运行生效，`limit.tokens` 对所有尝试和任务合计生效。
