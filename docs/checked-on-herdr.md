# 在 Herdr 0.9.3 上验证了什么

**使用看板不需要本页。** 本页是它背后的实验笔记：每一行是一项关于 Herdr
（或在 Herdr 中运行的代理程序）的事实，看板依赖它，以及在 Herdr 0.9.3 上实测的结果。其他
页面把证明链接到这里。可按第一列的词查找某条事实。

最后一节 [Herdr 缺少什么](#herdr-缺少什么)，列出看板的弹窗要做到像
Herdr 自己的菜单那样，还缺什么。

## 事实

| 事实 | 结果 |
|---|---|
| 插件事件钩子会在 `pane.agent_status_changed` 上触发 | 是；该事件带面板 id、空间 id、状态、代理 |
| 弹窗面板；弹窗里的 `HERDR_PLUGIN_CONTEXT_JSON` | 是；没有面板 id，但有空间、聚焦面板及其目录、工作树的仓库根目录 |
| 在弹窗里执行 `agent focus`，弹窗关闭后仍生效 | 是 |
| 另一个标签页显示时的弹窗面板 | 它属于打开它时所在的标签页（`HERDR_PLUGIN_CONTEXT_JSON` 里的 `tab_id`）：herdr 在其他标签页或空间获得焦点时隐藏它，它的程序继续运行，该标签页重新显示时它再次显示。它覆盖该标签页的所有面板，无论是否放大。只要它存在，无论显示还是隐藏，任何插件的 `plugin pane open` 打开弹窗都会得到 `ui_busy: a popup pane is already open`。（看板的弹窗自 2026-10-13 起就是这一种，另一个标签页显示时会自行关闭：见下面关于 herdr 弹窗的各行） |
| 用 `--placement tab` 打开的插件面板 | 聚焦空间里一个新标签页，含一个普通面板：其程序收到 `HERDR_PANE_ID`、`HERDR_TAB_ID` 和 `HERDR_PLUGIN_ID`，`HERDR_PLUGIN_CONTEXT_JSON` 仍指向你来自的那个面板。应答带 `plugin_pane.pane`（面板、标签页、空间）。你离开后它仍在，其他插件的弹窗在它旁边打开，`tab rename` 能给它改名，程序结束时它的标签页关闭。从另一个空间执行 `plugin pane focus <pane>` 没有把客户端移过去；先 `workspace focus` 再 `tab focus` 可以 |
| 用 `--placement overlay` 打开的插件面板（2026-10-13 在 tmux 内的临时会话中试过） | 一个带 id 的普通面板，从当前显示且放大的标签页分割出来，画在带面板标题的边框里：它占标签页整个大小减去边框（134×44 给看板 132×42），`--width` 和 `--height` 被拒绝（`width and height are only supported when placement is popup`），其程序收到 `HERDR_PANE_ID`、`HERDR_TAB_ID`、`HERDR_PLUGIN_ID`、`--env` 指定的变量，以及带来源面板（`focused_pane_id`）及其标签页的 `HERDR_PLUGIN_CONTEXT_JSON`。herdr 自己的按键在它里面仍有效：overlay 聚焦时看板键执行了它的动作（在浮动弹窗里 herdr 把 `ctrl+b` 和 `a` 当作输入的按键交给看板，什么也没执行）。`plugin pane close <id>` 关闭它并像关闭终端那样结束其程序：看板保存了草稿。它出现在 `pane list` 里，`pane layout` 把它报为放大且聚焦的面板。它曾在另一个插件的浮动弹窗打开时打开、藏在另一个标签页后面时也显示（它开在那下面），那个弹窗也曾在它上面打开：两种情况都没有 `ui_busy`。留在某个标签页上时，它随该标签页隐藏并继续运行。herdr 的放大键（`prefix+z`）把它缩小为分割面板；打开它之前放大的面板在它关闭后不再放大。看板键在 2026-10-13 之前为 `board.open` `popup` 打开的是这一种，现在不再如此 |
| 看板键在 `board.open` 设为 `popup`（当时指 overlay）时的实测（同一会话同一天；把第二个插件的浮动弹窗当作"另一个插件的"）。保留它是因为它说明了 overlay 的行为 | 在聚焦的标签页上打开；再按同一键关闭它；按两次 `Esc`（先是一份草稿，然后什么也不剩）关闭它；关闭前输入的词下次打开时还在输入框里。留在标签页 1 而在标签页 2 按键，以及留在一个空间而在另一个空间按键：在那里关闭，在这里打开。在任务所连的面板上按键：弹窗在该任务上打开。对另一个空间的任务、以及弹窗所盖的标签页上的任务按 `Enter`：弹窗关闭，herdr 到了该任务的面板上，并未放大（该面板停在 shell，所以 herdr 去了它的空间和标签页；从弹窗对运行中的代理执行 `agent focus` 前面三行已验证过）。标签页里有看板在跑时：两个都运行，各有自己的提示和草稿，一个看守。弹窗里：输入任务并按 `Enter`、删除确认按 `n`、`?` 菜单再 `Esc` 退出、滚轮划过列表（输入到客户端的 SGR 鼠标码会移动选中项）、其他机器的任务（`Alt+M`，一台已保存的机器），以及 56×18 和 56×8 下的布局。另一个插件的浮动弹窗显示时，看板键从未到达插件（herdr 把它输入到那个弹窗里）；随后通过 `herdr plugin action invoke` 调用动作在它下面打开看板。未试：从弹窗发给运行中的协调者（临时会话里它是关的：那些词成了一个任务），以及 herdr 对 overlay 回应 `ui_busy`（它从未这样做过；用假 herdr 的测试覆盖了那时看板说什么） |
| herdr 自己的菜单怎么画，插件在这方面得到什么（读自 herdr 在 v0.9.3 和 2026-10-13 的 main 分支 `241b99cd` 的源码；其文档 `herdr.dev/llms.txt` 和 2026-10-12 的预览版说法相同） | herdr 的菜单、设置、导航器、帮助和其他对话框由客户端自己画在整个窗口上（`src/client/shell/state.rs` 的 `ClientShellOverlay`：`GlobalMenu`、`Settings`、`Navigator`……）：一个带边框的方框，背后不压暗，每个按键和点击都归它。没有插件 API 能画在那里：插件面板有五种放置方式，`overlay`、`split`、`tab`、`zoomed`（普通面板）和 `popup`，此外没有客户端级或会话级的东西。（0.9.3 的 `herdr plugin pane open --help` 的列表漏了 `popup`；命令本身接受它。）弹窗最接近：整个会话一个（服务端状态里的 `popup_pane`），画在面板之上，但服务端记着它打开时所在的标签页（`src/server/headless.rs` 的 `popup_owner_tab_id`），只在该标签页显示时显示它并把输入给它。main 分支上没有变化 |
| herdr 的弹窗，为看板试过（2026-10-13，tmux 内的临时会话，140×44，插件链接进去） | `plugin pane open --placement popup --width 90% --height 90%` 画出标题为 `Work board` 的带框窗口，居中于窗口中面板所占的部分（不盖侧栏和标签栏），在 114×43 的面板区域里外框 102×38（看板是 100×36）；不给尺寸则各占一半。其程序收到 `HERDR_PLUGIN_ID`、`HERDR_PLUGIN_CONTEXT_JSON`（含 `tab_id` 和 `focused_pane_id`）、`--env` 指定的变量，没有 `HERDR_PANE_ID`。它下面的布局不受影响：打开前放大的面板关闭后仍然放大。所有按键都给它的程序：`ctrl+b` 然后 `a`（看板键）作为那两个按键到达，没有任何动作执行。点它外面没有反应：点标签栏里另一个标签页什么也不切换。socket 的 `popup.close`（命令行没有对应命令）关闭它并像关闭终端那样结束程序：看板保存了草稿；没有弹窗打开时它回应 `popup_not_open`。从命令执行 `tab focus` 和 `workspace focus` 仍会在它下面移动 herdr：弹窗随即隐藏、继续运行，并随其标签页再次显示。带 `tab.focused` 的 `events.subscribe` 对其中每次移动（包括移到另一个空间）都发出带该 `tab_id` 的 `tab_focused` |
| 看板键在 `board.open` 设为 `popup` 时的实测（同一会话同一天） | 弹窗在中间打开；再按该键（由看板自己读取）关闭它；按两次 `Esc`（先是一份草稿，然后什么也不剩）关闭它；关闭前输入的词下次打开时还在输入框里，从外部关闭（弹窗显示时通过 `herdr plugin action invoke` 调用动作）后也一样。显示时 `tab focus` 到另一个标签页：弹窗连同它的提示立刻消失，按键在新标签页上带着同样的词打开它。面板放大，弹窗开又关：仍然放大。对面板在另一个空间的任务按 `Enter`：弹窗关闭，herdr 到了那个面板上；在那里按键在该任务上打开弹窗。`?` 菜单、删除确认按 `n`、滚轮划过列表（输入到客户端的 SGR 鼠标码会移动选中项）、`Alt+M`（没有已保存的机器：看板说了）。在 60×20 的窗口（herdr 的窄布局，面板 60×18）里弹窗是 54×16，看板用其窄布局占 52×14；30×8（面板 30×6）里是 27×5，看板在 25×3 里显示其文本框和一行提示。另一个插件的弹窗显示时，看板键被输入到那个弹窗里（`^Ba`）；随后从命令调用动作得到 `ui_busy`，看板改在标签页里打开并说明原因。看板弹窗显示时，另一个插件的 `plugin pane open` 得到 `ui_busy`。未试：从弹窗发给运行中的协调者（临时会话里它是关的）、对聚焦面板上等待批准的任务执行批准动作（测试覆盖了看板问哪个）、同一会话的第二个窗口，以及用 kitty 协议发送按键的终端 |
| 看板弹窗还在启动时标签页或空间变了（2026-10-13，同类会话） | herdr 只从订阅建立那一刻起上报 `tab_focused`，而看板的程序需要约一秒才能就绪：那一秒里的 `tab focus` 或 `workspace focus` 会漏掉，弹窗留在其标签页上隐藏着，仍在运行。`tab.get` 对标签页回答 `focused`，只对 herdr 显示的那一个为真（对未显示空间的前台标签页为假）。所以看板在订阅开始（`subscription_started`）后问一次，并在其标签页不显示时关闭。试过按键后 0.15、0.3、0.6、0.9 和 1.3 秒移到另一个标签页和另一个空间：没有看板被留在运行中，回到第一个标签页也没有弹窗。在 herdr 执行按键动作之前发生的移动，会让弹窗开到那时显示的标签页上，并留在那里 |
| 看板键在 `board.open` 保持 `tab` 时的实测（同一会话） | 在 shell 面板上按键：看板的标签页打开，其提示点名那个面板；在看板上按键：herdr 回到那个面板；再按键：回到看板。无处可回时，看板的消息行在一秒内说明 |
| 按 id 把键盘交给某个面板 | 没有命令能做到（`pane focus` 只移到相邻面板，`workspace focus` 和 `tab focus` 让焦点留在标签页里原来那个面板上）。`pane zoom <id> --on` 聚焦该面板，即使标签页里另一个面板已放大也如此，之后 `--off` 把焦点留在那里：看板键用它回到与代理共用标签页的看板 |
| 面板终端上的第二个查看者（`herdr terminal attach`） | 它设定终端尺寸；herdr 随后从左上角显示终端的一个截取部分。`pane layout` 仍给出 herdr 显示的尺寸，所以看板在那里面绘制并说明受什么限制；`terminal session control --takeover` 立即退出，会断开另一个查看者并让 herdr 把终端重新调到面板尺寸：看板自己会做（设置 `ownTerminal`）。查看者附着期间没有别的办法拿回尺寸（客户端调整大小、放大、不带 `--takeover` 的 `control`）。弹窗里不适用：它没有面板 id |
| `agent start`；`agent get` 里的 `state_change_seq`、`completion_seq` | 是；`completion_seq` 在第一个回合结束后出现 |
| `agent start` 什么时候说 "ready"（2026-10-12 见到，claude 2.1.296、codex 0.160.0、pi 1.1.0） | 一旦程序成为面板的前台进程，约在命令输入后 3.5 秒，无论屏幕上是什么。一个被限制在处理器 8% 的程序那时还什么都没画：Claude 在启动后 13 到 17 秒画出屏幕，pi 19 秒；Codex 已经画了 |
| `agent prompt` 输入什么（由一个记录其输入的程序看到） | 文本作为一次括号粘贴（bracketed paste），300 ms 后一个单独的回车。`pane run` 把两者在一次写入里发出；`pane send-text` 只发裸文本 |
| 在 Claude 画出屏幕前输入的提示词 | 文本被保留并显示在其输入框里，回车丢失：状态保持 `idle`，`state_change_seq` 不动，只要没人按回车（观察了 60 秒）。`agent send-keys <pane> enter` 可以启动它。屏幕画出后在同一个被限速的程序上输入，同样的提示词靠自己的回车通过；herdr 在 12 秒后显示 `working` |
| 输入到运行中的 Claude（处理器 3%）或粘贴与回车在一次写入里的提示词 | 都能通过：只有仍在启动的程序会丢回车 |
| 在长回合中途输入给 Claude 的消息（2026-10-13 试过，tmux 内的临时会话，haiku 上真实的 claude 2.1.296：30 秒等待，然后 16 条各 29 KB 输出的命令） | Claude 把它排队，在回合的两个步骤之间读取；herdr 的状态和计数不因此变化。它保存的会话记录在入队和取出时各记 `queue-operation` 行，词句本身作为类型 `queued_command` 的 `attachment`，绝不作为 `user` 条目。回合结束前该附件达到 1.04 MB（任务 175 上是 3.5 MB）。当时的看板只读后 768 KB：代理停下来时它按了三次回车并说 "Not started: its instructions sit unsent in its box"，而工作已经完成。现在的看板在消息输入 15 秒后撤掉自己的标记，当时代理仍在工作，任务最终记为完成（[交接](handing-over.md)） |
| `agent get` 里的 `agent_session` | 仅当装了 herdr 对该代理的集成；没有它，重启的代理从头开始 |
| 服务重启后空间和面板 id | 保留；`terminal_id` 不保留；没有集成的代理消失 |
| 代理退回其 shell 时触发哪个事件 | `pane.agent_detected`（released）以及状态变为 `unknown` |
| 从钩子分离派生的进程 | 在钩子退出后存活；150 秒的钩子没有被掐断 |
| 钩子派生进程里的 `PATH` | herdr 的服务端环境，可能缺 `~/.local/bin`：插件改用登录 shell |
| 在没有对应空间打开时执行 `worktree create --cwd <repo>` | 可行；herdr 还会把仓库本身作为空间打开，工作树删除后它仍在（所以插件会关掉它） |
| Claude 和 Codex 在全新工作树中拥有完全访问权限时 | 两者都显示信任对话框，每个仓库一次；预先信任可免去它 |
| Codex 回复之后 | 这里报 `done`；收尾小结也覆盖 `unknown` |
| 有已保存机器时的 `herdr machine list --json`（2026-10-13 见到） | 每条目有 `id`、`label`、`target`（给 `herdr machine add` 的 `ssh://user@host:port`）、`session`（`default`）、`enabled` 和 `selected`。列表存在 `$XDG_STATE_HOME/herdr/client/endpoints.json`，不在配置文件夹里。`herdr machine add` 拒绝以前台 `herdr server` 启动的远程服务（"not ready for saved machines"），接受以正常方式启动的 |
| `herdr agent start` 下的 0.156.0 之前的 Codex（2026-10-15 见到，herdr 0.9.3，npm 的 codex 0.153.4 到 0.156.0） | 0.155.1 及更早没有 `--no-daemon`（0.156.0 是第一个）：codex 打印 `error: unexpected argument '--no-daemon' found` 并退出，面板回到 shell，`agent start` 等满 `--timeout` 后回答 `timeout`，"timed out waiting for agent startup"。不带该 flag 启动时，0.153.4 在面板里起来并显示其更新对话框（`agent_not_ready`，blocked） |
| `herdr agent start --kind <name> -- <the board's flags>` 下的 herdr 其他程序（2026-10-15 见到，herdr 0.9.3，一个私有会话；各程序版本：agy 1.3.6、copilot 1.0.98、cursor-agent 2026.10.01、devin 3000.11.92、droid 0.242.0、grok 1.0.52、hermes 0.19.0、kimi 2.2.0、omp 18.11.1、opencode 1.18.34、qwen 0.25.2） | 每个都约 3.5 秒启动，没有一个因参数退出（[代理](agents.md)）。没有一个已登录或配置好：herdr 把登录页或首次设置页读为 `idle`（agy、cursor、devin、grok、hermes、omp），droid 和 kimi 的目录信任对话框也一样；copilot 是 `blocked` |
| 用 npm 装的 kilo 7.8.12 和 qodercli 1.1.72，同一会话 | 它们的命令是 node 脚本，把真正的程序作为子进程启动，herdr 找不到它：`agent start` 等满超时，程序还在面板里运行。kilo 自己的二进制以 `kilo` 在 PATH 上，3.4 秒启动。Letta Code 0.34.15：herdr 找到了它（`unknown`），它首次运行的后端选择让它拖过了超时 |
| 没有 herdr 集成的 omp 18.11.1 | herdr 找到它并无论它做什么都说 `idle`：herdr 的文档说它的状态需要集成，所以看板不在没有集成时启动它 |
| 通过看板在 droid 0.242.0 上跑的任务（工作代理和评审者都是 droid，同一会话） | 用 `--auto high --append-system-prompt-file <rules>` 启动（其屏幕显示 `Auto (High) · allow all commands`），第一个提示词进去了，herdr 带新 `completion_seq` 的 `done` 结束了回合，看板把它当作无收尾小结的停止：该账户没有 Droid 订阅，没有模型应答 |
| 过长的启动命令或消息（2026-10-15 试过，herdr 0.9.3，一个临时 herdr，Codex 0.162.1 和 Claude Code 2.1.301） | `agent start` 把命令输入面板的 shell：shell 停在提示符时 7 KB 的 `-c developer_instructions=…` 能通过，shell 还在跑启动文件时会被截到 4096 字节（shell 随后停在 `>`，`agent start` 回答 "timed out waiting for agent startup"）；短命令在同一个慢 shell 里能起来。中断字符（`pane send-text <pane> $'\x03'`）清除输到一半的命令。给 Codex 的 `developer_instructions` 只指向一个规则文件时，它在第一次回答前读取并遵守该文件。`agent prompt` 向空闲的 Claude Code 和 Codex 输入 4 KB、22 KB 和 111 KB 的消息，每条都靠自己的回车完整送达；186 KB 根本无法交给 `herdr` 命令（"Argument list too long"：Linux 一个参数最多 128 KiB） |

2026-10-09 针对回合结束钩子和看守做过验证（herdr 0.9.3、Claude Code 2.1.296、Codex 0.160.0、pi 1.1.0），
在一个单独的临时 herdr 里：

| 事实 | 结果 |
|---|---|
| 通过 `herdr agent start` 运行 Claude `--settings <file with Stop and StopFailure hooks> --session-id <uuid>` | 每个回合结束都触发；输入指明会话、会话记录文件和最后一条消息 |
| `claude --settings` 追加到用户自己的钩子 | 是：项目级 Stop 钩子和 flag 的钩子都跑了（用户自己的设置里没有会丢的钩子） |
| 失败、吵闹或缓慢的钩子 | 不改变回复；慢钩子会拖延回合结束，除非标了 `"async": true`，插件就是这样做的 |
| Claude 恢复已知会话 id（`--resume <id>`） | 是，同一 id 同一文件 |
| Claude 的会话文件 | `~/.claude/projects/<folder>/<id>.jsonl`；完成的消息有 `stop_reason` `end_turn`，待处理的有 `tool_use`；API 或登录错误是带 `isApiErrorMessage` 和 `error`（`authentication_failed`、`model_not_found`）的条目；中断（Esc）是 user 条目 "[Request interrupted by user]"，不触发钩子；token 用量和模型在每条消息上。用量上限：未复现 |
| Codex 启动时遵守 `-c notify=[…]` | 是，headless 和交互模式都行；JSON 参数带线程 id 和最后一条消息。它还上报旁支会话（它的聊天命名会话），靠提示词区分；以及 Codex 代理启动的子代理的回合（`spawn_agent`、`followup_task`；2026-10-15 在 0.162.1 上见到），其上报带着该代理自己的首条提示词：这些靠会话文件区分，其首行指明 `parent_thread_id`。代理自己的文件把子代理的开始和结束记为 `SubAgentActivity` 条目。中断不触发任何东西，在文件里是 `turn_aborted`。它会顶掉用户自己的 `notify`，所以已有 `notify` 的用户得不到钩子 |
| Codex 的会话文件 | `~/.codex/sessions/<date>/rollout-…-<id>.jsonl`：`task_complete` 带最后一条消息，`token_count` 带上下文窗口，`turn_context` 带模型；`codex resume <id>` 继续同一文件 |
| 服务商截断的回合，在三个文件里（2026-10-12 在本机真实会话中见到） | Codex：`task_complete` 带 null 的 `last_agent_message` 和 `error: {message, codex_error_info}`，见于 "Selected model is at capacity"（`server_overloaded`）、一个 401 和一个 400（都归 `other`）；之前没有单独的 `error` 事件。pi：`stopReason: "error"` 带 `errorMessage`，见于被关闭的 socket、"servers are currently overloaded"、网关 502/503/507、提前结束的流、未知服务商和被标记的请求。Claude Code：带 `error` 和 `apiErrorStatus` 的 `isApiErrorMessage` 条目，只见过 `model_not_found`（404）。未复现：Claude Code 对服务器过载或限速的条目、三者中任何一个的用量上限，以及 Codex 其他的 `codex_error_info` 值（在见到之前只能靠状态和措辞区分）。裁剪过的结尾是测试的输入（`tests/fixtures/endings`） |
| Codex 的快速模式 | 在 `codex`、`codex resume` 和 `codex exec`（0.160.0）上给 `-c service_tier="fast"`：随后每个请求带 `service_tier: "priority"` 和路由提示 `tier=priority`，把 Codex 指向本地服务器并在其 trace 日志中见到；不给则没有该字段。`"priority"` 效果相同，未知 tier 被丢弃并告警；快速覆盖之后给 `-c service_tier="default"` 不发 tier 也不告警。`codex debug models` 列出每个模型的 tier（`gpt-6.1-sol`："Fast, 2x speed, increased usage"）。服务端自己的回复仍称 tier 为 `default`，所以测了速度：同一篇 1,800 token 的文章快速模式 81 到 87 秒，不开 102 到 107 秒，各三次。在看板启动的评审者里，Codex 自己的日志（`logs_2.sqlite`）在 `reviewerFast` 开时把会话 tier 记为 `priority`，关时记为 `default`。评审主要跑命令和读代码，收益很小：同一提交两种方式评审，快速 212 秒、普通 226 秒，一个小任务的评审 66 秒和 69 秒 |
| pi `-e <file>` | 扩展收到 `agent_settled`；会话文件是 `~/.pi/agent/sessions/<folder>/<time>_<id>.jsonl`，每条消息带 `stopReason` 和 `errorMessage`；`--session-id` 按确切 id 创建或重开 |
| 常驻规则用的启动 flag | Claude `--append-system-prompt-file`、pi `--append-system-prompt <file>`、Codex `-c developer_instructions=…`：三者都能让规则贯穿整个会话。herdr 拒绝含换行的启动参数，所以用文件。用于工作代理和评审者；协调者的规则改为放在其文件夹里的一个文件 |
| 启动文件夹里的规则文件（协调者的规则） | 真实试过，关闭工具，在一个只放这些文件的文件夹里：Claude Code 2.1.296 读 `CLAUDE.md`，旁边有 `AGENTS.md` 时也会自己读它；`CLAUDE.md` 只含 `@AGENTS.md` 时，`AGENTS.md` 被喂给它一次。Codex 0.160.0（`codex exec`）和 pi 1.1.0（`pi -p --no-tools`）读 `AGENTS.md` |
| 文件夹移动后的会话 | Claude Code 2.1.296 以文件夹的真实路径为保存的会话做键。文件夹移动、旧路径做成指向它的链接后，从那里执行 `claude --resume <id>` 找到了该会话并继续，此后保存在新路径下 |
| 用户级技能文件夹 | `~/.claude/skills`、`~/.codex/skills`、`~/.pi/agent/skills` 存在；插件的 how-to 刚写进去，Claude Code 就把它列为一项技能 |
| herdr 的侧栏显示插件的面板和空间元数据 | **默认不显示。** 状态标签和 `$summary` / `$progress` 令牌只有在 `config.toml` 的 `[ui.sidebar.*] rows` 列出时才显示；不设置时只有 `--display-agent` 和空间名会显示。因此任务处在什么状态看代理的显示名，任务的编号和标题看其空间名，首次运行会为其余各行提供配置（[herdr 的侧栏](sidebar.md)） |
| herdr 的侧栏怎么画含多个值的行（2026-10-13 试过，tmux 内的临时会话，面板里跑真实 Claude Code，之后在 [herdr 的侧栏](sidebar.md) 里走完整个任务流程） | 各值用 ` · ` 连接；行宽超过侧栏时宽度分摊给各值，每个在自己末端用 `…` 截断：短的保持完整（`Reading the side… · 8%`），两个长的都被截（herdr 默认 26 格里是 `Reading t… · 8% 16m l…`）。所以必须只在末尾截断的几项事实放进同一个值（`agent · working · R#1 PR#113` 是一个，即代理的显示名），任务编号放在其空间名的最前面。带 `hide = true` 的规则只按值自身文本隐藏（`{ token = "tab", rules = [{ equals = "Agent", hide = true }, { starts_with = "Reviewer #", hide = true }] }` 隐藏了任务的标签页名，别的没隐藏）；没有因一个值而隐藏另一个值的机制。空间名 60 个字母时侧栏仍是 26 格（没见到按名字加宽）；`sidebar_width = 40` 配 `sidebar_max_width = 40` 变成 40。`workspace rename <id> <text>` 重命名空间，其行立刻跟随。从命令写入的 `--display-agent`、令牌和空间名一秒内显示，无需重载。空值不显示并连带去掉它的 ` · `；没有值的行不画。条目旁的圆点从不因这样写入的任何东西而改变。`pane list`、`pane get` 和 `agent get` 返回面板的 `display_agent` 和 `tokens`；`herdr server stop` 后重新启动，面板都还在但两者都没有（`display_agent: null`），看板一次 tick 就把它们写回去 |
| 元数据令牌存活多久（2026-10-13 在临时会话中试过，带 `--ttl-ms` 的 `pane report-metadata` 和 `workspace report-metadata`） | 写入时带存活时间的令牌到期就自己消失（`pane get` 不再有它），期间写了什么都一样：之后上报状态标签、其他令牌，或 `--clear-token` 其他名字都不能取消它的过期，另一个来源的上报也不能。不带 `--ttl-ms` 再写同一令牌让它留存到被清除；带 `--ttl-ms` 再写则重新计时。不带存活时间写入的状态标签比令牌活得久。所以令牌绝不会超过自己的时间：留存更久的行是被再次写入的，或写成了常驻。看板曾做错的是它自己的问题：没有任何东西在任务停止进行的那一刻把观察者的进行中行（`summary`、`progress`、`eta`，各三分钟）从面板上撤掉，而且空间的 `summary` 有两个写入者：标签（写成常驻）和观察者的行（写成会过期）。两者已在 `crates/flow/src/flow/sidebar.rs` 解决，此后只有它一个写入者：一切都写成常驻，词变了就替换 |
| 用 `pane report-agent --state blocked` 告知 herdr 任务被阻塞（2026-10-13 试过，同一会话，面板里真实 Claude Code，来源 `plugin:omh`） | 面板的代理立刻读作 `blocked`（`state_change_seq` 加一），herdr 随后不再读代理的屏幕：往面板输入词句（`pane send-text`，再回车）让代理工作并回答，而 herdr 一直说 `blocked`，没有状态事件；期间 `agent prompt` 被拒（`agent_blocked`："is blocked and requires interactive input"）。`pane release-agent` 分三步交回状态，中间什么都没发生也一样：带新 `completion_seq` 的 `idle`（一个从未有过的已完成回合），然后 `unknown`，然后完全没有 `completion_seq` 的 `idle`。在 `blocked` 之上报 `idle` 同样计一次完成。所以该上报接管了面板的整个状态，如 herdr 文档对 `report-agent` 所说（"when an integration also reports state, herdr uses those reports instead of reading the screen"；文档让集成之外的钩子改用元数据）。看板**不**用它：它的钩子必须在你往代理自己的面板输入时看到代理重新开始工作，被阻塞的代理拒绝 `agent prompt`，而释放时跳变的 `completion_seq` 在钩子看来像一个它漏掉的回合。所以等待中任务的 herdr 自带圆点仍是 herdr 的（done 或 idle）；标签和它旁边的行说明任务处在什么状态 |
| `herdr server reload-config` 后新增的 `[[keys.command]]` | 无需重启即可生效。坏文件报为 `failed`，herdr 保留旧配置；一个键绑两次是 `partial`；`herdr config check` 验证 `HERDR_CONFIG_PATH` 给出的候选配置 |
| herdr 的配置路径和默认按键 | `~/.config/herdr/config.toml`；`herdr --default-config` 列出大多数内置按键（`prefix+w` 是 herdr 的空间选择器；`prefix+a` 空闲），但没有 `copy_mode`（`prefix+[`）和 `swap_pane_left/down/up/right`（`prefix+shift+h/j/k/l`），配置参考里有：`crates/core/src/herdr/default-keys.toml` 全都有。绑定可以是一个键或一个列表；`[keys.indexed]` 仍被读取；`herdr config check` 不报告一键绑两次 |
| herdr 内部终端显示什么 | herdr 的 `theme.auto_switch` 开启、或主题用终端自身颜色绘制时，看板向终端询问它的背景（OSC 11）和 16 色（OSC 4）；herdr 用客户端所在终端的颜色为其面板作答。没有回答（临时会话背后没有真终端）：深色，以及 xterm 的颜色。`theme light` 手动选择 |
| 面板里的鼠标（2026-10-12 在 tmux 内的临时会话中试过，鼠标码输入到客户端） | 没请求鼠标的全屏程序：herdr 把滚轮作为 `↑` 和 `↓` 发给它，一格一步，不带位置，拖拽在面板里选择并复制（herdr 自己的选择）。请求了鼠标的程序（`?1000h`，带或不带 `?1002h`）：herdr 把一切都按面板自己的坐标交给它：滚轮、按下、拖动、松开，按着 `Shift` 也一样，自己不做任何选择。面板里写入的剪贴板码（OSC 52）传给终端，附 herdr 的 `copied to clipboard` 提示。所以看板请求鼠标、接管滚轮、自己选择和复制（[鼠标](mouse.md)）：滚轮划过聊天区和列表、`Alt+U` `Alt+J`、一次复制了两行的拖拽，都在面板里的真实看板上见到过。未试：弹窗（`--placement overlay` 的插件面板需要把插件装进 herdr），以及真实 Mac 终端的 `Cmd` 和点击链接 |
| herdr 的主题，对插件而言 | 不传给插件。看板读取 herdr 读的 `[theme]` 和 `[ui]` 表（用完整的 TOML 读取器 `toml` crate），并按 herdr 0.9.3 的方式解析（`src/app/state.rs` 的调色板，`src/client/shell.rs` 的标记：`●` 进行中、blocked 和完成，`○` 待办；`status_indicators = "symbols"` 时是 `◐ × ✓ ○`）。文字用 herdr 的 `text` 颜色，主题设了 `sidebar_bg` 时侧板以它为底。评审页取写入时生效的主题；herdr 跟随终端的地方（`auto_switch`、`terminal` 主题），页面跟随浏览器的明暗设置。在 herdr 里改主题，看板下次打开时显示。标记、分组标题、进度条和百分比取 herdr 对任务状态的颜色（进行中 `yellow`，blocked `red`，完成 `teal`，待办 `green`；评审中也是 `green`；看板为运行中的评审和自动化用的自有蓝色和紫色不是 herdr 颜色；排队中的任务用待办的暗灰：[颜色](colours.md)）；任务标题无论什么状态都是 herdr 的文字颜色；标题之后其行内文字和任务面板里每个标签都变灰（`overlay0`，`terminal` 主题里还更淡，其灰色就是大多数终端的文字颜色）；面板的标题是粗体，分隔线和评审框是 `surface1`，只是你默认值的设置项是 `subtext0`；文字、次级文字、分隔线和选中行用 herdr 的侧栏颜色（`text`、`subtext0`、`overlay0`、`surface_dim`、`selection_bg`/`active_row_bg`）。评审分数在评审者说合并时是 `green`，说不合并时是 `red`：颜色即结论，没有文字。未设主题时 herdr 显示 Catppuccin（`herdr --default-config`：`# name = "catppuccin"`），看板也一样；用 `--remote` 打开的 herdr 用另一台机器的配置绘制，看板读不到（[颜色](colours.md)）。没人开始的任务是 `todo`，看板自有的暗灰，文本框的边框对协调者是 `text`，对其代理是任务的状态色 |
| 谁设定面板的终端尺寸 | herdr 自己的窗口，直到该终端上有 `herdr terminal attach`：终端随即有 attach 的尺寸（93×33 变成 200×50），herdr 窗口调整大小时保持不变；attach 一结束就恢复为窗口的。herdr 在任何地方都不列附着的查看者。`pane layout --pane <id>` 给出面板在 herdr 窗口里的位置：其终端是该位置减去最多 3 列 2 行（边框、滚动条）；对放大面板（布局中 `zoomed` 时的 `focused_pane_id`）则是标签页整个 `area` 减去同样的量。第二个完整的 herdr 客户端会同时改变布局和终端，不会被误认成这种情况 |
| 一个会话上两个 herdr 窗口时面板终端算谁的尺寸（2026-10-12 见到，临时会话里两个客户端 170×48 和 54×26，还有真实会话的日志和终端） | 最后输入过或调整过大小的那个窗口：每个面板的终端立刻取该窗口的尺寸，另一个窗口被输入时改回去。`pane layout` 对最后做过其中任一的窗口作答。另一个窗口只调整过大小后，两者不一致：布局是调整过大小那个窗口的，终端还是第一个的，直到有按键输入。`terminal session control --takeover` 此时不改变什么：尺寸只在持有 control 期间归它，herdr 在离开时把终端放回去。真实会话的日志把窗口记为 `client connected … cols=47 rows=44` 和 `cols=138 rows=44`，看板的终端在 0.6 秒内 141×46 → 138×44 → 141×46 |
| 会话上有两个不同尺寸的窗口（其一是远程，`herdr --remote`）时辅助操作的启动和结束（2026-10-12 在临时会话中见到，记录了面板自身的 resize 事件） | 没有调整大小：后台的 `workspace create`、`tab create`、`agent start`、`tab close` 和 `workspace close`，在最后输入的是本地窗口、最后输入的是远程窗口、以及远程窗口只被调整过大小之后，都没有改变任何面板的尺寸。随后派生的面板得到持有会话那个窗口的尺寸 |
| 看板的辅助操作对看板的面板做了什么（2026-10-12 见到，每秒 20 次监视真实看板） | 什么也没做：在另一个空间带 `--no-focus` 的 `workspace create`、`worktree create` 和 `pane split`、在那里的 `agent start`、`pane close` 和 `workspace close`，都没有改变聚焦空间里面板的终端尺寸和 `pane layout`，在带评审者的任务整个生命周期和协调者启动又停止的全程都是如此。从看板自己的面板分割出一个面板会把它减半 |
| 通过 `herdr agent start` 的过长启动命令 | herdr 把它输入面板的 shell，而终端一行最多取 4096 字节：Codex 的 `-c developer_instructions=…` 里的协调者规则被截断到达，停在 shell 续行提示符，启动超时。现在该 flag 只指向一个规则文件（起初规则也随第一条消息送入；自从输入框变成逐字透传后就不再如此，而这还没对真实 Codex 跑过）：Codex 连续三次全新启动和两次重启各约四秒起来，重启保持同一会话 |
| 代理自己的新开聊天命令（Claude Code `/clear`、Codex `/new`、pi `/new`） | 每个都为同一文件夹另开一个会话文件，第一个原样保留（pi：`<time>_<id>.jsonl`，首行指明工作文件夹和 id；`--session-id` 能重开它）。协调者有一个没有别处聊天的文件夹，所以自它开始以来保存在那里的最新会话就是它所在的会话。实测三个程序：说 "pineapple"，新开聊天，说 "watermelon"；看板显示 watermelon，重启后代理仍回答 watermelon |
| 协调者文件夹里保存的其他会话 | Codex 派生的辅助代理会在那里存自己的一份，比主会话新：其首行有 `"source": {"subagent": …}` 和 `parent_thread_id`（在 0.160.0 的真实文件里见到）；headless 的 `codex exec` 是 `"source": "exec"`。Claude Code 把辅助的旁支会话放在 `subagents` 子文件夹，每条指明其 `entrypoint`（交互式程序是 `cli`）。只有交互式程序自己的聊天才被当作协调者的 |
| `herdr tab create --workspace <id> --cwd <folder> --label "Reviewer #1" --env KEY=VALUE --no-focus`（2026-10-12 在单独的临时 herdr 中见到） | 回答新标签页及其唯一面板（`tab`、`root_pane`），在该文件夹的 shell 里，变量已设置。焦点留在原处（`tab list`：第一个标签页仍 `focused`），第一个标签页的面板在 `pane layout` 里保持其矩形（前后都是 120×40；分割会减半）。`tab rename <id> Agent` 重命名标签页；新空间的第一个标签页标为 `1`。关掉标签页的最后一个面板会关掉标签页，`tab close <id>` 连同其面板关闭它，已关闭标签页的 id 不再发放（`w1:t2` 关闭后，下一个是 `w1:t3`）；对已消失的标签页执行 `tab close` 回答 `tab_not_found` |
| 同一仓库的多个工作树作为同一空间的标签页（2026-10-13 在单独的临时 herdr 中见到，Claude Code、Codex 和 pi 各占一个标签页） | `worktree create` 创建的空间接受更多标签页，其 `--cwd` 指向同一仓库的另一个工作树（`tab create --workspace <id> --cwd <that worktree>`），各自启动的代理在自己的工作树里干活；关掉空间的第一个标签页（`worktree create` 建的那个），空间和其他标签页原样保留，留下的标签页在另一个工作树里时也一样；之后 `workspace close` 像关任何空间一样关掉它。一个同时做几次的任务就是这样把 `Agent A`、`Agent B` 和 `Agent C` 放进一个空间的（[并行尝试](attempts.md)） |
| 主空间，以及移进标签页的面板（2026-10-12 在单独的临时 herdr 中见到，面板里运行 Claude Code） | 不带标签的 `workspace create --cwd ~` 创建标为 `~` 的空间；标签跟随空间第一个标签页面板的目录（在那里 `cd /tmp` 后是 `tmp`，其标签页是最后一个时是协调者的文件夹名），所以不是固定名字。`--no-focus` 把焦点留在原处，会话的第一个空间除外。`pane move <pane> --new-tab --workspace <id> --label Coordinator --no-focus` 把面板放进该空间的新标签页，其程序继续运行：同一终端 id，代理保持名字和状态，焦点和其他面板的矩形原样不变，面板离开的那个空间在它是其最后一个面板时被关闭（`closed_workspace_id`）。面板有了新 id（`w4:p1` 变成 `w1:p5`）；`pane get` 仍对旧 id 作答，`agent get` 不会。程序自己在 `HERDR_PANE_ID` 里还是旧 id。`tab list --workspace <id>` 给出每个标签页的标签和面板数；标签页的标签是纯文本（`tab rename <id> ""` 清空它，从未命名的标签页用数字标记），所以改回去就能恢复。用看板的代码实测：第一条消息创建了主空间和 `Coordinator` 标签页，焦点留在另一个空间；一个被放回自己空间的协调者（主空间已关）在下一条消息时被移进新主空间的标签页，并从同一会话作答；看板的标签页打开时是 `omh`，按两次 `Ctrl+C` 后又是 `notes`，`omh list` 没有改任何名字。对空间的最后一个面板执行 `pane close` 会关闭空间（`workspace list` 不再有它），不带 `--workspace` 的 `pane list` 列出所有空间的面板及其目录 |
| `herdr workspace create --env KEY=VALUE` | 在该空间启动的代理拥有该变量：协调者的命令能触达存放在另一个数据文件夹里的看板 |
| 协调者跑在 Claude Code 和 Codex 上的实测 | Claude Code：由第一条消息启动，从文本框和它自己的面板向同一会话追加消息，在指定仓库里加了任务，重启后仍记得之前的回答（`--resume`）。Codex，一次性试过：一条消息，任务已加，从其文件夹和启动时间找到它自己的会话并读到最后一条消息。pi：作为协调者启动，三条消息，它自己的 /new 和一次重启（见上） |
| 从启动钩子启动的看守 | 在钩子退出后存活，更新时自我替换；没等到 "for hours"（数小时）那么久，所以钩子事件和看板也会启动它 |

## Herdr 缺少什么

看板的弹窗要做到和 herdr 自己的菜单一样，herdr（0.9.3，以及 2026-10-13 的 main 分支）还缺
以下这些。每一条都该向 herdr 提出；没有一条能由插件顶替。

- **属于窗口而不是标签页的弹窗。** 服务端记着弹窗打开时所在的标签页，另一个标签页显示时就把
  弹窗隐藏。弹窗若跟着打开它的客户端，盖住该客户端显示的任何东西，
  标签页变化时插件就不必关闭它。
- **弹窗持有键盘时 herdr 自己的按键仍有效**，至少打开它的那个键：现在每个按键都进
  弹窗的程序，插件必须读 herdr 的配置才知道自己的键，herdr 的其他按键
  一概无效。同样的缺口也是另一个插件的弹窗显示时看板键没反应的原因；给插件一个按键被拦下
  的事件至少能让看板说出来。（任务 135 记为 "keys swallowed by
  floating windows"：这项工作再次遇到它，只为看板自己的键绕开了它。）
- **从命令行关闭弹窗的办法**（`herdr popup close`）：socket 有 `popup.close`，
  命令行没有。
- **打开的是谁的弹窗**：`ui_busy` 不说，`popup.close` 关谁的都行。所以看板只关闭
  自己运行中的看板所拥有的那个弹窗。
- **盖住整个窗口（含侧栏和标签栏）并把背后内容压暗的弹窗**，如果 herdr 的菜单
  有朝一日做到的话：插件的弹窗居中于面板区域，什么也不压暗。

这次工作解决的：看板弹窗关闭时放大面板丢失放大状态（任务 135 记下的另一件事）。
那是 overlay 造成的；herdr 的弹窗不动布局。
