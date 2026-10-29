# 聊天里使用技能

**在看板的输入框里敲 `/` 用技能**，和 web 界面（Ovis）的聊天框一样。输入框上方的列表显示聊天对象的技能和命令及用途。选一个发送：它的程序执行它。

```
 ◐ PRJ-34 Fix the login  working
 3 skills and commands
▌  /plan-day   Plan the day
   /review-pr  Review a pull request
   /ship       Ship the release
────────────────────────────────────────────────────────────────────────────────
❯ /
────────────────────────────────────────────────────────────────────────────────
  ↑↓ pick a skill · tab enter put it in the box · esc close the list
```

## 用法

- **在消息开头敲 `/`**。列表显示该程序提供的东西。继续敲缩小范围：`/rev` 列出 `/review-pr`。以你敲的字开头的名字排最前，然后是某个词以它开头的，最后是包含它的。插件的名字（`/tools:lint`）按自己的部分也能搜到（`/lint`）。
- **`↑` `↓`** 选择。选中的高亮。
- **`Tab`** 或 **`Enter`** 把它放进输入框，后面带个空格。要指定作用对象就接着敲，再按 `Enter` 发送。
- **对已经选中的词按 `Enter`**（整词敲完的 `/ship`）立即发送。
- **`Esc`** 关掉列表，保留你敲的字。改词会重新列。再按一次 `Esc` 收起草稿，一如既往。
- 列表只对以 `/` 开头、且还只有一个词的消息出现。靠后的 `/`（`look at /tmp`）不列任何东西。

你发的字逐字送到代理，`/review-pr the login branch` 原样到达。认识的技能或命令不用列表也行：敲了就发。

## 用谁的技能

列表属于字要送去的程序，在它运行的文件夹里读：

| 你和谁说话 | 它的列表 |
|---|---|
| 任务的代理（任务上按 `Space`） | 它程序的技能，在任务的工作树里读（还没提交的技能也从仓库主目录读） |
| 协调者 | 它程序的技能，在协调者自己的文件夹（不含仓库的） |
| 仓库的协调者（仓库标题上按 `Space`） | 它程序的技能，带那个仓库的 |
| 机器的协调者 | 它程序的技能，在它自己的文件夹 |

字要送去另一台机器（技能在那台机器上）、还没有代理的任务、或没有协调者（你的字会加成一个任务）时不显示列表。你敲的字照样原样送出。

## 各程序提供什么

和 web 界面（Ovis）列的一样，从同样的地方读：

| 程序 | 它列什么 |
|---|---|
| Claude Code | `/name`：你的技能（`~/.claude/skills`，或 `$CLAUDE_CONFIG_DIR`）、项目的（到仓库顶层各层的 `.claude/skills`）、你和项目的命令（`git/commit.md` 就是 `/git:commit`），以及启用的插件的（`/plugin:name`）。标了 `user-invocable: false` 的技能不列 |
| Codex | `$name`：项目的 `.agents/skills`、你的 `~/.agents/skills` 和 `~/.codex/skills`（或 `$CODEX_HOME`）、它自带的，以及 `config.toml` 里启用的插件的（从插件缓存）。在那里关掉的技能（`[[skills.config]]`，`enabled = false`）不列。你的提示词作 `/prompts:name` |
| pi | `/skill:name`：你的（`~/.pi/agent/skills`、`~/.agents/skills`）和项目的（`.pi/skills`、`.agents/skills`）；提示词模板作 `/name` |
| OpenCode | `/name`：`opencode.json(c)` 和 `commands` 文件夹的命令、自带的技能（`customize-opencode`）、它自己文件夹的技能、`~/.claude/skills`、`~/.agents/skills` 和项目的 |
| Devin | 按技能文件夹定的 `/name`：Devin 自己的技能文件夹、Windsurf 的、Claude Code 的和 Copilot 的（按它的 `config.json` 允许读的），插件的作 `/plugin:name`。两处同名的会标明出处：`/claude:review` |
| Grok | `/name`：项目的 `.grok/skills` 和 `.claude/skills`、你的 `~/.grok/skills`（或 `$GROK_HOME`）和 `~/.claude/skills`、`config.toml` 里的路径、插件的。git 或它的 `[skills] ignore` 排除的不列 |
| Hermes | 用它自己拼法的 `/name`（`Web Tool` 是 `/web-tool`）：`~/.hermes/skills`（或 `$HERMES_HOME`）和其他技能文件夹、项目仅在其 `config.yaml` 信任时、技能包和快捷命令 |
| omp | `/skill:name`，来自它读的每个地方（自己的文件夹、Claude Code 的、`.agents`、Codex 的、OpenCode 的、`.github`、启用的 Claude Code 插件、它的 `settings.json` 指名的扩展和 npm 插件），按它的 `config.yml` 允许；命令和提示词作 `/name` |
| Antigravity（agy） | `/name`：项目的 `.agents` 技能（和它 `skills.json` 指名的）、你的 `~/.gemini/config/skills` 和插件的，以及它自带的 |

- 每个技能第一句就是用途：描述的其余部分说何时用，那是给代理的。
- 程序自带的命令（`/model`、`/compact` 等）不列。敲了能用。
- 其他程序（Copilot、Cursor、Droid、Kilo、Kimi、Letta、Mastra Code、Qoder、Qwen）什么都不列，和 web 界面一样。你敲的字照样原样送给它们。
- 新增或修改的技能几秒内就会出现。
