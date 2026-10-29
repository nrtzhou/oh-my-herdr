# 只聊天任务：对话直到你说结束

**有些任务是对话，不是工作**：咨询、讨论、和代理一起梳理思路。只聊天（chat only）任务永不自行结束、永不评审、永不合并。它一直是一场进行中的对话，直到你说完成。

```
omh add "Talk the pricing model through" --chat
```

## 代理收到的交代

- 这是和你的对话，不是要交付的工作。
- 不改仓库：不新增、不编辑、不删除文件，不提交（[和只研究一样](research.md)）。
- 聊天、阅读、查资料、回答。有帮助时可以把笔记或报告写进看板报告区里该任务自己的文件夹（`~/.omh/reports/<repo>/<number>/`）。但从不强制。
- 对话引出的后续工作，它在回答里用文字建议。不为此加任务。
- 它自己的每次停下都在等你。它只在需要权限或登录时才用 report 命令，绝不用来说自己在等。

## 每个回合结束都等你

代理停下时，任务等你的下一条消息（[等待你](waiting-for-you.md)）。行上显示代理写的最后一行，带问号，像向你提问的任务一样。

- **停下之后什么都不跟**：不检查、不评审（无论 `reviewer.*` 和 `review.*` 怎么说）、不推送、不开 PR、不合并、没有“完成”。
- 从看板回复，或在代理自己的终端里输入：它的下一回合开始，回合结束时再次等待。
- 看板催收尾那套不适用：绝不叫它收尾、不退回、不评审。
- 它的限额（`limit.time`、`limit.tokens`）照常生效（[limits](limits.md)）。
- 它保存的报告，在任务等待期间链接挂在任务面板里。

## 你说结束才结束

- 看板和网页看板上：**批准**（`Ctrl+L`，`end the chat, as it is`），在任务等你回答时。
- 命令行：`omh finish <id>`。

任务随即 **完成**，代理最后一条消息作为小结；写了报告的则是 `Chat ended. Report: <link>`。不检查、不评审、不推送、不合并，它的空间像完成的任务一样清理。

如果代理还是改了仓库，那些改动同样不会合并，那一行会说明：`Chat only, but its agent changed the repo (1 change in the repo): nothing was pushed, opened or merged.`改动会保留（清理会保留从未合并的工作）。

停止（`Ctrl+X`）和归档与任何任务一样。批准只聊天任务会被拒绝：没有东西可合并。

## 让任务只聊天

它和其它一样是[设置](settings.md)：`chat.only`，`true` 或 `false`，默认 `false`。查找方式与所有设置相同，就近优先：**任务 → 你在该仓库的条目 → 你 → 仓库自己的文件 → 默认。**

| 用途 | 方法 |
|---|---|
| 添加某个任务时 | `omh add "<title>" --chat`（`--chat.only true` 的简写） |
| 稍后对某个任务 | `omh delegate <id> --chat`（`--no-chat` 关掉）、`omh set chat.only true --task <id>`，或任务设置页上的那一行（`Chat only`）：终端看板上；网页看板在打开的任务下（**All settings**） |
| 整个仓库或你的全部任务 | `omh set chat.only true --repo <name>`，或不带 `--repo`。仓库自己的 `.omh.json` 也能写：`{ "chat": { "only": true } }` |
| [自动化](automations.md) | 创建或修改时加 `--chat`（`--no-chat` 关掉），或 `set chat.only true --automation <name>`。它生成的每个任务都带上自己的这份设置 |

- **协调者会自动这样做**：当你明确要的就是讨论或咨询、不构建任何东西（「我想和代理一起把路线图想清楚」）。要工作时不会，想把研究交付成报告也不会：那是[只研究](research.md)。
- 它在任务的代理启动时读取，和只研究一样：对进行中的任务改动，从下次启动生效，`show <id>` 会在取值旁说明（`counts from its next start: this run is not chat only`）。

## 只聊天与只研究同时开启

**聊天优先。**两者都开的任务是只聊天：一场永不自行结束的研究。代理按聊天的规则来，回合结束等你，行上显示 `[chat]` 而不是 `[research]`。

## 在看板上

- 任务名后显示 `[chat]`：看板、Herdr 侧栏、网页看板，另一台机器上的任务也一样。标签来自设置，绝不来自标题。
- 任务面板有 **Chat** 部分：`No change to the repo · ends when you say.`，有报告时带报告链接。它的设置在普通任务显示合并方式的位置显示 `Chat  no change to the repo · ends when you say`，并省去从不运行的评审和检查。
