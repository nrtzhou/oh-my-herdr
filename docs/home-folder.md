# 主目录

**看板为你保存的一切都在一个目录 `~/.omh/` 里，每台机器都一样。**

```
~/.omh/
  settings/     config.json: your own preferences (docs/settings.md)
  data/         everything the board writes while it runs
    tasks/<repo>/<n>/        one folder per task: task.json, inbox.jsonl, closing-note.md, artifacts/
    coordinator/<session>/   the Coordinator's saved state, and chat/, the folder it starts in
    automations/             the automations, their firings, the board events counted already (docs/automations.md)
    bin/omh      the command agents are handed, by its full path
    conflicts.jsonl          one line per conflict with main, once repaired (docs/how-work-gets-done.md)
    locks/ stamps/ *.json *.log   the board's own bookkeeping
  reports/      pages and reports an agent made for you that belong in no repo, one folder per task
    old-suggestions.md       the suggested tasks that still waited for an answer when they were removed from the board
  moved.json    written by the one-time move below: what was moved, from where
  set-aside/    only after a move that found a file in both places: the copy that was not kept in use
```

## 换个位置放

| 要移动 | 设置 |
|---|---|
| 整个目录 | `OMH_HOME` |
| 只移数据 | `OMH_DATA_DIR` |
| 只移设置 | `OMH_CONFIG_DIR` |
| 只移报告 | `OMH_REPORTS_DIR` |
| 只移协调者的启动目录 | `OMH_COORDINATOR_DIR` |

看板启动的东西（代理的标签页、评审者的标签页、发给代理的命令）都带着 `OMH_HOME`，只有某个部分放在主目录之外时才带它自己的变量。所以在一条命令前把 `OMH_HOME` 设成临时目录，就整体搬家：碰不到真正的看板。

`XDG_*` 变量（`XDG_STATE_HOME`、`XDG_CONFIG_HOME`、`XDG_DATA_HOME`）不再决定位置。它们只剩一个用途：找旧版本把文件放哪了，供下面的搬家用。

## 看板的东西不进代理工作目录

代理为看板写的东西放在它任务自己的目录 `data/tasks/<repo>/<n>/` 里：

- 它的收尾小结（closing note，`closing-note.md`）；
- 评审进行中，评审者的结论和证据（`review/`，评审结束移到 `artifacts/`）。

代理会拿到完整路径。所以任何仓库或工作树都不会多出看板的目录，在你主目录里干活的任务也不往 `~/.omh/` 本身写任何东西。

收尾小结在任务清理时删除（合并后、以无东西可合并告终的任务、或 `clear`）。任务被清掉时整个任务目录一起消失。

`.omh.json`（仓库自己的设置）是另一个文件。

### 旧版本的残留

旧版本让代理往工作目录里的 `.herdr-superwork/` 写东西（git 会忽略）：`closing-<n>.md` 和 `review-<n>/`。对在主目录里干活的任务，那个位置就是 `~/.herdr-superwork/` 本身，`data` 和 `settings` 旁边的 `closing-1.md` 就是这么来的。现在：

- 用旧规则启动的代理继续干活。看板仍会从旧位置读那个任务的收尾小结（两处都有时取较新的），直到该任务结束。本版启动的任务绝不去那里找。
- 这种任务被清理时，它在旧位置的收尾小结一并删除；旧目录因此空了的话也删掉。
- 旧 `.herdr-superwork/` 目录里的其他东西一律不动（本版启动的任务不从里面删任何东西，无论文件叫什么），非空目录也不动。确认搬家前启动的代理都退出后，自己删掉它。它在仓库 `.git/info/exclude` 里的那一行保留，残留永远不会显示为改动。

## 留在它外面的东西

- **how-to 文件**在每个代理程序自己的技能目录里（`~/.claude/skills/omh` 等），**标记块**在 Herdr 的 `config.toml` 里。它们必须放在那些程序读取的位置。`omh remove-extras` 把它们取走（[安装](install.md)）。
- **Web UI 的共享链接。**装了 Web UI（Ovis）且你同意后才会动：改的是那个 Web UI 的共享链接（`tailscale serve`）指向哪里，好让看板显示在里面。`omh web-ui unlink` 和 `remove-extras` 把它指回去（[Web UI（Ovis）里的看板](web-ui.md)）。
- **仓库的信任标记**，在代理程序自己的设置里（`~/.claude.json`、`~/.codex/config.toml`；pi 的在 `~/.pi/agent/trust.json`，按任务的工作树记）。
- **本仓库自己的测试运行器**把已通过的结果放在 `~/.cache/omh-check`（[测试](testing.md)）。那是给这份代码干活用的工具缓存，不是看板为你保存的东西：随时可删。

