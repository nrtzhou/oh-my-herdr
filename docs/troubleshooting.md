# 排查问题

**多数问题会在任务行上写明是什么、下一步做什么。**选中任务按 `Alt+P`（全屏 `Alt+T`）：任务面板里有完整信息。本页列出常见问题和继续排查的去处。

## 快速检查

| 命令 | 作用 |
|---|---|
| `omh doctor` | 健康检查：一次跑完下面所有项，每项附处理方法（[健康检查](#健康检查)） |
| `omh install` | 重写 `omh` 启动器，并提示 PATH 里是否有同名程序排在前面（[安装](install.md)） |
| `omh models [program]` | 各代理程序在此可用的模型名，或为何读不到它的列表（[模型](models.md)） |
| `omh machines` | 各机器是否响应、各自剩多少磁盘空间、不响应的原因（[其他机器](other-machines.md)） |
| `omh sweep` | 立即清除已结束任务留下的东西，并说明清了什么（[磁盘空间](disk-space.md)） |
| `omh show <id>` | 一个任务运行用到的每个设置及取值来源（[设置](settings.md)） |
| `omh details <id>` | 一个任务的全部信息：评审、交接、限制、在等什么 |
| `herdr workspace list` | Herdr 本身是否在运行并响应 |

## 健康检查

`omh doctor` 一次检查下面所有项，约花一秒。带 `✓` 的行没问题。带 `✗` 的行是问题，下面给出处理方法。有问题时退出码为 1，脚本也能调用。

| 检查项 | 报告的问题 | 怎么办 |
|---|---|---|
| 本程序 | 版本与安装方式（下载、源码构建）；比插件自带的版本旧 | `omh update`，或重新构建 |
| 启动器 | `~/.omh/data/bin/omh`（代理和钩子运行它）缺失、指向已消失的程序、或不是插件的 | `sh <plugin folder>/bin/run.sh install`，或从 Herdr 打开一次看板 |
| 你的 PATH | `omh` 不在上面，或同名程序排在前面 | 把启动器目录放最前（[安装](install.md#path-里的命令)） |
| Herdr 和插件 | 找不到 Herdr、比插件要求的旧、插件没装、被关掉、或链接到已消失的目录 | 安装或更新 Herdr；`herdr plugin enable omh`；重新链接或安装插件 |
| 你的设置里点名的每个代理程序 | 没装、比看板验证过的版本旧、没登录（Claude Code、Codex 和 Cursor 能报告）、缺 Herdr 集成 | 装上或换一个；`claude auth login`、`codex login`、`cursor-agent login`；`herdr integration install <name>` |
| 模型和努力度设置 | 程序不认的模型、它没有的努力度、干不了该角色的程序、手写但不算数的值 | `omh set <setting> …`（[模型](models.md)） |
| 计划审批 | 计划的写手无法在这里的只读挂载里运行（Linux 缺 bubblewrap 或不允许用户命名空间；macOS 上 `sandbox-exec` 不可用），而你的设置要求计划（`plan.approve`）。没设置要求时只作事实陈述 | 安装 bubblewrap；macOS 上在沙箱外启动 Herdr；或 `omh set plan.approve none`（[先计划](plan.md)） |
| 其他机器 | 有机器不响应、跑着旧版本、或无法向本机索取 | 看 `omh machines` 说什么（[其他机器](other-machines.md)） |
| 磁盘空间 | 放看板数据、仓库、工作树、构建缓存或临时文件的盘剩不到 10 GB：任务的环境准备可能失败 | 腾出空间，先清下面的遗留物 |
| 遗留物 | 已完成或已归档任务的工作树（清理会保留有未保存工作的）、挂在没有任务的分支上的工作树、git 还记着但目录已删的工作树、`~/.cache/omh` 里的旧热构建 | 它给出的 `git worktree remove`、`git worktree prune` 或 `rm -rf`。它自己绝不删东西；`omh sweep` 只删可证明属于已结束任务的（[磁盘空间](disk-space.md)） |
| 协调者的队列 | 有消息一分钟以上没送达，比如 `coordinator tell` 运行时找不到 herdr | 在 Herdr 里打开看板即送达；在那个 shell 里把 `HERDR_BIN_PATH` 设为 herdr 的完整路径（[协调者](coordinator.md)） |
| 旧安装 | herdr-superwork 或 omh 0.2 还挡着路 | 能接管的都接管，并说明剩下的（[旧安装](old-installs.md)） |

只有存在其他机器时才去问，每台最多问 5 秒。已完成任务的工作树，若其清理还没到、或 `workspace.cleanup` 为 `false`，不算遗留物。

## 代理起不来

行上显示 `Could not start: …` 加原因，比如 `codex is not logged in` 或 `sol-6.1 can't be used with your ChatGPT sign-in`。照它说的修好，再派发（delegate）一次。

原因和修法的完整表格：[代理起不来时](could-not-start.md)。

**第一次用某个程序？**自己在仓库目录的终端里跑一次：登录、完成首次设置，它问的话信任该目录（[代理程序](agents.md#程序首次接任务前)）。

## 磁盘空间不足

| 你看到 | 是什么 | 怎么办 |
|---|---|---|
| 任务的环境准备或检查报 `No space left on device` | 构建期间磁盘满了 | `omh sweep`，再用 `omh doctor` 看还有什么可清；然后重新完成或派发它 |
| `Disk nearly full on vm-str: … free on …; this task starts once 5.0 GB is free` | 看板阻止了任务在快满的磁盘上启动（`board.diskFree`） | 同上；空间够了它自己会启动 |

已结束的任务自己清理，看板每天清扫一次它们留下的东西：[磁盘空间](disk-space.md)。

## 模型名被拒

**名字写错时，设置它的一刻就会被拒**，并给出程序认的名字和最接近的一个：

```
$ omh set worker.agent codex/sol-6.1
worker.agent: codex has no model "sol-6.1" for your ChatGPT sign-in (pro plan). Did you mean gpt-6.1-sol? It takes
gpt-6.1-sol, gpt-6-astra, gpt-6-sol, … (codex's own list for that sign-in). Write it after the program: codex/gpt-6.1-sol.
```

- **看程序认什么：**`omh models codex`。对 Codex 就是你的登录能用的：ChatGPT 登录和 API key 认的名字不同。
- **“Its list cannot be read”（读不到它的列表）：**程序装了，但列模型的命令失败（没登录、离线）。在终端跑那条列模型的命令看原因，或者干脆不写模型。
- **新的 Claude 模型被拒：**看板的 Claude 名字表比模型旧。用短名（`claude/opus`）：它永远指同类里最新的。更新看板（`omh update`）会带来新名字。
- **努力度被拒：**每个程序有自己的级别。拒绝信息里会列出它有的。

### 代理工作中遇到 “can't be used with your ChatGPT sign-in”

Codex 在代理第一个回合就拒绝了模型（`The 'sol-6.1' model is not supported when using Codex with a ChatGPT account`）。看板会接住它：

- **`…: started again on codex's own default model`**：看板给 Codex 指定了那个模型；Codex 现在用它自己的默认模型干活。不用管。要换一个，`omh models codex` 列出你的登录能用的。
- **`Agent stopped: … can't be used with your ChatGPT sign-in; try gpt-6.1-sol`**：它的默认模型也被拒，或看板没给它模型（名字来自 Codex 自己 `~/.codex/config.toml` 里的 `model`）。给任务选一个，`omh set worker.agent codex/gpt-6.1-sol --task <id>`，或改掉那行，然后 `Ctrl+L`（重试）：代理在它上面重新启动。

更多：[模型与努力度](models.md)。

## 任务像卡住了

看它那行说什么：

| 行上显示 | 发生了什么 | 怎么办 |
|---|---|---|
| `Waiting for a free slot`（`Q#2`） | `board.agents` 的名额全满 | 等待，或调高 `board.agents` |
| `queued after HSW-12` | 在等其他任务完成（[一个接一个](after.md)） | 等待，或 `omh delegate <id> --now` |
| `Has not started yet: pressing enter again` | 任务文本还在代理的输入框里 | 稍等：看板会再按一次回车 |
| `Not started: its instructions sit unsent in its box` | 文本没发进去 | `Ctrl+L`（重试），或进代理里按回车（[交接](handing-over.md)） |
| `Finishing` 持续很久，代理已空闲 | 看板错过了回合（turn）结束 | `Ctrl+L`，或 `omh finish <id>` |
| `Stopped without a summary` | 代理停了，看板无法判断它是否完成 | `Ctrl+R` 让它继续；`Ctrl+L` 按现状收下工作 |
| `Agent's connection dropped, trying again in 2m` | 服务商切断了回合 | 不用管：看板自动重试（[被服务商切断的回合](provider-errors.md)） |
| `Agent stopped` | 等待解决不了的错误：用量限制、拒绝 | 在面板里读原因；回复，或 `Ctrl+R` |
| `Agent stopped: … can't be used with your ChatGPT sign-in` | 它的程序拒绝了那个模型 | 选一个模型，然后 `Ctrl+L`（[上文](#代理工作中遇到-cant-be-used-with-your-chatgpt-sign-in)） |
| `Plan ready` · `The plan's review could not run (…): yours to judge` | 计划的评审者（`plan.approve reviewer`）起不来或没给结论 | 读计划；`Ctrl+L` 开始构建，回复把它退回，`omh retry <id>` 让评审者再看（[先计划](plan.md#评审者的审阅)） |
| `Needs a login` | 代理的程序登出了 | 登录（`Enter` 进代理），然后回复或 `Ctrl+R` |
| `stopped at its 2h limit` | 到了 `limit.time` 或 `limit.tokens` | `Ctrl+R` 继续、调高限制，或 `Ctrl+L` 按现状收下（[限制](limits.md)） |
| `resolving a conflict with main` | main 在它脚下变了；它的代理正在解决 | 等待 |
| `behind main` | 拉不进 main；工作本身没问题 | 看板每分钟重试；原因在面板（[下文](#落后于-main拉不下-main)） |
| `Waiting for GitHub checks · 2 of 3 passed` | PR 的检查还在跑 | 等：通过后自动合并 |
| `Could not read GitHub's checks` | `gh` 没登录，或它的 token 没有 Actions 和 Commit statuses 的读权限 | 登录 `gh`，或给 token 这两项读权限（[工作如何完成](how-work-gets-done.md#细粒度-github-token-就够了)） |
| `The check stands still · nothing finished for 13m` | 仓库的检查卡住了 | 看看检查在跑什么（[检查的进度](check-progress.md)） |

**代理弹出对话框**（权限提示、选项问题）：聊天里会说明。按 `Enter` 进代理，在里面回答。

**代理用大白话提问然后停下**：看板当作已完成并进入评审（review）。回复代理即可继续（[等你的代理](waiting-for-you.md)）。

## 完成的任务留在 Herdr 侧栏

合并或结束的任务，其空间（space）会自动关闭（`workspace.cleanup`，默认开）。没关掉时，任务面板里它的行以 `Kept:` 结尾并说明原因：

| 显示 | 原因 | 怎么办 |
|---|---|---|
| `(uncommitted changes)` | 它的检出里有从未提交的文件 | 看看那些文件；回复任务，或自己删掉工作树 |
| `(unmerged work on <branch>)` | 它的分支上有没合并的提交 | 合并它们，或回复任务继续处理 |
| `(its space is in use: a turn in its agent's terminal)` | 你在它自己的终端里跟它的代理说过话 | 不用管：约每半分钟重试一次，代理安静一会儿就关 |
| `(its space is in use beside its agent: <process> (pid <n>) <why>)` | 代理的面板里跑着一个不是它助手的进程；`<why>` 说明哪里不对（见下） | 是你的就结束它，下次重试时空间关闭 |
| `(its space is in use in another of its panes: <process> (pid <n>) runs in it)` | 它空间的另一个面板里有命令在跑 | 结束它，或关掉那个面板 |
| `(its space is in use: herdr cannot tell whether its agent works, …)` | Herdr 判定代理状态未知，行的其余部分说明还有什么没定（屏幕还在动、上一回合没结束） | 不用管：代理静止一会儿就关 |
| `(its space is in use: herdr does not answer for its agent)`（或 `for its space`） | Herdr 没有响应 | 不用管：会重试 |
| `(its herdr session cannot be reached)` | 它所在的 Herdr 会话没在运行 | 启动那个会话；会重试 |

- **`<why>` 的含义**：代理自己进程组里的进程，只有输入输出接在代理上、错误不进任何终端时，才算它的助手（MCP 服务器）。否则它是 `streams to the terminal`（输出到终端）、`is not wired to its agent`（没接上代理：指向文件、`/dev/null`、代理组里没人持有的管道）、`has its input or output closed`（输入或输出已关闭），或 `cannot be looked at`（系统不给看）。代理之下自成进程组的进程（它启动的开发服务器）是 `runs in a group of its own`（接在代理上、或在某个助手之下的进程组算它自己的，不留东西）；由面板 shell 启动的进程是 `runs in the background of its shell`（在该 shell 后台运行）。
- **`; and 2 more`**：该行只列第一项。`omh details <id>` 在 `inUse` 下列出全部。
- **列出了代理自己的助手**（MCP 服务器、语言服务器）：这是 bug。请开 issue，附上 `Kept:` 行和 `omh details <id>` 的 `inUse` 列表。
- **没有 `Kept:` 字样但空间还在**：`omh show <id>` 能查 `workspace.cleanup` 是不是被关了、在哪关的。`omh set workspace.cleanup true` 重新打开。
- **带 MCP 服务器的代理曾永远 `in use`**：服务器被算成还在跑的命令。已修复，下一轮会自己关掉这些空间。
- **想立刻关掉一个**：归档任务（`Ctrl+A`）；磁盘上保留的东西不动，任务会说明在哪。

## 落后于 main：拉不下 main

任务完成后的评审前、合并前，看板都会拉取 main 并合进任务分支。拉取失败不算失败：任务照常检查和评审，然后以 `behind main` 状态等待。追上之前不能批准，看板每分钟自动重试。远端完全连不上、分支也推不上去时，任务在推送和评审前同样这样等，远端恢复后自动执行。你已批准的合并也这样等着，什么都没合并：等行回到待批准后再批一次。

面板（`Alt+P`）在 **Behind main** 下用大白话说明原因，**Details** 下是 git 自己的话：

| 原因 | 怎么办 |
|---|---|
| origin 要求登录 | 让检出里的 `git fetch origin` 不要提示就能跑：代理里加载 SSH key，或用凭据助手。看板绝不输密码 |
| origin 60 秒内没响应 | 它在等密码或口令的话同上；否则是远端慢或忙，下次可能就过了 |
| 连不上 origin | 网络或服务器的问题：不用管，看板会重试 |
| origin 没有分支 … | 远端主分支叫别的名字，或任务起始分支（`workspace.from`）没了：在检出里 `git fetch origin`，再 `git remote set-head origin --auto`，或重新推那个分支 |
| git 的锁文件 … 挡路 | 一直有另一个 git 命令在写同一个 ref，或停掉的命令留下了 `.lock` 文件：那个仓库里没有 git 命令在跑的话，删掉它点名的文件 |
| origin 不是看板能读的仓库 | `git remote -v` 看 origin 指向哪里 |

同一个仓库的多个任务同时完成从不互相挡路：它们共用一份 main（仓库的每个工作树写同一个 `origin/main`），所以看板一次为一个任务拉取，等别的任务拉取的那个直接用它的结果。git 因为恰好有别的程序在拉取而拒绝的 fetch（某个代理自己的 `git fetch`、你的、编辑器的），2 秒和 4 秒后重试，那个程序刚拉到的副本也算数。远端主分支不叫 `main` 且你的克隆里从没指过名（没有 `origin/HEAD`，比如 `git remote add` 之后）时，会问远端它是什么分支，然后拉那个分支，只拉一次。

## 日志在哪

全在看板的数据目录 `~/.omh/data/`（[主目录](home-folder.md)）：

| 文件 | 内容 |
|---|---|
| `tasks/<repo>/<n>/runner.log` | 一个任务的后台工作：派发、收尾流水线（finish line）、合并。启动失败会在末尾追加说明：原因、Herdr 说了什么、代理终端的最后几行 |
| `coordinator/<session>/start.log` | 协调者起不来的原因，附其终端的最后几行 |
| `keeper.log` | 看守（看板的后台进程）：启动、更新、报错 |
| `hook.log` | Herdr 的钩子和代理回合结束报告告诉看板的内容 |
| `jobs.log` | 不属于任何单个任务的后台作业 |
| `tasks/<repo>/<n>/closing-note.md` | 代理停下时说的话（收尾小结） |

任务面板已显示大部分。`omh details <id>` 打印其余。

## 看板本身

- **找不到 `omh`**：从 Herdr 打开一次看板（插件动作 "Open the work board"），然后把 `~/.omh/data/bin` 加进 PATH（[安装](install.md#path-里的命令)）。
- **装了却找不到 `herdr`**：纯 SSH 跑的命令（`ssh host omh …`）拿不到登录 shell，PATH 里常常缺 `~/.local/bin`。PATH 里没有 `herdr` 时，看板依次找 `~/.local/bin`、`~/.cargo/bin`、`~/bin`、`/usr/local/bin`、`/opt/homebrew/bin`，再看登录 shell 的 PATH。Herdr 在别处：把 `HERDR_BIN_PATH` 设为它的完整路径。
- **看板按键没反应**：可能有别的插件浮层开着（先关掉），或按键没绑定。看板首次运行的界面会提供按键绑定（[安装](install.md#首次运行)）。
- **看板显示被裁切或一半空白**：同一会话的另一个 Herdr 窗口（第二台设备、浏览器标签页）占着尺寸。在本窗口按任意键（[看板的尺寸](board-size.md)）。
- **Herdr 拒绝插件**：正在运行的 Herdr 服务器必须是 0.9.3 或更新。更新 Herdr 后重启它。
- **别的机器的任务不显示**：运行 `omh machines`，再看[机器不响应时](other-machines.md#机器不响应时)。
