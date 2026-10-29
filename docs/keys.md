# 按键、提问与 ? 菜单

**在看板上按 `?` 可以看到全部按键。** 本页末尾的表格也列出了它们全部。一条规则让它们好猜：

- **`Ctrl` 做事**（批准、停止、归档……）。
- **`Alt` 只显示或隐藏屏幕的一部分**，从不改动任何东西。
- **普通按键**（`Enter`、`Tab`、`Space`、`Esc`、方向键）用来移动和对话。把工作代理放到任务上（`Ctrl+S`）是在做事，所以它是 `Ctrl` 键；`Tab` 和 `Shift+Tab` 在顶部的[过滤行](board.md#过滤行)之间轮换。（`Alt+↑` 和 `Alt+↓` 是 Herdr 自己的：看板不接管它们。）

别的组合键都不做事，旧版本也没有任何一个键能作为这些键的第二种按法。

有两个键打破规则。`Ctrl+F`（查找）只改变列表显示哪些任务，按规则本该是 `Alt` 键；但它是所有程序共用的查找键，所以保留。`Alt+M` 只显示别的机器，但当一台机器被单独显示时，你输入的内容会发给那台机器的协调者：聊天里会说明。

## 按键与文本框

除 `Enter`、`Ctrl+S`、`Esc` 和 `Ctrl+C` 外，按键只有在文本框为空时才作用于看板。文本框里有草稿时，按键属于草稿（`Ctrl+A` 在那里是"行首"，不是"归档"）。

唯一例外是滚动聊天消息的两个键 `Alt+K` 和 `Alt+J`：它们只是多显示一些内容，所以输入时也能用，不动草稿。

## `?` 菜单

它列出与下方表格相同的三组，每行都带按键。`Ctrl` 或 `Alt` 的行也可以在那里选中，效果和按键一样。所以不传 `Alt` 的终端（没有"Option as Meta"的 Mac 终端、某些手机键盘）也不会损失什么。

在 Herdr 网页界面的终端视图里，`Alt` 键和 `Shift+Enter` 以旧的字节形式到达并按此处理，手机输入行在 `?`、`y`、`n` 或空格后补加的 Enter 会被丢弃（[网页界面](web-ui.md)）。

## 提问：不可撤销的操作之前看板会先问

只有合并和移除任务会问。别的都不问。

```
 Merge HSW-18 "Add an undo for clear" (PR#18) into main?
 A merge cannot be undone.
 y = merge · n = do not merge
```

- **`y` 表示同意。`n` 或 `Esc` 表示不同意**，大小写均可。`Enter` 从不确认，别的键也不作答：按其他任何键，对话框会显示 `press y or n`。
- **提问会说明它做什么。** 合并会列出任务名、落点，并说明不可撤销。对另一台机器的任务，会说明在那边落到哪里：`into main on otherbox`。评审失败后，会说明评审发现了什么，其同意项写作 `y = merge anyway`。
- **移除**会显示任务的完整标题和真正丢失的内容：任务连同它的描述与评审，以及它的工作树（仅有工作树时；其中未保存或未合并的工作会保留）。按键为：`y = remove · n = keep`。
- **显示位置：** 看板底部的小对话框，占文本框的位置，它的收尾行在窗口最后一行，就像聊天框压在提示上一样。比窗口长的提问会换行，按键从不被截断：窗口太短时先让细节让位，然后是规则。期间没有提示行：对话框占了那一行。
- 在网页看板上，合并或移除的两个回答都不是填充的按钮。

结果会留在屏幕上，直到你按下下一个键。错误会留在屏幕上，直到 `Esc` 关掉它，后面跟着 `(esc dismisses)`。

### 评审失败的工作不会被一次普通的 yes 合并

它不在 `ready to approve` 下，而在 `needs you` 下，显示为 `Review failed · 1 critical, 1 major`。它的提示先提供把评审发回去，然后才是 `merge this task anyway`。

防护在批准动作本身，不只是在提问里：`omh approve <id>`（以及协调者）会拒绝这类工作并说明评审发现了什么。`omh approve <id> --anyway` 照样合并。

## 关闭看板

**按两次 `Ctrl+C`**，和 Claude Code 一样。第一次在底行显示 `Press ctrl+c again to close the board`；在显示期间（两秒内）再按 `Ctrl+C` 即关闭，即使文本框里有草稿。按别的键或等待超时会撤掉提示，那个键照常做事。

在没有可后退的东西时 `Esc` 不关闭任何东西：它那一行会点名真正关闭的键（`ctrl+c closes the board`），在它显示时按 `Ctrl+C` 即关闭。在弹窗里，最后那次 `Esc` 会关闭看板，提示行也会说明（`esc close`）。

## 打开看板的键

它是 Herdr 的，不是看板的：写在 Herdr 的 `config.toml` 里的一行，[首次运行](install.md)时建议为 `prefix+a`。看板从不占用 Herdr 自己绑定的键（`prefix+w` 是 Herdr 的工作区选择器）或你的配置已绑定的键：它会拒绝，并说明该键今天做什么。

再按一次，打开看板的键带你回去：从它标签页里的看板，回到你来的面板；`board.open` 设为 `popup` 时，它关闭弹窗（[看板](board.md)）。Herdr 给弹窗每个键，所以在那里看板自己从 Herdr 的 `config.toml` 读那个键：Herdr 的 prefix 键加上那个键。

## 设置页掌管自己的键

任务的设置打开时（`Ctrl+E`），只有该页的键起作用：`↑` `↓`、`Enter`、`Ctrl+U`、`Esc`（或再按 `Ctrl+E`），以及用 `Ctrl+C` 关闭看板。列表里的任何键都不会在那里归档、合并、置顶或移除任务。提示行是该页自己的：`↑ ↓ move · enter change · ctrl+u put back · esc back to the task`，输入值时则是 `enter save this value · esc leave it as it is`。

## 提示行

底行只显示当前要紧的事，主要的在前，对选中任务最多四条提示。`?` 列出全部按键；提示行不是清单。

- **提示说明按键做什么、对谁做。** "这个任务"指选中的那个：`enter open this task's agent`、`tab put an agent on this task`、`ctrl+l approve and merge this task`。是谁的代理在行上说一次；之后就是 `the agent`（`space talk to the agent · ctrl+x stop the agent`）。同一动作在提示行和 `?` 菜单里措辞相同。
- **按键只在它做事的地方点名。** `esc` 会连同它从什么返回一起显示（`esc back to the list`、`esc clear the filter`、归档里的 `esc back to the board`），而不是在只演示看板如何关闭的地方。`↑↓ scroll the task` 只在整个任务比窗口长时显示在整任务上。`alt+c` 只在有消息可显示或隐藏时显示。`ctrl+x` 会说明它在这里做什么：`stop the agent`、`remove this task`，或者在你标记为自己工作的任务上，`put it back in to do`。
- **屏幕上已有的话不再重复。** 文本框自带的灰字说明谁会收到你的消息（`Send message to Agent`、`Send message to Coordinator`），所以提示行不说。
- **输入时**，它说明 `Enter` 把你的话发到哪里，并用标签点名任务，因为话锁定在该任务上，而列表可能显示别的：`enter send to the Coordinator`（别的机器是 `… on vm-a`）、`enter send to HSW-34's agent`、`enter send, and start an agent on HSW-34`、`enter add it as a task`。文本框的边缘不点名任何人。

它显示什么，例如：

| 位置 | 提示行 |
|---|---|
| 没人开始的任务 | `tab put an agent on this task · space talk about this task · ? all keys` |
| 工作中的代理 | `enter open this task's agent · space talk to the agent · ctrl+x stop the agent · ? all keys` |
| 代理问你事情 | `space answer this task's agent · enter open the agent · ? all keys` |
| 待批准 | `ctrl+l approve and merge this task · ctrl+r send the review to the agent · space talk to the agent · ? all keys` |
| 出了问题 | `ctrl+r tell this task's agent to fix it · ctrl+l retry as it is · space talk to the agent · ? all keys` |
| 未留小结就停止 | `ctrl+r tell this task's agent to continue · ctrl+l use its work as it is · enter open the agent · ? all keys` |
| 正在和任务的代理对话 | `enter open this task's agent · ctrl+x stop the agent · space back to the Coordinator` |
| 正在给协调者输入 | `enter send to the Coordinator · shift+enter new line · esc put away` |
| 整个任务 | `… · ↑↓ scroll the task · esc back to the list` |
| 归档 | `ctrl+a restore this task · ctrl+x remove this task · esc back to the board · ? all keys` |
| 一项自动化的行 | `enter open this automation · space turn this automation off · tab fire it now · ctrl+x remove it · ctrl+e its settings · ? all keys`（关闭时：`space turn this automation on`，没有 `tab`，它没什么可触发） |
| 单个自动化，整体 | `↑↓ pick a firing · enter go to its task · space turn this automation off · tab fire it now · esc back to the list`（`enter` 只在造出了任务的触发记录上有效） |
| 查找 | `enter keep the filter · esc cancel` |
| `?` 菜单 | `↑↓ pick a key · enter press it · esc close the menu` |
| 正在发往另一台机器的话 | `ctrl+x stop sending this · enter take the words back to change · ? all keys` |

**在窄窗口里提示行从不跑出屏幕：整条提示让位。**

- 第一条提示（主要的）和最后一条（`? all keys`，或返回的路）留下。
- 它们之间，`alt+c hide the latest message` 先让位，然后从最后往前让。`alt+c show the latest message` 比它们更持久，因为它是你看到回复的方式。
- 输入时，`shift+enter new line` 先让位，`esc put away` 留下。
- 只放得下一条提示时，就是主要的那条，除了你在和任务的代理对话时：那里 `space back to the Coordinator` 留下，唯一的返回路。
- 窄到连那一条也放不下时，它说得更短，而不是被截断：`enter send to HSW-34`、`ctrl+l merge this task`，用 `its agent` 代替 `this task's agent`。到大约 34 列都如此。草稿的提示只要放得下就继续点名收话人（`enter send to HSW-34`、`enter send to the Coordinator`、`enter send to vm-a`）；再窄才是 `enter send`。

重启协调者、启动新的协调者、进入它的完整聊天都不在看板上：见[协调者](coordinator.md)。

## 你自己的按键

**到不了看板的键可以挪到别的组合键上。** 有些窗口管理器和终端把 `Alt` 留给自己，而 `Alt` 键正是你常用的。用设置 `board.keys` 给键换上你自己的组合键：每个键按它在下方表格中的名字（名字在 `crates/flow/src/board/keys.rs` 里，写错时错误里也会给），然后 `=`，然后组合键。

```
omh set board.keys "chatMessage=ctrl+g, chatScrollUp=ctrl+up, chatScrollDown=ctrl+down"
```

- **该键此后只响应你的组合键。** 旧组合键什么都不做。`?` 菜单、提示行和看板消息都会显示你的组合键，网页看板也是。
- **已被占用的组合键会被拒绝，并告诉你是谁的键：** `chatMessage=ctrl+l: ctrl+l is yes's key already`。仍想用它，就在同一个值里把那个键也挪走（`undo=ctrl+c, close=ctrl+q`）。打开看板的键同样被拒绝；若该键以 Herdr 的 prefix 开头，那也不行。
- **普通字符只给本来就是单字符的键**（`talk`、`menu`、`sayYes`、`sayNo`）：别的会被打进文本框。文本框自己的键（`newline`）不能挪。
- **它只属于你**：从不按仓库区分，也绝不来自仓库自己的文件。
- `omh unset board.keys` 把每个键的组合键还回去。你在设置文件里手改出的错误不算数：那个键保留自己的组合键，`omh show` 会说明原因。

下方表格永远显示看板自带的组合键。

## Mac 上的键名

**在 Mac 上 `Alt` 键用 Option 按，看板可以照此显示。** 设 `board.keyStyle mac` 后，提示行、`?` 菜单和看板消息把 `alt+c` 写成 `opt+c`，菜单分组名写作 `opt: only shows or hides`。`Ctrl` 保持 `ctrl`：Mac 终端把 Control 传给看板，从不传 Cmd。只有名字变了；键是同样的。（终端须把 Option 当 `Alt` 发送：Terminal 里的 "Use Option as Meta key"，iTerm2 里的 "Esc+"。）

```
omh set board.keyStyle mac
```

`auto` 是内置值，按看板所运行的系统决定。从 Mac 通过 SSH 访问的 Linux 机器上的看板看不到你的键盘，所以那边显示 `alt`：在那台机器上设 `mac`。`pc` 永远显示 `alt`。下方表格永远用 PC 名称。

## 全部按键

<!-- made:keys Made by `omh docs` from crates/flow/src/board/keys.rs. Do not edit between these two lines. -->
**Plain keys**

| Key | Does |
|---|---|
| `↑` `↓` | move between tasks and their headings, and on to the automations under them |
| `←` `→` | on a heading (all machines, a machine, a repo, a status group): fold it to its one line, which says what it holds. On a task's or an automation's row: go to the heading it stands under. With words typed, ← and → move in them ([list-headings.md](list-headings.md)) |
| `Pgup` `Pgdn` | a page up or down: ten tasks in the list, a page of text in the whole task |
| `Tab` | the filter row on top (All · Coordinators · Needs you · Working · Ready · To do · Done · Automations · Pinned, each with its count, after whose board when there is another machine): `Tab` moves to the next, shift+tab to the one before, a click picks one. The list then shows only that kind of row, in its places: Coordinators leaves the machines' and projects' rows whose coordinators run (started and not stopped, on autopilot, or failed; the main Coordinator has no row: it is who the chat talks to); Pinned leaves what you pinned, a pinned machine or project with its rows under it ([board.md](board.md#过滤行), [pinned.md](pinned.md)) |
| `Enter` | send what you typed. With nothing typed, on a heading (all machines, a machine, a repo, a status group): fold it to its one line, which says what it holds, and unfold it; its caret says which it is, `▾` open, `▸` folded ([list-headings.md](list-headings.md)). On a task's row it does nothing: `Alt+O` goes into its agent, `Space` talks to it. On an automation's row: open the automation, whole ([automations.md](automations.md)) |
| `Space` | talk to the selected row's agent: a task's, or on a machine's or a project's row its coordinator; again, back to the top row's coordinator (All machines', or the one machine's). The chat names who, from the top row down. Nothing else changes it: while it is a row's agent, moving with `↑` `↓` makes it the agent of the row you moved to. On a task with no agent, your message puts one on it. On an automation's row: turn the automation on or off |
| `Esc` | back, one step at a time, and nothing else: find is left, a draft is put away, then the whole task, the filter and the archive close, then the chat's latest message is hidden, as `Alt+C` hides it. With none of those left it shows the same hint as `Ctrl+C`; the board as a popup (`board.open`) closes on it. It never shows the chat and never changes who you talk to |
| `?` | the menu: every key in these three groups, each line with its key. A ctrl or alt line can be picked with `↑` `↓` and `Enter`, which does what its key does. It opens with no line picked. Under the keys, what a row shows: each mark with its word, the numbers on the right, and what the colour of a review tag and of a pull request say. Every line is whole (a long one wraps); where the window is too short for all of it the last line says how many lines are below, and `↓` on the last key scrolls on down, `↑` on the first scrolls up |
| `y` `n` | answer a question the board asks: `y` yes, `n` or `Esc` no. Only while a question is up; `Enter` never confirms |

**`Ctrl`: does something**

| Key | Does |
|---|---|
| `Ctrl+L` | yes, to whatever the task waits for: approve the merge (it asks first), keep the selected one of several attempts (it asks first), say yes to the agent, retry the step that went wrong as it stands, or take work that stopped without a summary, or at a limit, as it is; end a chat-only task's conversation ([chat.md](chat.md)); on a task whose agent is at rest while its row still says "Finishing", finish it with what its agent said last |
| `Ctrl+R` | send it back: whatever went wrong (a failed check, a conflict, an error, a review with findings) goes to the task's agent to fix; a stopped agent is told to continue |
| `Ctrl+N` | on a task done several times at once ([attempts.md](attempts.md)): move to its next attempt, in the task panel. `Ctrl+L` keeps the selected one, `Enter` goes into its tab |
| `Ctrl+S` | put an agent on the task. Words for a coordinator go with `Enter` alone: it decides itself what to start, and starts what you ask it to. With no Coordinator on this machine, what you typed is added as a task with an agent on it at once. On an automation's row: fire the automation now. On a coordinator's row (a machine's or a project's row), with nothing typed: start that coordinator, in its own conversation, as an agent is put on a task (`Ctrl+O` is its autopilot); with no machine's, project's or task's row selected (a status group's heading, an empty list), the main Coordinator |
| `Ctrl+X` | stop the agent, when one is running (working, starting, checking, under review, or waiting for your answer); the selection stays where you were, on the next row when the stopped task moves to another group. On a task with no agent running (idle, never started, stopped, finished, archived) it asks to remove the task instead: `y` removes it, its worktree too. A task you marked as your own work in progress has no agent to stop: there the key puts it back in to do. A stop never goes on to that question by itself: removing takes a second `Ctrl+X`, and a `y`. On an automation's row: remove the automation (it asks first; the tasks it made stay). On a coordinator's row: stop that coordinator (with no machine's, project's or task's row selected: the main Coordinator); your next message, or `Ctrl+S`, starts it again in the same conversation |
| `Ctrl+O` | on a coordinator's row (a machine's or a project's row): its [autopilot](autopilot.md) on, with its saved goal and settings, and off again; on one at `needs you` or `could not start` it starts again. With no machine's, project's or task's row selected (a status group's heading, an empty list): the main Coordinator's. On, its row keeps its mark and has `[autopilot]` after its name and a line under it that says what autopilot does |
| `Ctrl+A` | archive the selected task, or restore it from the archive. Archiving closes its space (its agent ends) and frees its temporary files; its worktree, branch and history stay. Restoring starts nothing. On a project's row (a repo's heading, grouped by repo, `Alt+S`): archive the whole project with its tasks, after a yes when one of them works, waits or is to do; in the archive, on an archived project's heading: restore it with its tasks as they were |
| `Ctrl+P` | pin the selected row, or unpin it: a task, a project's row or a machine's row (here or on another machine). Nothing moves: the row gets a yellow pin mark `★` and shows under Pinned, the last filter on the filter row (`Tab`) ([pinned.md](pinned.md)) |
| `Ctrl+D` | open the selected task's link in the browser: its report when it has one ([research.md](research.md#报告放在哪里)), else its pull request. It opens with this machine's own opener (`open` on macOS, `xdg-open` on Linux with a display). Where nothing can open it (no opener, ssh without a display) it copies the link instead (OSC 52, as the board's copy does, [mouse.md](mouse.md)) and says so. On the web board the page opens it. On a task with neither it does nothing |
| `Ctrl+E` `Ctrl+U` | the selected task's settings, full screen: every work setting by its name and its value. `↑` `↓` move to a row; `Enter` changes it (a switch or a choice goes to its next value, anything else is typed and saved with `Enter`); rows set for this task only are marked `*` (where any other value comes from is `omh show <id>`'s). `Ctrl+U` puts the selected row back to what it would be without this task's own answer. `Esc` or `Ctrl+E` again goes back ([settings.md](settings.md)). On an automation's row: the automation's own settings, the same page. On a coordinator's row (a heading stands for its coordinator): that coordinator's own settings first (`coordinator.*` and `autopilot.*`; while its autopilot is on, an autopilot row is saved for this time), then on this machine's heading your own settings, which every task starts from (on a repo's heading: your entry for that repo), changed as `omh set` changes them; another machine's coordinator is shown whole instead, as its settings are changed there |
| `Ctrl+Z` | undo: bring back a draft that was put away, or take the last archiving back |
| `Ctrl+F` | find: type, and the list shows only the tasks that match, archived ones too. `Enter` keeps the filter, `Esc` leaves find; `Esc` on the list clears a filter that is kept |
| `Ctrl+K` | only while a remote view holds the board's terminal at another size than herdr shows it (the line at the top says so): end that view, so the board has its own size again ([board-size.md](board-size.md)) |
| `Ctrl+C` | twice: close the board, with text typed too: the draft is kept and is back when the board reopens |
| `Ctrl+T` | a voice call with whoever the chat talks to: a live conversation with the board's voice agent, which sends what you ask for to the Coordinator, the task's agent or a coordinator, and reads their answers back in short. The call follows the chat: `Space` and moving between tasks change who it talks to. The microphone and the speaker are the computer you sit at: its call helper holds the call there, in the terminal's computer, with no browser. The key again hangs up. With `voice.calls off` it only says that calls are off. More: [voice calls](voice.md) |

**`Alt`: only shows or hides**

| Key | Shows or hides |
|---|---|
| `Alt+C` | the chat's latest message, the Coordinator's or a task agent's. The one key for it, both ways |
| `Alt+K` `Alt+J` | more of a chat message that is longer than its box: `Alt+K` scrolls it up, `Alt+J` down, half a box at a time, also while you type. Up opens the box to the conversation before the newest message, each press further up; it stays open, whatever comes meanwhile, until `Alt+J` at its end goes back to the newest message (`Alt+C` hides and shows the whole chat; talking to another starts it at the newest too). The box says where there is more, and names the key (`alt+k more ↑`, `alt+j more ↓`). The mouse wheel does the same with the pointer over the chat ([mouse.md](mouse.md)) |
| `Alt+P` | the side panel: the selected task, automation or coordinator (its row: its chat, and its autopilot while that is on), beside the list |
| `Alt+T` | the whole task (on an automation's row: the whole automation; on a coordinator's row: the coordinator, with its last wakes while its autopilot is on), full screen, and back to the list |
| `Alt+A` | the archive, and back to the board |
| `Alt+S` | how the list is grouped, in turn: by status, by repo (each group under its repo's name; a row's tag never names a repo), by machine (each machine under its own heading, its tasks under it by how much they need you) |
| `Alt+D` | every done task, or none: the list then counts them on one line |
| `Alt+O` | go into the selected row's agent: its real herdr pane. On a task's row its agent; on a machine's or a project's row (or All machines) its coordinator, started first when it is not running, on another machine there. The board keeps running in its own tab and is there as you left it; your key goes back to it (as a popup it closes, and your key opens it again where it was). On a status group's heading there is none ([machine-and-project-coordinators.md](machine-and-project-coordinators.md)) |
| `Alt+M` | whose board shows, the filter row's first item: every machine, then each machine by itself (this one too) in the order of herdr's sidebar |

**The text box's own** (not a key of the board, and not in the `?` menu)

| Key | Does |
|---|---|
| `→` | on a folded heading: unfold it |
| `Shift+Enter` | a new line in what you are typing. Like the cursor keys and backspace, it is the text box's own editing key, not a key of the board |
<!-- /made:keys -->
