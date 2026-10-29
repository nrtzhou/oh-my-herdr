# 磁盘空间

**任务自己收拾自己。**它的代理和评审者（reviewer）造的临时东西全进任务自己的一个目录，任务结束时那个目录消失。更早结束的任务留下的东西会被自动清扫，本会在快满的磁盘上启动的任务改为等待，而不是构建到一半失败。

## 任务自己的临时目录

每个任务拿到一个短的专属目录，比如 `/tmp/hp-1000/3f9a2c/HP-96`（你的用户 id，然后每看板一个目录，再是任务名）。它的代理和每个评审者都以它为 `TMPDIR`（以及 `TMP`、`TEMP`、`CLAUDE_CODE_TMPDIR`）启动，所以他们建的测试项目、预览、venv、冒烟安装都落在那里，他们自己工具存的东西也是：Claude Code 的 scratchpad 就在里面。规则里会告诉他们这件事，并让他们收尾前停掉自己起的服务器（除非你让他们留一个跑着）。

## 任务结束时

完成、归档或移除后，代理连同它的空间（space）一消失：

1. 临时目录里还在跑的东西（代理或评审者在那儿起的开发服务器）被停掉；
2. 删掉临时目录，还有代理程序为工作树（worktree）在它外面留的目录（本次之前启动的代理的 Claude Code `/tmp/claude-<uid>/<工作树路径、斜杠换破折号>`）；
3. 它的行说明这释放了多少：`Freed 1.4 GB of temporary files.`

保留的工作树（有未保存或未合并的工作，或 `workspace.cleanup` 关着）原样不动：只清临时文件。完成但代理还开着的任务（`workspace.cleanup` 关着）也照样清临时文件，只留程序给它自己的目录（Claude Code 的 scratchpad 目录，清空），这样你再跟它说话它还在；那之后它造的东西，没人跟它说话一小时后进清扫。代理还在干活时，什么都不碰。

### 它留下还在跑的东西

完成或移除的任务，其工作树消失前一刻，从那个工作树或临时目录里还在跑的东西会被按 `stop` 的规则（[角色与命令](roles-and-commands.md)）结束：代理为做视觉检查起的开发服务器或 mock 网关，包括离开代理终端的（交给 init，所以看不出空间被占），以及代理在其终端里启动的东西。绝不碰它别的面板里的（你的、它的评审者的）、看板自己跑的、你检出里的任何东西；herdr 不报告它面板里跑着什么时，它目录里的东西也不碰。它的行说明收了什么，并点名还跑着、要你自己结束的：不是看板能收的、收不掉的、为该任务起的但现在在别处跑的，以及 herdr 不报告时、从它目录里跑的：

```
Merged into main. Ended what it started: node …/vite.js (pid 81234). Still running, end it yourself: node …/mock.js
(pid 81240), in /home/me/wt/proj/work-12-x, from another of its panes.
```

`omh details <id>` 在 `stillRunning` 下列出还跑着的东西。移除任务（`clear`）的回答里同样说明。保留的工作树不结束任何东西。归档保留一切，所以也不结束任何东西：它只点名保留的工作树里还跑着的（`Still running, end it yourself: node …/vite.js (pid 81234), in …`）。

## 清扫

清扫移除更早结束的任务留下的东西：看板打开时跑（上次清扫超过一小时）、每天自动一次、在快满的磁盘上启动之前、以及你运行 `omh sweep` 时（它说明删了什么；`omh sweep --dry` 只说会删什么）。它要移的目录里还跑着的东西（比如留下的开发服务器）先被停掉。它收：

- 完成和归档任务的临时目录，其代理已消失（或开着但安静了一小时，见上）；
- 代理程序为已消失的看板工作树留的目录；
- 任务已被移除的看板任务目录；
- 已完成或已归档任务保留的工作树的构建产物（`target/`、`node_modules/`、`.next/`，且只收 git 忽略的），条件是任务上 `board.oldBuilds` 天（7；`0`：从不）没有任何变化；
- `~/.cache/omh` 里没人用的旧热构建。

**它绝不碰：**干活中的任务、它的工作树和它的目录；你的仓库和主目录；看板没建的 `/tmp` 散目录；最近一小时内变过的目录；你的环境指向的目录（旧 scratchpad 里的 `PATH` 条目会保住它）；以及经链接到达的任何东西：一个目录要它自己和它上面的每一层（直到临时目录、构建缓存或它所在的工作树目录）都是真目录才会被收。临时目录位置上的链接被移走，它指向的东西和里面跑的东西不动。代理为工作树建的目录，只有名字是已知工作树目录加任务分支式的名字（`work-`开头）、那里没有同名目录、旁边没有别的目录以它开头时，才算看板的。上次清扫删了什么记在 `~/.omh/data/sweep.json`。

## 磁盘快满

任务启动前（工作树、环境准备、代理，以及计划获批后的构建），看板会看它要写的盘：仓库的、工作树的、看板数据的、临时文件的。剩余空间低于 `board.diskFree` GB（5；`0`：不检查）时先清扫；仍然不够，任务回到队列并给一句大白话，而不是失败报 "No space left on device"：

```
Disk nearly full on vm-str: 778 MB free on /; this task starts once 5.0 GB is free (board.diskFree). Free some space:
omh sweep, and omh doctor lists more that can go.
```

空间够了它自己启动。`omh machines` 说每台机器剩多少，`omh doctor` 列出每个盘和能清的遗留物。

### 小磁盘

磁盘不必留出自己一半以上的空间。小内存盘式的 `/tmp`（总共 3.4 GB；很多 Linux 机器给 `/tmp` 一半内存）永远拿不出 5 GB 空闲，任务会永远等下去：在它上面，任务只需 1.7 GB 空闲，它的行会说明为什么少于 `board.diskFree`：

```
Disk nearly full on vm-str: 1.2 GB free on /tmp; this task starts once 1.7 GB is free (/tmp is only 3.4 GB in all, so
it needs 1.7 GB free, not the 5.0 GB of board.diskFree). Free some space: omh sweep, …
```

`omh doctor` 用同样的方式衡量这种盘，两者永远一致。

### 等待中的任务怎么说

等空间的任务保持它在队列里的位置，并说明它在等磁盘空间，不是在等空闲名额：

- 看板和 Web 看板上它的行：名字下面是整句 `Disk nearly full on …`，右边是它的位置（`Q#1`）；
- 任务面板的 State 行、`list`、`details`、以及你启动它时的那行：`waiting for disk space, Q#1`；
- Herdr 的侧栏（sidebar）：`waiting for disk space`，带位置；
- `omh doctor` 在磁盘旁边：`Only 1.2 GB free on /tmp, which holds temporary files: HP-12 waits for room on /tmp, and new tasks wait too`。

等 `board.agents` 名额的任务仍显示 `waiting for a free slot, Q#2`。
