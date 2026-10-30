# 自动模式：`access auto`

**`worker.access auto` 用程序自己的自动模式启动代理**：程序自己检查每个动作，安全的直接做，有风险的停下。它介于看板一直有的两个值之间：

| `<role>.access` | 代理 | Claude Code | Codex |
|---|---|---|---|
| `full`（内置） | 什么都做，不问 | `--dangerously-skip-permissions` | `--dangerously-bypass-approvals-and-sandbox` |
| `auto` | 做自己检查认为安全的，其余停下 | `--permission-mode auto` | `--approve-for-me` |
| `ask` | 行动前问你 | 它自己的默认 | 它自己的默认 |

`full` 和 `ask` 一如既往，`full` 仍是内置值。

每个有访问权限的角色（role）都有同样的值：`worker.access`、`reviewer.access`、`coordinator.access`。只研究任务属于工作代理，用 `worker.access`。按任何设置的方式设置它：`omh set worker.access auto`，单个任务用 `omh delegate <id> --worker.access auto`，或在任务面板的设置页上，访问权限那行依次是 `full`、`ask`、`auto`。任务面板在代理下方显示 `auto mode`。

## 哪些程序有这个模式

五个：Claude Code、Codex、Cursor、Qwen Code、Qoder。程序表在 `Auto mode` 列给出各自的 flag 和最早版本（[agents](agents.md)）。

- **Claude Code**：`claude --permission-mode auto`，从 Claude Code 2.1.111 起（2.1.71 到 2.1.110 还要额外加隐藏的 `--enable-auto-mode` 才认这个值，2.1.70 及更早直接退出）。一个分类器模型在每个动作运行前读它：留在仓库里的放行，可能在仓库外造成危害的（把下载当脚本运行、强推、生产环境改动）被拦下，并告诉 Claude 原因，让它换条路。拿不到自动模式时（模型不支持、Claude Code 设置里的 `disableAutoMode`、或账号层面关了），Claude Code 改用它的默认模式，也就是会询问的那种：绝不会用 `full`。
- **Codex**：`codex --approve-for-me`，从 Codex 0.147.0 起。这是 Codex 的自动评审：在它的 workspace-write 沙箱里代理在仓库里自由工作，越界的请求由 Codex 自己的评审者回应，而不是你。连续 3 次拒绝，或最近 50 次里有 10 次拒绝，它带警告结束回合。
- **Cursor** `cursor-agent --auto-review`（2026.06.22 起）、**Qwen Code** `qwen --approval-mode auto`（0.16.0 起）、**Qoder** `qodercli --permission-mode auto`（0.2.14 起）：各自有自己的分类器，见各自文档。这三个是从文档里读来的，没在这里试过。
- **omp**：`omp --approval-mode=write`。它没有分类器；`write` 档是最接近的替身：文件改动不经询问直接做，执行命令前才问。`ask` 对 omp 也是这一档，所以提示不会因此变多。

有些程序有个叫 `auto` 的 flag 但不是这个：`kilo --auto`、`opencode --auto`、`kimi --auto` 和 MastraCode 的 `--permission-mode auto` 会批准一切，和 `full` 一样。Hermes 和 Grok 在自己的设置文件里有受检模式，但没有启动它的 flag。这些都不给 `auto`。

其他程序都没有这种模式，给它们 `auto` 会被拒绝，绝不会背着你变成 `full` 或 `ask`：

- **你设置时**：`worker.agent: pi cannot do "auto": it has no auto mode (only claude, codex, omp, cursor, qodercli, qwen have one)`；当角色的程序没有该模式时设 `worker.access auto` 同样被拒。
- **手写在设置文件里**，而别处指定了这样的程序：值保持 `auto`，不像不合适的 effort 或 `ask`（那些会落到下一处），因为下一处可能是 `full`。任务面板显示 `no auto mode in pi: it will not start`，代理不启动，任务说明原因：`pi cannot be started with access "auto": it has no auto mode (…). Set the role's access to full or ask, or choose another program.` `omh doctor` 会点出每一个这样的设置。
- **任务面板的设置页**上，访问权限那行会跳过它的程序做不到的值：OpenCode 是 `full`、`ask`、`full`；两样都不行的 pi 停在 `full`。
- **备用代理（fallback）**没有该模式时，在访问权限是 `auto` 期间被跳过，就像不能询问的在 `ask` 期间被跳过一样（[备用代理](fallback.md)）。
- **先计划（plan first）的任务**在写计划之前就被拒绝，而不是等你批准计划之后。

## 版本太旧

看板会读程序的版本（`claude --version`、`codex --version`，和它检查其他东西一样），在比第一个有该模式的版本更旧的版本上绝不以自动模式启动。任务会改为说明：

```
claude cannot be started with access "auto": claude 2.1.70 has no auto mode: it came in claude 2.1.111. Update claude, or set the role's access to full or ask.
```

`omh doctor` 在任何任务启动前也会这么说。读不出版本就当作新的：程序自己会说它有什么不知道。

## 代理停下来找你时

自动模式不是 `full`：它的检查决定不了的地方，程序会问，那就是它屏幕上和任何其他一样的权限提示。Claude Code 在连续 3 个动作被拦、或累计 20 个被拦后再次显示提示，你自己的 `permissions.ask` 规则点名的任何情况也一样。对看板来说没有任何新东西：

- **检查拦下的一步**不会停住代理：它会得到原因，换条路走。当这让它需要你时（比如它想运行网上下载的脚本），它会像任何代理一样说明，用看板的 `report … --state blocked --kind permission`，任务带着原因等你：`REPO-8 · Need OK to download and run the install script from example.com`。在聊天里回应它（[等你的代理](waiting-for-you.md)）。
- **它屏幕上的权限提示**（在上述拦截之后，或由你自己的规则引起）就是普通的对话框：Herdr 说代理被阻塞，任务显示在「需要你」下面，聊天显示 `The agent shows a dialog on its screen that waits for your answer: alt+o opens it.`。按 `Alt+O` 进入代理，在那里回应。

2026-10-19 在 Herdr 上用 Claude Code 2.1.303、在一块试验看板上试过：带 `worker.access auto` 的任务以 `claude --permission-mode auto` 启动，底部显示 `auto mode on`，做完工作并合并；另一个被检查告知运行 `curl … | sh` 的任务，下载被拦下（`denied by the Claude Code auto mode classifier … [Code from External]`），通过看板等一个批准，如上所述。
