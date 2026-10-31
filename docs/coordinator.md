# 协调者（所有机器的协调者）

**本机的顶层协调者（Coordinator）就是顶行的**：`All machines` 显示全部机器（`the coordinator of all machines`，所有机器的协调者），或者在只有本机的看板上，是本机的行（`vm-str's coordinator`）。看板打开时停在顶行，聊天也从它开始。输入要做的事，它把它们变成任务（task）。它是一段长期存活的代理会话，不是每条消息一次调用：你输入的一切都进入同一段对话，所以「把那个也派发出去」有效，它也会反问你。Herdr 和命令仍然称它 Coordinator。

每台机器和每个仓库（repo）也有自己的协调者，在它旁边：在机器行或项目行上按 `Space`（[机器与项目协调者](machine-and-project-coordinators.md)）。除此之外没有别的主协调者。

- `Enter` 发送消息。**启动什么由它自己定**：说「and start it」，它会给新增的任务放上代理。
- `Alt+C` 在看板（board）上显示/隐藏它的最新消息。

```
omh coordinator                     how it stands
omh coordinator tell <text>         send it a message
omh coordinator tell <text> --delegate   and start every task it adds
omh coordinator restart             start it again, same conversation
omh coordinator new                 a fresh conversation
omh coordinator new [--agent KIND[/MODEL]] [--effort E]    a different one, this once
omh coordinator stop
```

restart 和 new 是命令，不是看板按键。你也可以直接让协调者执行它们。`new --agent …` 只影响下一次：你的默认值保持不变。

在找不到 herdr 的地方运行 `coordinator tell`（SSH 强制命令、cron、systemd：herdr 不在 PATH 上，不在 `~/.local/bin`、`~/.cargo/bin`、`~/bin`、`/usr/local/bin` 或 `/opt/homebrew/bin`，也不在登录 shell 的 PATH 上），消息会留在队列里但不会说 `sent`：它会说明消息已入队、未送达以及原因，并以退出码 1 退出。看板和它的看守（keeper）在 herdr 里运行后就会送达；把 `HERDR_BIN_PATH` 设为 herdr 的完整路径则从那里送达。`omh doctor` 会点出在队列中等待超过一分钟的消息。

## 它的行与它的设置

**它的行就是顶行**：`All machines`，或者唯一的那台机器。它的读法和所有协调者的行一样：它的标记（`○` 空闲，`◐` 进行中，`?` 它问你），右侧是时间，下面是它在做什么（开启[自动驾驶](autopilot.md)时，名字后有 `[autopilot]`，下面是自动驾驶在做什么）（[机器与项目协调者](machine-and-project-coordinators.md)）。选中另一台机器时，顶行是那台机器的，本机不在看板上。

- 在那里按 `Space` 与它交谈；在别的行按 `Space` 与那行的代理交谈，再按一次回到这里。`Alt+O` 进入它。
- 在它的行上按 `Alt+P`、`Alt+T` 和 `Ctrl+E` 可查看它和它的设置（settings）；`Ctrl+S` 启动它，`Ctrl+X` 停止它，`Ctrl+O` 给它开启/关闭自动驾驶。
- 它的设置（`coordinator.*`）的查找方式与任务相同：**此协调者 → 你 → 内置**。只给它单独设的值放在 `config.json` 的 `coordinators` 部分（`set … --main-coordinator`）。改动从它的下一次全新会话起生效（[设置](settings.md)）。

## 你的消息与你当时所在的位置

你的话原样送达。你当时所在的仓库和打开的任务记录在每条消息旁，协调者会自己去查（`omh coordinator where` 列出最近几条消息，各带自己的位置）。

下一条人工消息发给协调者时，看板会附上它名下新完成任务的编号和状态。Coordinator 使用 omp 时，不应使用 omp 的子代理/task 工具或 TODO planner 管理看板工作；只把通知中的编号与当前上下文对齐。通知不包含任务标题、描述、收尾说明或代理输出，不会单独唤醒协调者，也不会派生新任务。

从协调者自己的面板（pane）打开的看板停在它上次所在的仓库：那个面板的文件夹不属于任何仓库。

## 启动它新增的内容

