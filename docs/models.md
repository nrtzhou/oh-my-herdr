# 模型与思考力度

**模型写在程序后面，用斜杠分开：`codex/gpt-6.1-sol`。** 不写就用程序自己的默认。思考力度（代理思考得多用力）是独立的设置。名字或级别写错，设置时就被拒，并给出能用的。

```
omh models            every program, a few names each, and its effort levels
omh models codex      every name codex takes, written as a setting writes them, and its effort levels
```

看板启动的每个代理都有一个**程序**，还可以有一个**模型**和一个**思考力度**。按角色选：工作代理（`worker.`）、评审者（`reviewer.`）、协调者（`coordinator.`）、计划撰写者（`plan.`）、观察者（`watcher.`），以及点名代理的列表（`worker.fallback`、`reviewer.fallback`、`attempts.by`）。

## 示例

```
omh set worker.agent codex/gpt-6.1-sol          you, for every task
omh set worker.effort high
omh set reviewer.agent claude/opus --repo my-app  one repo
omh delegate 12 --worker.agent pi/anthropic/claude-opus-5-5 --worker.effort medium   one task
omh set worker.fallback "codex, claude/sonnet"
omh coordinator new --agent claude/sonnet --effort low   the next Coordinator only
```

也可以用自己的话吩咐协调者（“这个用 Opus、高力度做”“让 Codex 评审”）：它用 `omh models` 查名字，用同样的方式设置。它只写那个列表里的名字（“sol 6.1” 就是 `gpt-6.1-sol`，不然就问你）；你不提模型它就不设：程序用自己的默认。多个地方都设了值时以谁为准：[设置](settings.md)。

## 模型

模型跟着程序走，Claude 的模型永远不会交给 Codex。名字用**程序自己的**，各程序命名方式不同：

| 程序 | 写法 | 它的名字 |
|---|---|---|
| `claude` | `claude/opus`、`claude/claude-opus-5-5`、`claude/claude-opus-5-5[1m]` | 短名（`default`、`fable`、`opus`、`sonnet`、`haiku`、`opusplan`：各是同类中最新的）或它的全名之一（`omh models claude`） |
| `codex` | `codex/gpt-6.1-sol` | 当前登录账号在 Codex 列出的名字（其 app-server 的 `model/list`；问不到时用 `codex debug models`） |
| `pi` | `pi/anthropic/claude-opus-5-5`（先写服务商）、`pi/anthropic/claude-opus-5-5:high` | `pi --list-models` 列出的名字 |
| `omp` | `omp/openai/gpt-5.2`、`omp/@smol` | `omp models --json` 列出的名字（写它的 selector）；`@名字` 是 omp 自己的模型角色（它的 `modelRoles` 设置里定义），照原样交给 omp，不检查 |
| `cursor` | `cursor/claude-opus-5-5-high`、`cursor/claude-opus-4-8[effort=high]` | `cursor-agent models` 列出的名字；列表里的模型名去掉最后一段后可用方括号带参数 |
| `opencode` | `opencode/anthropic/claude-opus-5-5`（先写服务商） | `opencode models` 列出的名字 |
| `droid`、`mastracode` | 不写模型 | 在程序自己的设置里选 |
| 其他 | `agy/gemini-3-pro` | 不检查：程序自己的 help 给出的名字 |

OpenCode 2 的全屏应用不接受 `-m`（一给就退出），所以看板先在启动目录里按模型建好会话（`opencode api session.create`），再打开那一个：`opencode --auto --session <id>`。重开的会话保留原有模型。OpenCode 1 照旧给 `-m <model>`（#394）。

### 写错的名字当场被拒

不是等代理启动失败才发现。回答会说明程序接受什么，以及最接近的名字：

```
$ omh set worker.agent codex/sol-6.1
worker.agent: codex has no model "sol-6.1" for your ChatGPT sign-in (pro plan). Did you mean gpt-6.1-sol? It takes
gpt-6.1-sol, gpt-6-astra, gpt-6-sol, … (codex's own list for that sign-in). Write it after the program: codex/gpt-6.1-sol.
```

