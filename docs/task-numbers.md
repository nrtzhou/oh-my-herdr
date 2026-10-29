# 任务编号和仓库 key

**每个任务都用它仓库的短 key 加编号称呼：`HSW-134`。** 这是每行、每条消息里的名字，绝不光写 `#134`。它的完整 id，`omh/134`（仓库名加编号），在任何地方都仍然可用，且永不改变。

- 看板第一次见到每个仓库时就给它一个 key，从名字生成（`gtm-skills` → `SKL`）。
- 用 `omh set repo.key HSW --repo omh` 改 key。
- 在任何接受 id 的地方输入 `HSW-134`（大小写不限）：`omh delegate HSW-134`。

## 行上的编号标签

```
 ready to approve
 ! HSW-98  Add an undo for clear                                       R#2 PR#18 12m ago
           Fail · 1 critical, 1 major
 working
 ◐ HSW-134 Tidy the first-run questions               ███░░░░░ 40% 12m  R#1 PR#17  5m
           Running the tests
 to do
 ○ HSW-9   Rename the hint line                                                   2h ago
```

- **位置：** 状态标记右边、标题左边：`HSW-134`，仓库的 key 加编号。
- **安静：** 用主题里给灰色词的灰色，每行每种状态都一样，好让标题是你先读到的。
- **一列，好让每个标题从同一个地方开始。** 所有标签共用一列，宽度是列表里最宽标签的宽度（包括所有任务、折叠的也算，这样列表折叠时它不动）。短一些的标签用空格补齐。所以即使 id 长短不一，标题也对齐：`HP-1` 与 `HSW-206` 对齐。标签从左往右写：

  ```
   ○ HP-1      Tidy the first-run questions
   ○ CRM-2     Rename the hint line
   ○ HSW-206   Add an undo for clear
   ○ HSWXY-140 A task with a long key and a long number
  ```

  活着的任务在第二行说的话，从它自己的标题下方开始，不在标签下方。
- **不是右边那些标签。** `R#2` 是评审轮次，`PR#18` 是 PR：它们贴着右边缘，用评审和检查结果对应的颜色。任务的编号标签在左边，灰色，没有 `#`：key 和编号用短横连接。
- **一个列表里多个仓库：** 靠 key 区分。在每个列表里，编号标签都是仓库的 key 加编号（`PW-12`），所以两个仓库里编号同为 1 的任务显示 `PW-1` 和 `HSW-1`。
  要把同一仓库的任务放在一起看，按仓库分组（`Alt+S`，在 `?` 菜单里是「group by repo: see which repo each task is in」；web 看板上的「Group by repo」按钮）：每组的标题命名它的仓库。整个任务也会命名仓库，在它的 `Task id` 行（`pi-work/12`），带会话。
- **其他机器。** 另一台机器的任务带着那台机器给它的编号标签（那台机器的 key）。每台机器数自己的任务，所以两台机器可以各有一个 `#1`。显示所有机器时，每台机器是一个自己的标题，任务在它下面（[列表标题](list-headings.md)），所以行上是光秃秃的编号标签（`HSW-1`），绝不带机器名。上面没有标题的地方（看板的消息、聊天、整个任务），它命名为 `otherbox:HSW-1`。查找只认标签本身，不带机器。（还在旧版本上的机器的任务，其仓库在那里没有 key，按那台机器给的编号显示：`#1`、`otherbox#1`。）
- **空间不够时**，编号标签是名字其余部分之前最后一个让路的：在行说的话、进度条、`ago` 这个词、长名字的结尾、百分比加剩余分钟、`R#` 标签加 PR、以及时间之后（[看板](board.md)有完整的顺序）。它对整个列表一次性让路，好让标题留在一列里：当最宽标签的行给名字剩下的不到 16 格、旁边什么都没有时（大约 25 列以下的列表）。因为这一列从每行拿走几格，其他东西各自都比原来早让出那么多列。
- **窄屏上**（80 列以下），活的行的名字会折行时，名字的行、任务说的话的行、进度条的行都从标题下方开始：一行只有一个左边缘。

## 其他称呼任务的地方

