# 备用代理：换一个代理接手

**代理进行不下去时（额度用完、登出、服务商挂了），另一个代理可以接手。** 用订阅账号的代理多了，这是夜间工作停下的常见原因。两个设置：

- **`worker.fallback`**：工作代理进行不下去时，谁接任务。
- **`reviewer.fallback`**：评审者不行时，谁来评审。

各是一张代理列表，按序尝试，写法同 `worker.agent`，逗号分开（`codex, claude/claude-opus-5-5`：斜杠后是模型），或 `none`。`none` 是内置：没列表就什么都不变。**一切顺利时不花钱**：没有交接就没有多余的代理跑。

```
omh set worker.fallback "codex, claude/claude-opus-5-5"   for every task of yours
omh set worker.fallback codex --repo my-app               for one repo
omh set worker.fallback none --task 12                    not for this one
omh delegate 12 --worker.fallback pi --reviewer.fallback claude
```

## 任务何时交接

工作代理**进行不下去**有三种情况：

1. **服务商挂了**：模型满载、服务器过载、限流、断连。这些会自己过去，所以看板先等，让同一个代理继续，隔 1、2、4、8 分钟（[被服务商中断的回合](provider-errors.md)）。只有第五次也被掐断才交接任务。
2. **额度用完，或登出**，或模型没了，或回复被永久截断（超长、拒绝）。等待修不好这些，所以绝不重试：任务立刻交接。看板给的模型被程序拒绝时先走这一步：同一个程序用自己的默认模型重启，也被拒才交接（[模型](models.md#代理运行中被拒绝的模型)）。
3. **它的工作过了轮数还是失败。** 失败的评审退回代理 `review.rounds` 次。最后一次之后的评审仍失败，工作交给下一个代理。仓库的检查同理：退回 `review.rounds` 次，之后仍失败的检查就交接。`review.rounds 0` 时什么都不自动退回，评审或检查也不交接：等你。

所以一个糟糕回合的顺序是：**看板自己的重试（只对会过去的错误）→ `worker.fallback` 上的下一个代理 → 你。**

**不交接**，和以前一样：代理停下说等你、它没写收尾小结就停、你停了它、和 main 冲突或 PR 上有反馈、或你批准的合并正在修复。这些归你。

## 交接包含什么

列表上的下一个代理**在同一工作树**、同一分支、同一 Herdr 面板接任务。进行不下去的程序被结束，下一个从原处开始。它被告知：

- 任务，和第一个代理拿到的一模一样（标题、描述、看板的规则）；
- 它接手、从谁手里、为什么，看看现有的继续，不要重来；
- **到目前做了什么**：分支上的提交、有没有未提交的改动、上一个代理最后的话；
- **未决的发现**：上次评审找到的问题，或那次评审失败了，或失败检查的输出。

新代理：

- 用它的条目写的模型（`claude/claude-opus-5-5`），否则程序自己的；
- 接受任务的 `worker.effort` 和 `worker.access`，程序支持的话；
- 轮数从头算：上一个用掉的不算它的。

你之前给过的同意会被重新要一次。

### 谁是下一个

列表上第一个满足这些的：

- 不是任务上的代理；
- 你上次介入后没上过这个任务；
- 装在这台机器上；
- 能在任务的 access 下干活（`pi` 不会先问再动手，`worker.access` 是 `ask` 时跳过；没有自己的 auto 模式、或版本太老的，`auto` 时跳过）。

启动不了的也跳过。谁都起不来时，任务原样：同一代理、它的失败、用掉的轮数。

**列表用尽时，任务像没列表一样等你**：同样的“Agent stopped”“Needs a login”或失败的评审，外加任务面板里一条 `Note`，说明跳过了谁、为什么。

### 介入，与改列表

- 你一介入（回复、`Ctrl+R`、在代理的终端里打字），任务上的代理继续，列表可以从头再走一遍：那时额度可能回来了。
- 重新派发任务从 `worker.agent` 重新开始。
- 设置在交接那一刻读取。所以你可以在进行中的任务上设它，从下一次代理进行不下去时生效。

## 评审者

`reviewer.fallback` 对评审者同理，适用于这些情况：

- **服务商挂了**（看板自己的五次重试后）、额度用完或登出、或模型没了：列表下一个**以同一次评审**评审。被掐断的尝试不算评审，不耗轮数。
- **它的程序没装在这台机器上**：列表上第一个装了的评审。这是一次和其他一样的交接（`claude took over the review from codex: codex is not installed here`）。没有列表时，由工作代理的程序评审，和以前一样。

接手的评审者也评审该任务之后的轮次，直到你介入。列表用尽时，评审像没有列表一样失败。

**有一种情况不适用于评审者**：没有“失败 `review.rounds` 次”这一说。评审过并找出问题的评审者已做完它的工作。换人直到有人通过改动，是在挑结论。超时或没出结论就结束的评审（两次：第一次会再启动一次）和以前一样是失败的评审，不是交接：没有证据说换个程序会更好。

## 你会看到什么

- **行**在新代理起步时用大白话说交接，直到观察者写下第一行：`codex took over from claude: out of allowance until 17:00`。它的程序列显示现在的代理。
- **任务面板**在 `Status` 下保留每次交接，作 `Took over`（列表旁：最近一次；全屏：全部，各带时间）。`On it` 和 `Agent` 行是现在的代理。两者下面各有一行 `Takes over`，是它的列表。一旦评审者交接过，每次评审都写明谁做的。
- **历史**：`omh details <id>` 在每次评审旁列出每次交接（谁、从谁、为什么、何时），并说明现在谁在做。
- **Herdr**：交接后代理的标签页是 `Agent (codex)`，评审者是 `Reviewer #2 (claude)`，侧栏点名代理：`agent codex · working`、`reviewer claude · under review`。

原因用服务商自己的话说：

- `out of allowance until 17:00`，服务商说了恢复时间时（`resets 5pm`、`try again at 5:00 PM`、`try again in 2 hours`）；没说就是 `out of allowance`；
- `logged out`、`gpt-6.1 can't be used with your ChatGPT sign-in`、`claude can't use the model claude-x`、`its provider is overloaded, still after 5 tries`；
- `its work still failed review after 3 rounds`、`the check still failed after 4 tries`；
- 其他，按服务商原话。

## 和其他设置一样

两个都是工作设置（[设置](settings.md)）：

- 就近取值：任务 → 你对仓库的条目 → 你 → 内置；
- `set` 设置、`unset` 去掉、传给 `delegate`（和 `add --delegate`）作 `--worker.fallback <列表>`；
- [自动化](automations.md)可以带着它们；
- `show` 和 `show <id>` 列出它们和值的来源，看板设置页也有（`Ctrl+E`：`Agent's fallback`、`Reviewer's fallback`）。

名字写错会拒绝整个列表，什么都不改，并给出最接近的正确名。只点名现在就在做任务的代理的列表，等于什么都不做。

**仓库自己的文件不能设它们。** `.omh.json` 只放它的七项事实，不多，所以代理写的代码打不开也关不掉交接（[设置](settings.md)）。

## 与其他设置的关系

- **[并行尝试](attempts.md)。** 每个尝试像独立任务一样做，所以各有各的备用列表，一个尝试里的交接不改其他尝试。换过手的尝试被选中时，任务保留接手的代理、它的模型和交接记录。两者不同：交接是一个接一个、只在失败时、顺利不花钱；并行尝试是一起开、一直开。
- **[时间与 token 限额](limits.md)。** 触到限额的任务**不**交接。限额是你给任务花费设的上限，token 限额把所有代理一起计，换代理只会接着花。它等你，如该页所说。

## 给贡献者

交接是一个函数（`crates/flow/src/finish/hand_over.rs` 里的 `hand_over`），任何一步都能带着原因调它。

- `crates/core/src/store/handovers.rs`：任务上存什么、列表下一个、措辞；
- `crates/flow/src/finish/hand_over.rs`：工作代理的交接；
- `crates/flow/src/finish/review_fallback.rs`：评审者的交接；
- 测试：`tests/fallback.rs`、`tests/fallback_more.rs`、`tests/fallback_settings.rs`。
