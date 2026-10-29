# 代理起不来时

**代理起不来的任务会在行上用大白话写明原因和怎么办。**照它修好，然后重新派发（delegate）（重启的代理则是回复它）。

| 行上显示 | 怎么办 |
|---|---|
| `Could not start: codex is not installed here` | 装好它，让 `codex` 能在这里的终端运行，或换一个程序（`worker.agent`）。 |
| `Could not start: codex is not logged in` | 在这里的终端运行 `codex` 并登录。 |
| `Could not start: codex does not know the model it was given`、`Could not start: sol-6.1 can't be used with your ChatGPT sign-in; try gpt-6.1-sol` | 选一个它有的模型（`omh models codex` 列出你的登录能用的；[模型](models.md)），或不指定模型。 |
| `Could not start: codex 0.150.0 does not take one of the board's start arguments` | 更新程序。 |
| `Could not start: codex quit as it started` | 在任务面板看它打印了什么，照着修。 |
| `Could not start: codex quit as it started`，且它打印了 `cannot unmarshal ~/.codex/config.toml` | 它的 config.toml 读不了：`omh doctor` 会指出哪一行、或哪个项目的表在文件里出现了两次（旧版看板可能加过）。删掉第二份；看板从不重写该文件。 |
| `Could not start: codex did not come up within a minute` | 看它的终端：它显示什么（一次更新、一个问题、一个报错），就是什么挡着。 |
| `Could not start: its start command was cut off: the terminal's shell was still starting` | 重新派发：看板已清掉输了一半的命令。 |
| `Could not start: its terminal's shell was not ready for the start command` | 等任务的终端出现提示符，再重新派发。看板已经等过并试了 4 次；shell 启动慢（它的 `.zshrc`）会造成这样。 |
| `Could not start: its terminal was not at its shell prompt` | 结束任务终端里在跑的东西，或关掉那个终端。看板说已等过并试了 4 次时，也可能是 shell 极慢：等它出提示符。 |
| `Could not start: herdr did not answer in time` | 检查 Herdr 在运行并响应（`herdr workspace list`）。 |
| `Could not start: its start was cut off, twice` | 重新派发。有东西从外部掐掉了启动，最常见是随自身退出而结束所有子进程的脚本或服务（systemd oneshot）。只要机器上跑着 herdr，它的看守也在；脚本里的 `omh delegate` 会把启动交给看守续命。被掐一次的启动会自动重试。 |

## 还去哪看

- **任务面板**有完整说明：原因、Herdr 说了什么、代理终端的最后几行、继续排查的去处。
- **终端还开着**：在任务上按 `Alt+O` 进去。
- **任务自己的日志**末尾有同样的说明：看板上任务目录里的 `runner.log`（`~/.omh/data/tasks/<repo>/<number>/`）。
- **评审者**起不来时，会在它的评审里写同样的说明。
- **协调者**会在输入框上方说明，并把终端的最后几行存进自己目录的 `start.log`（它的标签页会关掉）。

更多问题和日志位置：[排查问题](troubleshooting.md)。

## 超长规则与超长消息

曾有两个限制让长文本起不动代理，且不给原因。现在都已处理。

- **超长规则写进文件。**Herdr 通过往面板的 shell 里敲命令来启动程序。那个 shell 还在启动时（行编辑器没就绪：`.bashrc` 或 `.zshrc` 慢），终端一行最多收 4096 字节，macOS 上只有 1024。Codex 过去把任务的规则内联在 `-c developer_instructions=…` 里。配了你自己的 `worker.instructions` 就会超限：命令被截断，shell 停在 `>` 续行提示符上，启动超时。现在超过 300 字节的规则写进任务目录里的文件（`rules.md`），并让 Codex 读它、整个会话（session）遵循它。Claude Code、pi、Droid 等一直以文件名接收规则，启动命令保持短小：只有 Codex 只收文本。
- **等还没出提示符的 shell。**Herdr 0.9.5 在 shell 提示符出现之前拒绝超过终端一行限额的行（`the terminal is in canonical mode with a 1024-byte limit; wait for the shell prompt and retry`）：macOS 上新任务的首次启动就这样失败过。现在看板会等面板出现提示符再重试，每次启动（任务的代理、评审者、协调者、并行尝试的选择器）最多再试 3 次、约 15 秒内。它先看面板里有没有代理，所以绝不会启出第二个。
- **herdr 还看不见的 shell 也会等。**Herdr 0.9.5 还会拒绝在刚创建、shell 仍在启动的面板里启动（`.zshrc` 花一秒）：`agent target pane … is not an available shell`（`agent_pane_busy`）。更新后协调者的启动就这样失败过。现在看板同样等了再试。真有程序在跑的面板，试完仍报 `its terminal was not at its shell prompt`；已有代理在跑的面板立即失败。
- **超长消息写进文件。**消息是 `herdr agent prompt` 的一个参数，Linux 一个参数最多传 128 KiB。超过 100,000 字节的消息（任务的首个提示词（prompt）、回复、给协调者的消息）写进看板目录里的文件（`messages/`），改为输入指明该文件的一行。更短的原样输入：4,000 和 100,000 字符都完整进了 Claude Code 和 Codex。

这些限制怎么发现的：[在 Herdr 上验证](checked-on-herdr.md)。