| 位置 | 显示什么 |
|---|---|
| 列表、归档、查找：每个组的一、二、三行行式 | 编号标签，同上 |
| 任务面板和详情页 | 标题前的编号标签，灰色；标题写着 `Task details  HSW-134 "它的标题"`。id 本身（`omh/134`）在「整个任务」里只出现一次，作为 `Task id` 带会话：命令需要它的地方 |
| 聊天里点名你在和谁说话的那行 | 标记和任务名之间的编号标签；窗口太窄、给状态留不下 20 格时就没有它 |
| 自动化的触发 | `made HSW-140`，看板上和命令行上 |
| 看板自己的消息 | 编号标签加标题，一种形式：`Archived HSW-134 "Fix the footer".`、`Pinned HSW-14 "…" to the top.`、`Stopped HSW-14 "…".`、`Removed …`。动作用 id 写的消息也按编号标签读出（`HSW-14: approved, merging`）。只有显示待运行命令的行保留命令接受的 id。固定会说明，在另一台机器上的动作说明它在那边做什么（`Pinning HSW-2 on otherbox…`），绝不光一个 `…` |
| Herdr 的侧栏、面板标题 | 任务空间用编号标签加任务标题命名（`HSW-34 Add a file…`），这是它的代理条目的第一行（[Herdr 的侧栏](sidebar.md)）。面板标题一样，所以 web UI 的侧栏也读作 `HSW-34 Add a file…`。评审者的面板同样命名 |
| web 看板 | 每行同样的编号标签，同样的列、标题前同样有空间限制。任务说的话和进度条从标题下方开始，不在标签下方。列表太窄、给最宽标签的标题留不下 16 个字母时，所有行都不显示编号标签。任务打开时编号标签在任务标题前，聊天那行里也是 |

### 仓库名旁边的 key

看板命名仓库的地方，它的 key 立在名字旁边，变暗：`gtm-agent · AGT`。也就是：

- 看板的顶行，命名新任务去往的仓库（`omh  gtm-agent · AGT · 3 to do`，web 看板上也是）；
- 按仓库分组时列表里仓库的标题（web 看板上也是）；
- 「整个任务」的 `Task id` 行（`gtm-agent/12 · AGT`），以及自动化的 `Repo` 行。

绝不在任务的行上，它开头已经是 key 了（`AGT-12`）。窄屏上标题在尾部截断，所以 key 先走、名字留下。

### 短形式还是 id

所有用文字称呼任务的地方都用短形式，处处一样：

| 位置 | key 为 `HSW` 时 |
|---|---|
| 命令的回答和报错（`describe`、`delegate`、`archive`、…） | `HSW-134: updated (description)` |
| 看板的消息，以及命令通过看板回答的 | `HSW-135: restored` |
| 任务的代理被告知关于它自己任务的内容 | `its id on the work board is omh/134; the human also calls it HSW-134, and every command takes that name too` |
| PR 的正文、看板做的提交 | `Task: HSW-134 (omh/134)` |

id 本身留在程序读取的地方，因为它才是任务真正的名字、永不改变的那个：`--json`、分支名、文件夹和路径（路径里的 id 绝不重写），以及给代理运行的命令。（`omh list` 的第一列是短名，对齐显示。）

## 仓库怎么拿到 key

看板知道的每个仓库和文件夹都有一个 key：git 仓库、普通文件夹、主文件夹、`/`。不在仓库里的任务属于它被添加时所在的文件夹，所以也有那个文件夹的 key（主文件夹里添加的任务是 `HOME-3`）。

**从名字生成，立刻。** 看板第一次见到一个仓库或文件夹的那一刻（在那里加了任务、在那里打开看板、协调者在那里归档了任务），它从名字生成一个 key，存为你对该仓库的条目，在 `~/.omh/settings/config.json`：

```json
{ "repos": { "gtm-skills": { "repo": { "key": "SKL" } } } }
```

| 名字 | key | 规则 |
|---|---|---|
| `team-paperwork` | `TPW` | 三个词或更多（长词在该断词处断开：`paper` `work`）：取各自的首字母 |
| `gtm-skills`、`open-crm` | `SKL`、`CRM` | 两个词：取后一个，它说明这是什么；短词原样取，长词取首字母和其后的辅音 |
| `proj`、`tmp`、`repo` | `PRJ`、`TMP`、`REPO` | 一个词：同上；辅音太少的词取前四个字母 |
| `proj-2` | `PRJ2` | 名字里的数字放最后 |
| 你的主文件夹、`/` | `HOME`、`ROOT` | |