你只管发消息，协调者自己决定怎么做。它添加任务，只有你在话里要求时才启动它们（「fix the login page and start it」）。开着自动驾驶时它自动启动新增的任务（[自动驾驶](autopilot.md)）。输入文字后按 `Ctrl+S` 无效：没有第二种发送方式。

- 从 shell 运行 `coordinator tell <text> --delegate`，会让该消息新增的每个任务立即派发（delegate），不论多少，无需再确认。消息在你的话下面带一行说明这一点（看板聊天只显示你的话），协调者在该消息的回合（turn）中新增的任务由 `add` 本身派发。
- 什么都不新增的消息（提问、查状态、关于已有任务的一句话）不会额外派发任何东西。

## 它说它做了什么

当它在看板上做了操作，它的消息末尾会有一行短句，用平实的话点出相关 id：新增、更新（标题、描述、设置）、移除、启动代理、批准、归档、恢复。例如：「Added proj/12 and started an agent on it」。命令用同样的话回答（[角色与命令](roles-and-commands.md)）。

## 进入它

在看板上你只看到它说的和你说的，永远看不到它程序的界面（[看板](board.md)）。要看完整对话、它的对话框、斜杠命令和菜单，在 Herdr 里**进入它**：

- 它是主空间（home space，Herdr 标为 `~`）的 `Coordinator` 标签页（tab）里的普通代理。
- 插件动作 `omh.coordinator`（给它绑一个按键绑定）从 Herdr 任何位置直达它的面板，没运行就先启动它。
- 返回的方式和从任何代理返回一样：你的看板按键，或 `omh`。

## 它是什么

- 普通的 Herdr 代理，在主空间（`~`）一个名为 `Coordinator` 的标签页里。
- 它在看板数据文件夹里自己的文件夹中运行。`OMH_COORDINATOR_DIR` 可指定另一个；那个文件夹不要作他用，因为看板会把协调者的规则写在那里。
- 它的启动参数和任何工作代理（worker）一样：全部工具，毫不保留。
- 你的第一条消息启动它，之后它一直在。如果它的程序退出或 Herdr 重启，你的下一条消息会在同一段对话里再次启动它。
- **关闭看板不会停止它**，关看板时看板也会这么说。一个协调者服务本机的所有看板（标签页、弹窗、web 看板）和 `coordinator tell`，它可能还有一个回合在工作。`omh coordinator stop` 关闭它的标签页；你的下一条消息会在同一段对话里再次启动它。
- 代理自己的新会话命令（`/clear`、`/new`）会被遵从：会话和重启都跟随它当前所在的对话。
- 它的程序、模型和 effort 是设置（`coordinator.agent`……）：见[角色与命令](roles-and-commands.md)。
- `coordinator new`、`restart`、`stop` 不改动[自动驾驶](autopilot.md)。对开着自动驾驶的协调者执行 `stop` 会提示：它下次被唤醒时会再次启动。

## 它的常备规则是一个文件

规则（协调者是什么、它怎么用看板）写在它启动文件夹里的 `AGENTS.md`（`~/.omh/data/coordinator/<session>/chat`，见[主文件夹](home-folder.md)）。程序每次对话开始时自己读这个文件。所以无论它自己执行 `/new` 或 `/clear`、执行 `coordinator restart` 和 `coordinator new`、执行 `coordinator new --agent <another kind>`（下一个程序读同一个文件），还是你手动在那个文件夹里启动程序，它都知道自己的规则。

| 种类 | 读取 | 看板因此写入 |
|---|---|---|
| Claude Code | `CLAUDE.md`（2.1.296 也会读旁边的 `AGENTS.md`） | `AGENTS.md`，外加一个只做转交的 `CLAUDE.md`（`@AGENTS.md`）：规则只送达一次 |
| Codex | `AGENTS.md` | `AGENTS.md` |
| pi | `AGENTS.md` | `AGENTS.md` |
| 其他种类 | 未知 | `AGENTS.md`，并在它第一条消息前加一行指向该文件 |

