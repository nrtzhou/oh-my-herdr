# 代理程序

**每个角色都能用任何 Herdr 能启动、且有对应集成的程序。** Claude Code、Codex、pi、omp 能做看板的一切。其他程序也能用，只有几处限制（见下）。

```
omh set reviewer.agent agy/gemini-3-pro
omh set worker.fallback "codex, copilot/gpt-5.5"
omh delegate 12 --worker.agent qwen --reviewer.agent cursor
```

模型写在斜杠后面（[模型与思考力度](models.md)）。挑选程序的设置是 `worker.agent`、`reviewer.agent`、`coordinator.agent`、`watcher.agent`、`plan.agent`、`worker.fallback`、`reviewer.fallback` 和 `attempts.by`。

程序有：Claude Code（`claude`）、Codex（`codex`）、pi（`pi`）、Antigravity CLI（`agy`）、GitHub Copilot CLI（`copilot`）、Cursor Agent（`cursor`）、Devin（`devin`）、Factory 的 Droid（`droid`）、Grok（`grok`）、Hermes（`hermes`）、Kilo Code（`kilo`）、Kimi Code（`kimi`）、Letta Code（`letta`）、MastraCode（`mastracode`）、oh-my-pi（`omp`）、OpenCode（`opencode`）、Qoder（`qodercli`）和 Qwen Code（`qwen`）。Herdr 的集成名和程序自己的名字也能用（`antigravity-cli` 就是 `agy`，`cursor-agent` 就是 `cursor`）。

## 程序首次接任务前

**自己先启动一次**，在终端里，进某个仓库的文件夹：登录，完成首次设置，问是否信任文件夹就信任。

原因：Herdr 把登录页当作停着的程序，看板会把任务打进登出的程序，像它已就绪一样。这里试过 Cursor、Devin、Grok、agy 和 Letta，全都停在登录或设置页。

新工作树里的文件夹信任对话框和其他对话框没两样：任务会「需要你」（`needs you`）。Claude Code、Codex 和 pi 会被提前告知信任该文件夹（[工作如何完成](how-work-gets-done.md)）。Codex 那边，看板往 `~/.codex/config.toml` 加一张 `[projects."<repo>"]` 表，只在文件可读、且该仓库以任何形式（任意引号、点号键）都没有条目时才加；写着 `untrusted` 的条目原样不动。pi 信任一个文件夹及其里面的文件夹，只在该文件夹有自己的 pi 设置（`.pi/settings.json`、`.pi/extensions` 等）时才问：看板把 pi 启动所在的文件夹（任务的工作树）加进 `~/.pi/agent/trust.json`，除非 pi 对它或其上层已有决定、文件读不出、或 pi 正在写它。

启动仍然失败的话，任务会写明原因：[代理无法启动时](could-not-start.md)。

## 程序如何启动

按 Herdr 的方式启动（`herdr agent start --kind <name>`），后面跟看板自己的参数。Herdr 在面板里找到它，并说它是在干活还是在等。

文末的表格说明每个程序怎么启动、能做什么。它从代码生成（`crates/core/src/agents/kinds/`），所以永远和看板的行为一致。加一个程序就是在那里加一行。

## 辅助代理（子代理）

任务里的代理会像在看板外一样，用程序自带的辅助代理（Claude Code 的 subagents、Codex 的 spawned agents、opencode 的 tasks，以及用到它们的技能）。看板的参数不限制任何工具，规则只说工作不能交给看板上的另一个任务或代理：辅助代理是点名允许的，任务或技能要求时也一样。只被告知“绝不交给别人”时，Claude 和 Codex 理解成不要辅助代理，即使任务要求也拒绝（HP-53）。测试同时盯住两者：`tests/agent_helpers.rs`。

## 看板知道每个程序的什么

看板读取 Claude Code、Codex、pi、omp 保存的会话记录。所以它知道它们说了什么、服务商出了什么错、用了多少 token（[工作如何完成](how-work-gets-done.md)）。

其他程序的会话它不读。对它们，下面每条信息另有来源：

