# 置顶与「Pinned」筛选

**把要紧的置顶，想看时只看它们。**任务、项目或整台机器都能置顶。置顶不挪动任何东西：行按原顺序排，名字后面加一个小黄星 `★`。筛选行最后一个筛选「Pinned」（置顶）只显示你置顶的东西，就像「需要你」只显示需要你的。

```
 omh  proj · PRJ
 vm-str │ All 299   Coordinators 7   Needs you 1   Working 5   Ready 0   To do 11   Done 283   Automations 2   Pinned 2
 ○ ▾ vm-str  2 projects · 4 tasks
   ○ ▾ gtm · GTM ★
     ? GTM-14  Pick the ICP                                       2m ago
     ○ GTM-15  Write the first sequence                            1h ago
   ○ ▾ omh · HP
     ◐ HP-182  Pin machines and projects ★                ██░░ 40%  5m
```

## 置顶操作

- **`Ctrl+P`** 置顶选中的行，再按取消：任务、项目行（按仓库分组时 `Alt+S`，或协调者下）、或机器行（显示所有机器时，或顶部显示单台机器时）。进行中它的行显示 `Pinning…`（[进行中](under-way.md)）。
- **什么也不挪。**任务留在它的分组里，项目和机器留在原位；选中仍在该行。三者用同一个标记，像标签一样变暗。
- **从 shell：**`omh pin <id | machine | project>` 和 `omh unpin …`。任务用 id 或标签（tag），机器用名字，项目用名字或键（key）；别的机器的项目加 `--on MACHINE`，别的机器的任务用 `vm-b:web/7`。`pin` 置顶，原来是什么就是什么；`unpin` 取消。`omh list --pinned` 列出本机置顶的：机器、它的项目、它的任务。
- 置顶的完成任务绝不会被 `archive --done` 归档。已归档的任务或项目不能置顶；置顶的项目被归档后离开「Pinned」筛选，直到恢复。

## 「Pinned」筛选

「Pinned」和筛选行上其他筛选一样，在 `Automations` 之后：`Tab` 到那里，点一下显示它，看板像记住任何筛选一样记住它，关掉再开也还在。

- **只显示置顶的，各在各位：**置顶任务在它的分组里，置顶项目带它的全部任务，置顶机器带它的项目和任务。看板的分组（`Alt+S`）和折叠标题照旧：按仓库时，置顶项目就是它的标题、任务排在下面（没有任务时孤零零显示 `0 tasks`）；按状态或按机器时，它的行在任务上方，就像需要你的项目在「需要你」下那样。
- **计数**是你置顶的东西，每个置顶算一次：置顶的机器、项目和任务，和 `list --pinned` 列的一样（显示所有机器时，每台机器的都算）。置顶项目或机器的其他任务显示在它下面，但不加进计数，完成与否都不加。
- **一切和别处一样：**同样的标记和颜色，需要你的用红色，所有按键都可用。
- **看谁的看板**（`Alt+M`）在这里同样算数：显示所有机器时，每台机器的置顶都算。
- **完成的任务折叠收起**，和 `All` 下一样，置顶的也收：一行计数（`↓ 2 done`），`Alt+D` 显示它们。
- **没置顶任何东西：**一行提示怎么置顶。
- **需要你的永远看得见：**筛选行的「需要你」和它的红色计数就留在旁边的行上。
- **Web 看板上**是同一个筛选，筛选行最后一颗胶囊。

## 置顶存在哪

任务的置顶归它自己，随任务保存。机器和它项目的置顶存在那台机器上（数据目录里的 `pins.json`），随它的任务一起发送，所以显示该机器的每块看板看到同样的置顶，重启后也一样。置顶别的机器的项目或那台机器本身，是请那台机器去做，和它任务的置顶一样。旧版本的机器做不了：看板会提示去那边更新。

代码位置：`crates/board/src/board/update/pinned.rs`（筛选和按键）、`crates/board/src/board/update/shows.rs`（筛选行）、`crates/core/src/store/pins.rs`、`crates/flow/src/actions/pins.rs`、`crates/flow/src/remote/pins.rs` 和 `src/cli/pins.rs`。