- **保持最新。** 每次启动和每条消息之前都会重写这些文件。所以新版本的规则会在运行中的协调者之下就位，它在下次新会话或重启时读到。
- **运行中的它会得知规则变了，仅一次。** 看板保留每个协调者程序启动时规则的指纹。当看板要打入你的下一条消息时规则已经不同（看板的新版本、新的 `coordinator.instructions`），消息前会加一行："[The board was updated: your standing rules in <file> changed; read them again.]"（看板已更新：<file> 里的常备规则变了，请重读）。自动驾驶唤醒保留开头的 `Autopilot:`：这行加在其后，并说明这条消息是真实的唤醒，无论旧规则怎么说。这只会发生一次，绝不会作为它自己的消息：那会多花一个回合，还可能插进某个回合中间。由旧版看板启动、没有保留指纹的程序也会得知。
- **你自己的笔记会保留。** 看板只拥有两行标记之间的那个块。你在 `AGENTS.md` 或 `CLAUDE.md` 里写在它上方或下方的内容会留下，协调者也会读到。
- **每种程序一种方式，绝不用两种。** 协调者不再通过启动参数传规则（过去是：Claude 和 pi 的附加系统提示词，Codex 的 `developer_instructions`）。文件对每种程序都一样有效，而且比程序活得久。参数随进程消失，得由启动程序的人再传一遍，而且 Codex 的参数没法在一行启动命令里带这么多文字。有了文件，参数就会把同一件事说两遍。只有看板不认识的种类——它可能读也可能不读 `AGENTS.md`——才会额外得到那行指向。
- **工作代理和评审者（reviewer）不同。** 它们在你的仓库里工作，仓库的 `AGENTS.md` 是你的，所以它们的规则仍随启动传递。

## 它在 Herdr 里的位置

它在侧栏（sidebar）里没有自己的空间。它是主空间（Herdr 标为 `~`）里一个名为 `Coordinator` 的标签页：整个标签页，绝不是你某个标签页里的面板。在侧栏的代理列表中它叫 `Coordinator`，状态显示在看板输入框上方（`Coordinator · idle`），下面是它最新的消息（[Herdr 的侧栏](sidebar.md)）。

- **从任何地方找到或启动。** 来自面板里看板的消息、Herdr 弹窗里看板的消息，或来自 shell 的消息（`coordinator tell`），都会送达运行中的协调者。如果没在运行（它的程序退出、它的标签页或主空间被关、Herdr 重启），消息会在同一段对话里再次启动它，在主空间的一个 `Coordinator` 标签页里。永远不会有第二个，也不会为它新建空间。
- **本机没有主空间？** 会先建一个（在你的主文件夹，按 Herdr 的做法），然后加上那个标签页。
- **你眼前的画面不变。** 标签页以及不得不新建的主空间打开时不抢焦点：你停在原处，看板和其他每个标签页的大小布局都不变。
- **哪个空间是主空间。** 标着 `~` 的那个。Herdr 按空间里第一个标签页所在的文件夹给空间命名，所以你在那个标签页里 `cd` 到别处，标签就会变。看板记得它上次把标签页放在哪个空间，当没有空间标着 `~` 时就用那个。任务的空间永远不会被占用。
- **标签页可以改名。** 看板按面板找协调者，就像找任务的代理和评审者一样（[任务的标签页](tabs.md)）。如果看板跟丢了（它保存的状态丢了），它会在协调者自己的文件夹里找那个面板，不论它在哪个空间：在那里运行的协调者会收到你的消息，在它当前的对话里；那个文件夹里空着的 `Coordinator` 标签页会被复用，而不是再开一个。
- **`coordinator stop`、`restart`、`new` 只关它的标签页**，绝不关周围的主空间。

### 从旧布局迁移

协调者过去有一个名为 "Coordinator" 的独立空间。仍在那里面运行的会被原样移进标签页：同一个程序，做到一半的事继续，什么都不重启。Herdr 在最后一个面板离开时关闭旧空间。

- 更新后一分钟内自动完成（由看守或打开着的看板完成），如果先来了你的下一条消息，就在那时完成。
- 对于不会移动面板的 Herdr，你的下一条消息会关掉它在旧空间里的面板，在标签页里同一段对话中重新启动它。`stop`、`restart`、`new` 也一样。
- 只会关掉它自己的面板。如果你把自己的面板或标签页放在那个空间里，它会连同它们保持打开。