- **回合结束**来自 Herdr：程序从干活变成停着（它的 `completion_seq`）。
- **代理说了什么**是它的**收尾小结**：看板让每个代理停下时写下的文件。没写就停下的，以 `Stopped without a summary` 等待，附屏幕上最后的话。
- **评审**以评审者的结论文件结束，和所有程序一样。评审者没有结论文件就停下的，Herdr 一看见它停下（15 秒后）就结束这一轮，不等评审的全部时长。
- **和代理的聊天**显示它屏幕上的话，不是它的会话记录（[看板](board.md)）。
- **重新启动**后（你停了它，或计划获得同意后），它会尽其所能重开自己的会话：按 Herdr 集成报告的 id（安装：`herdr integration install <name>`），或按看板启动时给它的 id（copilot、grok、qodercli、qwen 接受一个）。两样都没有，就用任务和它上次说的话开新会话。
- **常驻规则**写进它的第一条提示词，启动时读规则文件的程序除外（droid、omp）。

这些程序**得不到**的东西：

- 不计 token（`limit.tokens` 对它们写着 `does nothing`）。
- 分不清服务商的暂时错误和永久错误。所以看板不等也不重试，备用代理也不替它接手。登出画面仍然会交接。
- 不追踪代理启动的辅助代理。

## 什么会被拒绝，为什么

干不好某个角色的程序，在你设置时就被拒绝，附原因和能用的程序：

- **MastraCode 接不了任何任务**（`worker`、`reviewer`、`coordinator`、`plan`、备用代理和并行尝试）。启动时既不能告诉它用哪个模型，也不能让它不问就干（只有环境变量做得到，Herdr 的启动设不了）。而且它会重开文件夹里最后一个会话，评审者会接进工作代理的会话。它可以当观察者。
- **观察者**需要无头一次性调用（one-shot）：答一次就结束的命令。这里的每个程序都有；没有的会被拒绝。
- **模型**：给启动干活时不接受模型的程序设模型：Droid。在 Droid 自己的设置里选模型。它的一次性调用接受模型，所以 `watcher.agent droid/<model>` 可用。
- **程序不接受的模型名**，并附它接受的名字。从程序自己的列表读（`codex debug models`、`pi --list-models`、`omp models`、`cursor-agent models`、`opencode models`），没有列表的从表格里写的名字读（Claude Code）。`omh models` 列出它们（[模型与思考力度](models.md)）。
- **程序没有的思考力度**：不在程序给出的列表上的级别（Codex 的目录，按模型；其他程序的 `--help`；读不到就用表格里的），或启动时不接受力度的程序的任何级别（Herdr 的其他大多数）。Cursor 的力度写在模型名里：`cursor/claude-opus-4-8[effort=high]`。
- **`access ask`**：给从不先问再动手的程序（pi）。
- **`access auto`**：给没有自己的 auto 模式（auto mode）的程序（除 Claude Code、Codex、Cursor、Qwen Code、Qoder 之外的全部），以及启动时版本比它首个支持版本旧的（[auto 模式](auto-mode.md)）。

还有两条在程序启动时说，不是设置时：

- **omp 和 MastraCode 需要 Herdr 自己的集成**，否则 Herdr 看不出它们是否在干活。看板不会替你装进代理的设置。它让你去装：`omp needs herdr's own integration, or herdr cannot see when it works: run herdr integration install omp, then try again.`
- **Herdr 看不见的程序。** 用 npm 装的 Kilo Code 和 Qoder 藏在一个小 node 启动器后面，真正的程序是它的子进程，Herdr 0.9.3 在面板里找不到它们。启动失败：`herdr did not see kilo come up in its pane …`。改用程序的独立安装包（试过：Kilo 自己的二进制一次就能找到）。

## 不支持

Herdr 也能启动 Gemini CLI、Cline、Kiro、Amp、Maki 和 Muse，但没有它们的集成（称它们测试较少）。看板不收它们。每个都只是表格里再加一行。

## 程序一览