- **绝不与本机另一个仓库现有或曾用的 key 相同。** 最好的 key 被占用时，试名字的下一种写法（第二个 `proj` 用 `PRO`），然后是带数字的最好版本（`PRJ2`、`PRJ3`、…）。同样的名字永远给同样的 key。
- **保留。** 你设的、或仓库自己的 `.omh.json` 里写的 key 绝不被覆盖：看板只给还没有 key 的仓库发 key。
- **替身（stand-in）。** 两个仓库已经说了同一个 key 时（两个手写的条目，或两个仓库的文件），看板先认识的仓库保留它，另一个用看板做的**替身**。自己的文件（或你的条目）写着别的仓库曾用的 key 的仓库也是如此：那个 key 归那个仓库。替身存在 `~/.omh/data/repo-keys.json`，绝不在你的设置里：你写的保持你写的样子。`show --repo` 会说明（`not used: another repo has or had the same key; its tasks go by BETA`），看板升级时的提示也一样（`BETA for beta (its own KEEP is proj's too)`）。给那个仓库另设一个 key，替身就消失。
- **升级时。** 这个版本第一次运行时，看板已知但没有 key 的每个仓库一次性拿到一个，看板下次打开时说明是哪些：`Every repo and folder has a short key now: HOME for exedev, ROOT for home. Change one: omh set repo.key <KEY> --repo <name>`。被你手动从文件里拿掉的 key 在看板下次打开时重新给出，并同样说明。

## 改 key

`repo.key` 是一个设置（[设置](settings.md)），有它自己的规则，因为它是一个仓库的事实，绝不是关于某个任务或所有仓库的：

**你对仓库的条目 → 仓库自己的文件 → 看板生成的那个。**

```
omh set repo.key HSW --repo omh     your entry for that repo
omh unset repo.key --repo omh       back to the repo's file's key, or to the one made from its name
omh show --repo omh                 what it is and where it comes from
```

或者，要让它随仓库走，写在仓库自己的 `.omh.json` 里（你的条目没写时以它为准）：

```json
{ "repo": { "key": "HSW" } }
```

你开口时，协调者用同样的命令设置或修改 key。

- **2 到 5 个字母或数字，首字符是字母**，保存和显示都用大写（`hsw` 存为 `HSW`）。其他一律带着原因拒绝。`PR` 被拒绝：在行上它已经表示 PR。
- **绝不没有。** `set repo.key none` 被拒绝，并说明怎么回到生成的 key。`unset` 绝不让仓库没有 key：它回到仓库文件的 key，或从名字生成的那个，并说明是哪个。
- 在任务上（`--task`）和对你自己（不带 `--repo`）设置会被拒绝，附正确设法。
- **两个仓库一个 key：拒绝。** `set` 拒绝另一个仓库已有的 key，并点名那个仓库。也拒绝另一个仓库*曾用*的 key（`HS was omh's key and still names omh's tasks`），好让旧名字永不会指向别的仓库的任务。仓库可以拿回自己曾用的 key。
- 仓库自己的文件仍可能带来另一个仓库已有的 key（文件改动时没人被询问）。这时你自己的条目压过文件，两个同类地方都写了它的仓库里，看板先认识的保留。另一个用替身，如上，`show --repo` 会说明为什么它自己的 key 不被使用。所以一个 key 绝不会称呼两个仓库的任务。
- 仓库的文件按它现在在仓库文件夹里的样子读取（你的检出），不从任务的分支读。

## 用 key 称呼任务

`HSW-134`，任何大小写，在接受 id 的地方都被接受：每个命令（`details`、`delegate`、`tell`、`stop`、`approve`、`review`、`retry`、`fix`、`finish`、`report`、`describe`、`pin`、`archive`、`restore`、`clear`、`show`、`set … --task`）。所以「start HSW-134」就是 `omh delegate HSW-134`。协调者的规则和技能里也这么说。

id 继续可用，仍是任务真正的 id：没有任何东西存在 key 下面。改 key 不弄坏任何东西：id 仍然称呼任务，旧 key 也一样（`HS` 变成 `HSW` 后 `HS-134` 仍可用），无论 key 怎么改的：用 `set`、在设置里手动改、或在仓库自己的文件里改。看板把一个仓库用过的每个 key 记录在 `~/.omh/data/repo-keys.json`，永远归那个仓库：别的仓库不能拿走它，无论用 `set` 还是用文件。

## 找任务

在查找里（看板上 `Ctrl+F`，web 看板的搜索框），任务可以按编号、编号标签和文字找到：`134`、`#134`、`hsw-134` 都能匹配。你输入时照常即时过滤（`13` 也会显示 `HSW-13` 和 `HSW-135`）。恰好是某个任务的编号或编号标签时，选中跳到那个任务。`omh list --query` 同样匹配。