## 从 herdr-superwork 或 omh 0.2 迁来

`~/.herdr-superwork` 变成本目录。旧 omh 0.2 的项目本来就放这个名字下，会先搁置到 `~/.omh-legacy`：[旧安装](old-installs.md)。

## 从旧版本迁来：一次性搬家

旧版本把数据放在 `~/.local/state/herdr-superwork`，设置在 `~/.config/herdr-superwork`，代理的报告在 `~/.local/share/herdr-superwork-reports`。**新版本首次运行会把三者都搬进 `~/.omh/`**：任何一次运行都算，看板、命令、或 Herdr 调用的钩子。然后在你第一次会看到的地方（看板打开时，或你敲过命令后）用一行说明：

    Moved the board's files into ~/.omh: data (from ~/.local/state/herdr-superwork), settings (from
    ~/.config/herdr-superwork), reports (from ~/.local/share/herdr-superwork-reports). The old places are links to
    the new ones, so whatever is running carries on.

（含义：看板的文件已搬进 `~/.omh`，数据、设置、报告各从旧位置搬来；旧位置变成指向新位置的链接，运行中的东西照常继续。）

### 为什么安全

- **不复制、不删除，旧路径一刻都不缺。**旧路径先是目录本身，然后一步变成指向新位置的链接。每个目录走三步，每步要么整体完成要么没做：
  1. 建两个备用链接（`<old>.moving` 和新位置）；
  2. 旧目录和备用链接互换；
  3. 新位置和备用名字互换。

  「互换」是一次系统调用（Linux `renameat2`，Mac 的 `renamex_np`），所以任何时刻从旧路径启动的命令都找得到自己的文件。中断的运行由下一次从它留下的形态接上，第二次运行无事可做。`moved.json` 在第一步之前写好。
- **旧位置变成指向新位置的链接。**还握着旧路径的一切照常在同一份文件上工作：还在跑旧代码的看板或看守、发给代理的命令路径（`~/.local/state/herdr-superwork/bin/herdr-superwork`）、运行中代理的钩子设置、任务存的产物、页面服务器。锁在哪条路径上都是同一个文件，新旧代码照样轮流拿。搬家前启动的东西都退出后，链接可以删；看板不动它们。
- **新位置已有的数据绝不被覆盖。**新位置里的东西先放进旧目录（在用的那个），然后按上面搬。只有一边有的都保留。两边都有的文件，较新的继续用，另一份进 `set-aside/`，说明里会写有多少个。
- **不从任何进程手里抢锁。**两处有同一个锁文件时，旧位置（在用的位置）那个就是锁，绝不替换或删除：握着它、或已打开即将拿它的进程，事后仍然握着唯一的真锁。新位置那份只在没人握时才走；有人握时那个目录这次不搬，下次运行再搬。看板拿到锁后还会检查手里这个是否仍是该路径上的文件，不是的话拿路径上那个。
- **运行绝不在空的新位置上开始。**运行开始时还没搬完的东西（另一个运行搬了三秒还没完、某步失败、新位置的锁被人拿着）这次就在原地、旧位置用。
- **两个路径无法一步互换**（没有那个系统调用的系统），**或旧位置在另一块盘上**时，目录留在原地，新位置变成指向它的链接。还是一个目录，旧路径还是不缺，说明里会写。
- **你自己指定的目录不动。**`OMH_DATA_DIR`（或设置、报告那个）指向你自己的位置时，那部分不搬。
- **只有看板运行的那份才搬家。**这份代码的第二个检出（代理在干的 git 工作树、测试中的副本）不动文件，就地对着现有看板干活。

### 协调者的对话

它的目录随数据一起搬（`data/coordinator/<session>/chat`），运行中的协调者留在里面。下次重启后还是同一场对话：看板按 id 重新打开它，三个程序都能从搬过的目录里按 id 找到对话。（用 Claude Code 2.1.296 真实验证过，它按目录给保存的对话做键：`--resume` 找到了并在里面继续。Codex 和 pi 按日期和 id 保存。）

搬家前启动的程序仍按旧路径认识它的目录，并把新聊天（`/clear`、`/new`）存到那下面。看板也去那里找，所以照常跟得上。

### 撤销

停掉看板，删掉三个链接，把 `data`、`settings`、`reports` 移回旧位置，跑旧版本。别的什么都没改。