- **列表是向程序问的**，用你登录的账号，保存一小时。不在列表上的名字先再问一次，程序刚加的模型立刻可用。
- **Codex 的名字取决于登录方式。** ChatGPT 登录和 API key 接受的名字不同，在别处没问题的名字会在第一回合被 Codex 拒绝（“The 'sol-6.1' model is not supported when using Codex with a ChatGPT account”）。所以看板通过 app-server 直接问 Codex：这次登录能用哪些名字、是哪种登录（`omh models codex` 会说明）。app-server 问不到时（旧版 Codex），读 `codex debug models`。
- pi、cursor、opencode 只列出自己的登录和 key 能到的模型，所以列表本来就是你的。
- **列表读不到时**（程序装了但没登录、离线、或列表命令失败），名字同样被拒，附程序说的话。在终端跑它的列表命令修好，或者不写模型。
- **Claude Code 没有列模型的命令**，所以它的名字写在看板的程序表格里。比表格新的 Claude 模型会被拒，直到看板更新。它的短名（`claude/opus`）永远指同类中最新的。

### 代理运行中被拒绝的模型

名字仍可能在代理运行时被拒：手写在设置文件里的值（只检查正在设置的那个）、登录变了、另一台机器上的程序、或程序自己的默认模型。看板在回合结束时接住，绝不把任务连同一个错误留在代理里：

- **它把模型给了程序：**同一个程序立刻用自己的默认模型重启，在同一工作树，被告知已完成的部分。行上写着 `gpt-6.1 can't be used with your ChatGPT sign-in: started again on codex's own default model`，任务面板列在 “Took over” 下。评审者同理：评审用程序默认模型重来。
- **默认模型也被拒，或它没给模型：**任务等你，附原因和一个可试的名字（`Agent stopped: gpt-6.1 can't be used with your ChatGPT sign-in; try gpt-6.1-sol`），面板里有下一步：为任务选一个（`omh set worker.agent codex/gpt-6.1-sol --task <id>`），或修好程序自己的默认（Codex：`~/.codex/config.toml` 里的 `model`），然后 `Ctrl+L`（重试）。程序已被结束，重试会按当时设置的模型重新启动它。
- 设了 `worker.fallback`（或 `reviewer.fallback`）时，默认模型之后轮到列表上的下一个（[备用代理](fallback.md)）。

**这些情况不检查名字**（原样传给程序）：

- 程序没有列表，表格里也没写名字；
- 程序没装在这里；
- 程序指向自己的服务商：Claude Code 用 `ANTHROPIC_BASE_URL` 或云（`CLAUDE_CODE_USE_BEDROCK`、`…_VERTEX`、`…_FOUNDRY`），或 Codex 在 `config.toml` 里配了 `model_provider`。

## 思考力度

`<role>.effort` 是代理思考得多用力：`default`（程序自己的）或一个级别，如 `low`、`medium`、`high`。

- **每个程序有自己的级别，看板向程序问**，和模型名一样：
  - Codex：`codex debug models` 为每个模型列出的（目前 `low` 到 `max`，部分模型有 `ultra`）。级别按你选的模型检查（`codex/gpt-6-luna` 没有 `ultra`）；不写模型时，按所有模型的全部级别检查。
  - 其他：它们的 `--help` 在力度参数处列出的（`claude --help`：`low` 到 `max`；`pi --help`：`off` 到 `max`）。
  - 级别保存一小时。不在列表上的级别先再问一次，程序刚加的级别立刻可用。
  - **读不到时**（程序没装在这里、离线、或 help 没列），用看板程序表格里写的级别（[agents 里的表格](agents.md#程序一览)）。绝不会拒绝所有级别。
- 角色的程序（或模型）没有的级别被拒，附它有的：

  ```
  $ omh set worker.effort minimal
  worker.effort: codex has no effort "minimal": it takes low, medium, high, xhigh, max, ultra, as `codex debug models` lists them (or default, for its own).
  ```

- 启动时不接受力度的程序只接受 `default`。
- Cursor 的力度写在模型名里（`cursor/claude-opus-4-8[effort=high]`）。pi 也接受跟在模型后面的（`:high`）。
- 给一个程序设的力度，在你换程序或换模型后保留；新程序没有它时，启动时忽略。启动按程序上次给出的级别走：绝不等待询问。
- `<role>.fast` 是 Codex 更快的服务档位。别处不起作用。

## 新增程序

一个程序的模型怎么列、怎么读，是程序表格那一行里的一个条目（`crates/core/src/agents/kinds/`，`models`）：列出它们的命令及格式（还有 Codex 的 app-server，`server`），或者它的短名和全名。两者都没有的程序不检查。思考力度级别从它的 `--help` 读，除非它的行另有说明（`efforts_read`：Codex 的目录，或只有表格）；`efforts` 是读不到时使用的级别。