<!-- made:agents Made by `omh docs` from crates/core/src/agents/kinds/. Do not edit between these two lines. -->
| Program | Started for work as | Model | Effort | Asking first | Auto mode | Standing rules | Reopened by | Watcher's one-shot | Turn's end read from | herdr's integration | Can do | Checked |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `claude` | `claude --dangerously-skip-permissions` | `--model <model>`; `default`, `fable`, `opus`, `sonnet`, `haiku`, `opusplan` or one of its full names in the table (`claude-fable-5-1`, `claude-fable-5`, `claude-opus-5-5`) | `--effort <level>`; the levels `claude --help` lists, else low, medium, high, xhigh, max | its own default | `--permission-mode auto`, from 2.1.111 | `--append-system-prompt-file <file>` | `--resume <id>` | `claude -p --no-session-persistence --tools ""`, the prompt on its input | its conversation file | `claude` | every role | everything the board does, on 2.1.296 to 2.1.300 |
| `codex` | `codex --dangerously-bypass-approvals-and-sandbox --no-daemon` | `-m <model>`; the names `codex debug models` lists | `-c model_reasoning_effort="<level>"`; the levels `codex debug models` lists for the model, else low, medium, high, xhigh, max, ultra | `--no-daemon` | `--approve-for-me --no-daemon`, from 0.147.0 | `-c developer_instructions=…` | `resume <id>` | `codex exec --skip-git-repo-check --ephemeral --color never <prompt>` | its conversation file | `codex` | every role | everything the board does, on 0.153.4 to 0.162.1 |
| `pi` | `pi` | `--model <model>`; the names `pi --list-models` lists | `--thinking <level>`; the levels `pi --help` lists, else off, minimal, low, medium, high, xhigh, max | cannot ask | none | `--append-system-prompt <file>` | `--session-id <id>` | `pi -p --no-session --no-tools --no-skills --no-context-files -- <prompt>` | its conversation file | `pi` | every role | everything the board does, on 1.1.0 |
| `omp` | `omp --auto-approve` | `--model=<model>`; the names `omp models --json` lists | `--thinking=<level>`; the levels `omp --help` lists, else off, minimal, low, medium, high, xhigh, max, auto | `--approval-mode=write` | none | `--append-system-prompt=<file>` | `--resume=<id>` | `omp -p --no-session --no-tools --no-skills --no-rules <prompt>` | its conversation file | `omp`, needed | every role | everything the board does, on 18.13.3 to 18.14.0 |
| `agy` (also `antigravity-cli`, `antigravity`) | `agy --dangerously-skip-permissions` | `--model <model>`; not checked | `--effort <level>`; the levels `agy --help` lists, else low, medium, high, xhigh, max | its own default | none | in its first prompt | `--conversation <id>` | `agy --disable-slash-commands -p <prompt>` | herdr's state and its closing note | `antigravity-cli` | every role | flags from agy --help 1.3.6; started by herdr 0.9.3 on the board's flags (it came up at its login) |
| `copilot` | `copilot --yolo` | `--model <model>`; not checked | `--reasoning-effort <level>`; the levels `copilot --help` lists, else none, minimal, low, medium, high, xhigh, max | its own default | none | in its first prompt | `--resume=<id>` | `copilot -s --no-custom-instructions --no-ask-user --allow-all-tools -p <prompt>` | herdr's state and its closing note | `copilot` | every role | flags from copilot --help 1.0.98; started by herdr 0.9.3 on the board's flags (its folder-trust dialog) |
| `cursor` (also `cursor-agent`) | `cursor-agent --yolo` | `--model <model>`; the names `cursor-agent models` lists | cursor takes its effort in the model's name: cursor/claude-opus-4-8[effort=high] | its own default | `--auto-review`, from 2026.06.22 | in its first prompt | `--resume <id>` | `cursor-agent -p --output-format text --mode ask --trust <prompt>` | herdr's state and its closing note | `cursor` | every role | flags from cursor-agent --help 2026.10.01; started by herdr 0.9.3 on the board's flags, given a task |
| `devin` | `devin --permission-mode dangerous` | `--model <model>`; not checked | none | its own default | none | in its first prompt | `--resume <id>` | `devin --respect-workspace-trust false -p <prompt>` | herdr's state and its closing note | `devin` | every role | flags from devin --help 3000.11.92; started by herdr 0.9.3 on the board's flags (it came up at its login) |
| `droid` | `droid --auto high` | none at start | none | its own default | none | `--append-system-prompt-file <file>` | `--resume <id>` | `droid exec <prompt>` | herdr's state and its closing note | `droid` | every role | flags from droid --help 0.235.0; a task on 0.242.0 in herdr 0.9.3: started, prompted and its turn's end seen |
| `grok` | `grok --always-approve` | `-m <model>`; not checked | `--reasoning-effort <level>`; the levels `grok --help` lists, else any level | its own default | none | in its first prompt | `--resume <id>` | `grok -p <prompt>` | herdr's state and its closing note | `grok` | every role | flags from grok --help 1.0.52; started by herdr 0.9.3 on the board's flags (it came up at its login) |
| `hermes` | `hermes chat --yolo` | `-m <model>`; not checked | none | its own default | none | in its first prompt | `--resume <id>` | `hermes --ignore-rules -z <prompt>` | herdr's state and its closing note | `hermes` | every role | flags from hermes --help 0.19.0; started by herdr 0.9.3 on the board's flags (its first setup) |
| `kilo` | `kilo --auto` | `-m <model>`; not checked | none | its own default | none | in its first prompt | `--session <id>` | `kilo run <prompt>` | herdr's state and its closing note | `kilo` | every role | flags from kilo --help 7.8.12; started by herdr 0.9.3 on the board's flags as its own program (npm's launcher hides it from herdr) |
| `kimi` | `kimi --auto` | `-m <model>`; not checked | none | its own default | none | in its first prompt | `--session <id>` | `kimi -p <prompt>` | herdr's state and its closing note | `kimi` | every role | flags from kimi --help 2.2.0; started by herdr 0.9.3 on the board's flags (its folder-trust dialog) |
| `letta` | `letta --new --yolo` | `-m <model>`; not checked | none | its own default | none | in its first prompt | `--conversation <id>` | `letta --ephemeral -p <prompt>` | herdr's state and its closing note | `letta` | every role | flags from letta --help 0.34.15; started by herdr 0.9.3 on the board's flags (its first setup held it) |
| `mastracode` | `mastracode` | none at start | none | its own default | none | in its first prompt | starts afresh | `mastracode --permission-mode deny --prompt <prompt>` | herdr's state and its closing note | `mastracode`, needed | Watcher only: mastracode can be told neither its model nor to work without asking when it starts (only through environment variables, which herdr's start cannot set), and it reopens its folder's last conversation, so it cannot work on a task: it can be the Watcher | flags from mastracode's own source 0.47.0 |
| `opencode` | `opencode --auto` | `-m <model>`; the names `opencode models` lists | none | its own default | none | in its first prompt | `--session <id>` | `opencode run <prompt>` | herdr's state and its closing note | `opencode` | every role | flags from opencode --help 1.18.34; started by herdr 0.9.3 on the board's flags; OpenCode 2.1.4's conversation made on a model and opened by hand |
| `qodercli` | `qodercli --dangerously-skip-permissions` | `-m <model>`; not checked | `--reasoning-effort <level>`; the levels `qodercli --help` lists, else any level | its own default | `--permission-mode auto`, from 0.2.14 | in its first prompt | `--resume <id>` | `qodercli -p --no-session-persistence --tools "" -- <prompt>` | herdr's state and its closing note | `qodercli` | every role | flags from qodercli --help 1.1.72; npm's launcher hid it from herdr 0.9.3 |
| `qwen` | `qwen --yolo` | `-m <model>`; not checked | none | its own default | `--approval-mode auto`, from 0.16.0 | in its first prompt | `--resume <id>` | `qwen -p <prompt>` | herdr's state and its closing note | `qwen` | every role | flags from qwen --help 0.25.2; started by herdr 0.9.3 on the board's flags |
<!-- /made:agents -->
