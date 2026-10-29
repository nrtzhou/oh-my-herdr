# 其他机器

**每台机器保存自己的任务，按 `Alt+M` 把别人的任务显示到你的看板上。** 按它轮流切换：所有机器，然后按 Herdr 侧栏的顺序逐台显示（包括本机）。对别的机器上任务的操作在那台机器上执行，走 SSH。工作从不在机器之间移动。

你看哪些机器是筛选行的第一项，用基础色，没有光标：`All machines`（素色），或 `otherbox`（高亮，和当前显示的筛选器一样）（无响应时黄色）。点击它也轮流切换（[看板](board.md#过滤行)）。聊天下方不会有行来确认选择：那一项说明显示的是谁的看板，聊天框说明谁收你的话。在 web 看板上，点它会列出所有看板供你选（[web UI](web-ui.md)）。

## 配置另一台机器

机器只通过 Herdr 发现：你用 `herdr machine add` 添加的，或你在 `board.machines` 里点名的。没有 Tailscale 自动发现。它们按 Herdr 的机器侧栏顺序列出（[机器顺序](machine-order.md)）。

没有别的机器时就没有「所有机器」可显示，终端看板和 web 上都是：`Alt+M` 向 Herdr 询问它的机器（`Looking for other machines…`），没有就说明，一有另一台就立刻显示所有机器。曾显示所有机器的看板，在别的机器消失后回到本机自己的看板。

每台别的机器需要做一次：

```sh
# git, gh, the agent programs it will run, then:
git clone https://github.com/nrtzhou/oh-my-herdr && cd omh
herdr plugin link "$PWD"
sh bin/run.sh install             # gets the program and writes its launcher
```

……以及让登录 shell 的 PATH 上有 `omh`（下一节）。程序如何获取和更新见[安装](install.md)。

为什么用 SSH：插件在别的机器上的命令走插件自己的 SSH 连接，因为 Herdr 没有在那边运行插件命令的手段。

### `omh` 必须在登录 shell 的 PATH 上

看板用 `$SHELL -lc 'omh remote list'` 询问别的机器：一个没人打字的登录 shell，不是交互式的。只给交互式 shell 设的 PATH 不算数。很多系统上 `~/.bashrc` 会在「if not running interactively, return」那行打住，所以那行之后设的 PATH 找不到。

两种修法任选：

- 在 profile 文件里设 PATH：bash 是 `~/.profile` 或 `~/.bash_profile`，zsh 是 `~/.zprofile`，fish 是 `~/.config/fish/config.fish`（放在它的 `if status is-interactive` 块之外）；
- 或把启动器链接到登录 shell 已有的文件夹里：`ln -s ~/.omh/data/bin/omh ~/.local/bin/`。

任何登录 shell 都行，fish 也行：看板只给它所有 shell 都一样读的词，让 `/bin/sh` 带着命令启动它。

从看板所在的机器测试。最后是一行 JSON 就说明一切正常：

```sh
ssh <machine> 'exec $SHELL -lc "omh remote list"' | tail -c 200
```

shell 启动文件在那行前后打印的任何东西（横幅、告别语、stderr 上的任何输出）都无妨，哪怕再多、哪怕本身就是 JSON：看板只读最后一行属于它自己的回答。

## 检查每台机器：`omh machines`

它列出本机和每台别的机器：是否响应、插件版本、有多少任务，或者原因和该怎么做。它用与看板相同的代码解析回答。

```
vm-home  this machine: a login shell (/bin/bash) finds omh 0.2.0
vm-a     answers · omh 0.2.0 · 136 tasks  (exedev@vm-a)
vm-b     does not answer: omh is not found there  (exedev@vm-b)
    A login shell on vm-b does not find `omh`.
    …
```

对本机，它用同一个登录 shell 加纯净环境运行（账号、它的 shell 和一条短 PATH，也就是 SSH 给的环境）。所以你能在机器本身上看到别的机器能不能问到它。`--json` 以数据形式给出同样的内容。

## 机器不响应时

它绝不会显示成没有任务的机器。在它的任务该出现的地方，终端看板和 web 看板都显示一行说明，附原因：

```
vm-a: its tasks cannot be read: omh is not found there.
```

- 在所有机器视图里，其他机器的任务仍显示在它下面。
- 在那台机器自己的视图里（`Alt+M` 会轮到它，即使它不响应）你会看到其余内容：返回了什么、含义是什么、该怎么做。web 看板把它收在「Why, and what to do」后面。超过一屏时，方向键滚动，还有一行说没显示完。
- 看板每半分钟重问一次，这行留在原处。它绝不作为消息重复发送。
- 对停止响应的机器上的任务执行操作会说明原因，在它唯一的一条消息里。

| 行上说 | 发生了什么 | 该怎么做 |
|---|---|---|
| cannot connect（名字未知、连接超时、拒绝连接、从这里无法到达）、ssh could not connect | SSH 根本没连上机器 | 它开机了吗、在网络上吗？在 `herdr machine list` 或 `board.machines` 里核对名字和端口 |
| its host key changed | 机器出示的密钥与看板为它保存的不同 | 如果它被重新装过：`ssh-keygen -R <host> -f ~/.omh/data/known_hosts`（看板有自己的列表；端口不是 22 时名字写作 `'[<host>]:<port>'`，看板那行就是这么给的） |
| it does not let the board in | 没有可用的密钥，或者要密码：看板以批处理模式运行 SSH，没法输入 | `ssh-copy-id <machine>`，或把你的密钥加载进看板运行所在的 SSH agent |
| omh is not found there、node is not found there、its node is too old | 那里的登录 shell 找不到程序（退出码 127），或找到了基于 Node 的旧版，但没有 Node 或 Node 早于 22 | 按上文修好登录 shell 的 PATH；本版不需要 Node |
| it answered something else than the board's answer、it answered nothing、it answered with an error、it answered, but not with its tasks | 命令跑了但没打印看板的列表：插件太旧、启动文件打印后卡住、报错 | 在那边更新 omh；跑上面的测试看全貌 |
| it took too long | 20 秒内没有回答 | 看板自己再问；找找有没有启动文件在等什么东西 |

## 你能看到别的机器的什么

- **标题。** 显示所有机器时，每台机器是自己的标题，下面是它的仓库（或状态组）。行用它朴素的标签（`#1`）；上面的标题点名它的机器（[列表标题](list-headings.md)）。
- **自动化。** 机器的[自动化](automations.md)随它的任务一起来，和任务一样排在它的部分里：任务组之后的独立组（按仓库时：在它们的仓库下面），随它的标题折叠。开关、触发、删除和改设置都在拥有它们的机器上进行。
- **聊天。** 每个有人在做的任务，它的机器发来对话里最近说的几句话：它的代理的，或评审期间它的评审者的，从那边程序保存的会话里读出。聊天和任务面板把它们显示得和本机的一样，署名行也一样（[看板](board.md)）。
- **它的协调者。** 当看板单独显示另一台机器时，你输入的内容去那台机器的协调者，聊天显示那个协调者的会话。那台机器发来它的协调者状态（`idle`、`working`、有几条你的消息在等）和最近十几条对话。输入框把它们显示得和本机的一样：你的话亮色，它的回答灰色，没有标签，可以往回滚动。没有协调者的机器会说明（你在那里输入的内容会作为任务添加）。
- **插件太旧的机器**从缺失的版本认出来：新字段留空，看板说「update omh on <machine>」。它不发聊天：它的代理最后一条消息仍会显示，而对评审者或它的协调者，输入框会说这台机器需要更新。

## 进入别的机器的代理

在别的机器的任务上按 `Enter`，那台机器会先重新打开任务的空间（space）并重启已退出的代理，在同一段会话里。

- **在 Herdr 窗口里**，那台机器的 Herdr 把代理的面板放到前面，就像 `Enter` 对本机的代理那样。看板留在原处并说：`vm-a: its agent is in front there. Pick vm-a in herdr's sidebar to go to it: “<its title>”; Local there brings you back here.` Herdr 没有把窗口切到另一台机器的命令，所以那一下点击要你自己来。点过去代理就在那里。
- **回来：** 侧栏里的 **Local**（或 `prefix+w` 加本机的一个空间）带你回到看板，原样不变。在显示那台机器期间，看板自己的按键不行：Herdr 会在那边执行它，打开的是那台机器的看板。
- **在 Herdr 客户端之外**（纯 SSH），或机器的插件太旧时，你的终端会改为附到那一个代理上（`herdr agent attach`）。`ctrl+b q` 返回看板。

## 任务的工作设置随它走

当你给别的机器的任务放上代理（或给那台机器发话加一个任务）时，你的机器算出你给它的设置（你自己的，以及你对该仓库的条目）并随行发过去。它作为任务「随身携带」的内容保存在任务上，与任务跟踪的其他内容分开。在那里它站在那台机器自己的「你」的位置：在任务自己的设置之下、仓库文件之上。那边的 `show <id>` 会说 `sent from <your machine>`。

看板设置（同时几个代理、观察者（Watcher）、协调者）留在各机器上（[设置](settings.md)）。

## 你附带的文件随你的话走

给别的机器的代理或协调者的话可以点名本机的一个文件：你附带的（`@<path>`）、你拖进终端的（`/Users/you/Desktop/Screen\ Shot.png`，或带引号的），或你粘贴的图片，输入框显示为 `[Image 1]`（[粘贴图片](image-paste.md)）。话发出之前，看板把每个这样的文件经自己的 SSH 链接发到那台机器，话里点名它落在那里的位置。你什么都不用复制。

- **落在哪：** 任务的代理：在那台机器上那个任务的工作文件夹里，`.omh/files/` 下。协调者：在它在那里的自己的文件夹里，同样方式。那边没有协调者（话变成任务）：在那台机器的看板文件夹里，`files/` 下。Git 不理会那个文件夹。放在那里的文件 30 天后清除。
- **发什么：** 只发本机上有的文件。文件夹、这里不是文件的路径、指向系统文件夹或隐藏文件夹的裸路径（`/etc/hosts`、`~/.zshrc`）原样保留：它们指的是那台机器自己的。
- **单个文件最大 8 MB。** 更大的不发。
- **文件发不了时**（太大、机器不响应、被拒绝），看板说明原因，并且你的话一个字都不发：绝不发那边代理打不开的路径。
- **旧版本的机器**收不了文件：看板会说明。去那边更新 `omh`。

## 在 web UI（Ovis）里

web 看板像 `Alt+M` 一样显示其他机器，对它们的任务执行操作，并在 web UI 自己那台机器的视图里打开它们的代理（当 web UI 把那台机器列为它的一台 PC 时）。终端看板在 web UI 的终端视图里也能用，本机和别的机器都行。什么怎么用、缺东西时看板说什么：见 [web UI（Ovis）里的看板](web-ui.md)。
