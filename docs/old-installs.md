# 旧安装：herdr-superwork 与 omh 0.2

**你什么都不用做。**本版首次运行会自动接管这两个旧程序，并用一行说明做了什么（终端上，或下次打开看板时）。不删任何东西。

- `omh doctor` 随时复查，接管剩下的，并汇报现状。
- `omh install` 结尾输出同样的说明。
- 再跑一次两者任何一个，都不会有变化。

「首次运行」指任何一次运行：看板、一条命令、或 Herdr 调用的钩子。

| 旧程序 | 它的下场 |
|---|---|
| **herdr-superwork**（本版的工作代号） | 迁入 `~/.omh` 继续用：任务、键（key）、历史都保留 |
| **omh 0.2**（旧的 Rust 程序，现在在 `legacy` 分支） | 搁置到 `-legacy` 目录，绝不混入 |

## 它们留下的东西

两者占用了本版要用的名字：

- **herdr-superwork** 的一切都在 `~/.herdr-superwork/`（任务、设置、报告、协调者的聊天）。它在 Herdr 配置里写 `# >>> herdr-superwork …` 块，在每个代理程序的技能（skills）目录里写 `herdr-superwork` how-to，还跑着自己的看守（keeper）和页面服务器（带 Web UI 前门）。
- **omh 0.2** 的项目在 `~/.omh/`（本版的主目录），设置在 `~/.config/omh/`，插件的文件在 Herdr 的状态目录里。它改过 Herdr 配置、代理程序的设置（钩子）和它们的技能目录。

## omh 0.2：搁置，不混，不删

- **它的项目。**`~/.omh` 里若装着旧版的项目（带 `PROJECT.md` 的目录）或它自己的文件（`.ticker.log`、`.machines.json` 等），整个原样改名为 `~/.omh-legacy`。若本版的文件也在里面，只移旧版的进那个目录。
  - 先按它自己的方式（放一个 `.ticker.stop` 文件）请它的 ticker 停止，再用信号。它还在跑时不动任何东西，下次运行再试。
  - 旧版用 `OMH_ROOT=~/.omh-legacy` 就能在那里找到它的项目。
- **它的设置**（`~/.config/omh`）移到 `~/.config/omh-legacy`。它在 Herdr 状态目录里的位置（`~/.local/state/herdr/plugins/omh`，本版按 id 共用）移到 `…/omh-legacy`。
- **只取走会冲突的**，其余每一行原样保留：
  - Herdr 配置里：它的按键（`omh.open-popup` 动作）、它在标签栏里的行（`… needs-you --line`）和它的侧栏（sidebar）行（`$hp_…` 标记；它用一行顶掉 Herdr 两行的地方，恢复 Herdr 的两行）；
  - Claude、Codex、Droid、Gemini 和 Copilot 设置里的钩子（`… omh --root … hook --agent …`），每个文件先备份（`<file>.bak-omh`）；
  - 它的 `autoproject` 技能链接；
  - 它放进你 PATH 的 `omh` 链接（当它指向 Herdr 插件而不是本程序时）。
- `OMH_ROOT` 只有旧版读。设置了的话 `doctor` 会说明。

## herdr-superwork：迁入 ~/.omh，继续用

- **目录。**`~/.herdr-superwork` 用与主目录一次性搬家（[主目录](home-folder.md)）相同的三个完整步骤变成 `~/.omh`：任何时刻都至少有一个路径可用，中断的运行由下一次接上完成。
  - `~/.herdr-superwork` 留作指向新目录的链接，让还握着旧路径的代理、钩子、页面继续工作。搬家前启动的东西都退出后再删它。
  - 已经有本版文件的 `~/.omh` 按那次搬家的方式合并：两边都有的文件用较新的，另一份进 `set-aside/`。
  - 旧的 0.2 项目先搁置：两者绝不混。
- **它里面的路径。**看板读回的内容（`data/` 和 `settings/` 里的 `.json`、`.jsonl`、`.md` 文件）中，所有指向 `~/.herdr-superwork` 的路径变成 `~/.omh` 下的同路径，发给代理的命令（`…/data/bin/herdr-superwork`）换成新的（`…/data/bin/omh`）。
  - 任务 id 里的仓库名（`herdr-superwork/218`）是名字不是路径，保留。任务、键（`HSW-218`）、历史原样继承。
  - 原样保留：任务的产物、报告、协调者的聊天目录、旧版本留的副本（`data-0.2.0`）、早期搬家的记录。
  - 旧启动器 `data/bin/herdr-superwork` 现在运行本程序，供还在用它的代理。
- **协调者的对话**延续：下次重启后按 id 在搬过的目录里找到，和首次搬家后一样。
- **它的进程。**停掉它的看守和页面服务器（带 Web UI 前门），但只在它们跑的是 herdr-superwork 程序时；然后启动本版的。前门开在旧版原来的端口上，Web UI 的共享链接不用改就指向新的。搬过家的链接一并跟过来（[Web UI（Ovis）里的看板](web-ui.md)）。
- **它的插件**：Herdr 里若还装着 `herdr-superwork`，会被关掉（`herdr plugin disable`，不移除），这样它的钩子不会再在搬过家的看板上跑旧程序。
- **它的附加内容**归本版：
  - 它在 Herdr 配置里的标记块（`# >>> herdr-superwork key`、`… sidebar`）换成本版的，按键不变；
  - 绑定 `herdr-superwork.<action>` 的地方改指 `omh.<action>`（同一个动作）；
  - 删掉它的 how-to 目录，写入本版的；
  - Herdr 为该插件准备的配置和状态目录归入 `omh` 的。
  - Herdr 的配置只有在 `herdr config check` 在副本上认可结果后才改。先备份，Herdr 拒绝就还原；`doctor` 会说明并重试。
- **仓库自己的设置文件。**只有 `.herdr-superwork.json` 的仓库保留它的设置：在该仓库有 `.omh.json` 之前，看板读那个文件。

## 只有看板运行的那份才动手

和一次性搬家一样，这些都不会由 git 工作树里的程序副本（正在改这份代码的代理）、测试、或 `help`、`version`、`docs` 来做。

## 撤销

1. 停掉看板。
2. omh 0.2：把 `-legacy` 目录改回原名。
3. herdr-superwork：删掉链接 `~/.herdr-superwork`，把 `~/.omh` 改回那个名字，再把 `~/.omh` 做成指向它的链接（里面写好的路径现在叫 `~/.omh`）。
4. Herdr 配置和代理设置的备份就在它们旁边（`.bak-omh`）。
5. `herdr plugin enable herdr-superwork` 把它的插件重新打开。
