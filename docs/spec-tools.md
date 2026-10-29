# 规格工具（OpenSpec）

**规格工具通过设置与看板配合：不需要专门支持。** 这一页用 [OpenSpec](https://github.com/Fission-AI/OpenSpec)（先写提案和规格，再构建，然后归档）；别的规格工具照同样方式配置。

## 设置

```sh
omh set worker.instructions "Use OpenSpec. Write the change's proposal and specs first, under openspec/changes/. Then build it. When it is built, run openspec archive for that change." --repo my-app
omh set check.command "openspec validate --all --strict --no-interactive" --repo my-app
```

- **`worker.instructions`** 告诉那个仓库里每个任务的代理干活的顺序（[设置](settings.md)）。
- **`check.command`** 让任务的工作只有在规格有效时才算完成。把你的测试用 `&&` 接在它后面。它也可以放在仓库自己的 `.omh.json` 里（`check.command`）。
- **给 `validate` 一个目标。** 不给的话，OpenSpec 会在终端上问是哪一个。看板跑检查时没有终端可问，所以每次都失败。`--all`、`--changes` 或某个变更的名字都行。
- **工作树没问题**：`openspec/` 在仓库里，所以每个任务的分支上都有。

## 批准提案

你多半想在构建之前先对提案说 yes。有一件事要知道：开着 `plan.approve` 时，计划步骤**写不了提案文件**。在计划拿到它的 yes 之前，代理运行在只读文件系统的仓库里，里面什么都不会变（[先计划](plan.md)）。计划是看板自己文件夹里的 `plan.md`，永远不是仓库里的文件。

两种做法都行：

| 方式 | 怎么做 | 你对什么说 yes |
|---|---|---|
| **先批计划，然后提案被写出来** | `set plan.approve you --repo my-app`，并让说明里写着「the proposal and specs are your first commit」（提案和规格是你的第一个提交） | 任务面板里计划的文本。构建随后把提案写成第一个提交，评审对照计划检查工作 |
| **在 PR 里评审提案** | 不开 `plan.approve`；保留 `merge.by you`（内置） | 合并之前，在 PR 里对提案和代码一起说 yes |

第一种在写任何代码之前拦下错的想法。第二种让你看到真实的提案文件。如果你想先单独看到提案，就把它做成独立任务（「write the OpenSpec proposal for …」），合并它，再用 `--after` 把构建加成一个任务（[一个任务接一个](after.md)）。
