# 标题

**标题最多 60 个字符。** 超长的标题会被缩短，不会被拒绝，而且全文会保留下来。

无论在哪里创建或修改标题，限制都一样：在输入框里敲的任务、`add`、以及 `report --title`。

## 过长的标题

```
$ omh add "After the startup dialog handoff, finish jobs wrongly mark the task as failed"
proj/12: added The title was over 60 characters, so it was shortened to "After the startup dialog handoff, finish
jobs wrongly mark…" and the full text was kept in the description. Rename it if a better short title exists.
```

上面这条回复的意思是：标题超过 60 字符，被缩短为 `After the startup dialog handoff, finish jobs wrongly mark…`，全文保存在描述里；如果有更好的短标题，就重命名它。

- 在单词末尾截断，结尾是 `…`：`After the startup dialog handoff, finish jobs wrongly mark…`。
- 一个长单词无处可断时，就在空间用完处截断。
- 只超出一个句号的标题，直接去掉句号。
- 全文成为描述的第一段，放在原有内容之上。如果描述里已经写了，就保持不动。

为什么缩短而不是拒绝：代理替你加的任务不会因为标题而丢，回复里也会告诉你该怎么做。

## 为什么是 60

这是标题在列表行上能占的位置：120 列的窗口里打开任务面板后，列表宽 72 格，剩下的给标记和时间。Herdr 的侧栏里任务空间名排在任务号之后，得到的也是这么多位置。60 字符的标题在两处都能完整读到。100 字符的句子会挤满整行，把其他内容顶掉。

## 字符怎么数

按屏幕占位算，和画行的方式一致。多数字母算 1 格。宽字符（中文、日文、韩文、emoji）占 2 格、算 2 个字符，所以 30 个就填满标题。顶格的标题是 60 格，无论什么语言。

## 谁会被告知

让标题写出来就合尺寸、无需截断：

- 协调者的规则在讲如何添加和重命名任务时给出这个限制。
- 看板的 how-to（`SKILL.md`）在讲如何添加任务时给出这个限制。

## 限制之前写下的标题

保持原样：没有东西会改写已保存的任务。列表会像以前一样用 `…` 截断长标题。限制在下次重命名时才生效。

这个数字是 `crates/core/src/store/titles.rs` 里的 `TITLE_MAX`，除此之外没有别处知道它。
