# Oh My Herdr（omh）

herdr 插件：一张任务看板，把要做的事交给多个 coding agent 并行完成。你说需求，协调者（Coordinator）拆成任务；每个任务一个 agent、一棵独立工作树、一条独立分支，绝不碰你自己的 checkout。任务收尾自动提交、跑仓库检查、由第二个 agent 评审，最后等你点头才合并。需要你回答或批准的任务排在最前。

## 安装

前置：Linux 或 macOS；[herdr](https://herdr.dev) 0.9.3+（**正在运行的 server 也要是 0.9.3+**）；`git` 与 `curl` 或 `wget`。可选：`gh`（开 PR）、`tailscale`（私有评审链接）。不需要 Node/npm。

```sh
herdr plugin install nrtzhou/oh-my-herdr
```

首次打开看板时它会请求写入 herdr 配置（按键 `prefix+a`、侧栏行），同意即可。把 `~/.omh/data/bin` 加进 PATH 后可以直接敲 `omh`。

- 从源码构建：`git clone https://github.com/nrtzhou/oh-my-herdr && cd oh-my-herdr && cargo build --profile quick`，产物在 `./target/quick/omh`。
- 更新：`omh update`。固定版本：`herdr plugin install nrtzhou/oh-my-herdr --ref v1.2.27`。
- 出问题先跑：`omh doctor`。

## 上手（三步）

1. **打开看板**：herdr 里按 `prefix+a`（或任意面板敲 `omh`）。底部输入要做的事，回车——协调者把它变成任务；按 `Ctrl+S` 则立即派发一个 agent。
2. **看分组**：看板按 需要你 / 待批准 / 进行中 / 待办 / 完成 排序，人的注意力永远在「需要你」上。按 `?` 看全部按键。
3. **等结果**：任务完成走收尾流水线（提交 → 合并 main → 检查 → 评审），通过后进「待批准」，`Ctrl+L` 说 yes 才合并。中途有疑问它停在「需要你」等你答。

## 常用命令

```sh
omh add "修复登录页" --delegate   # 加任务并立即开工
omh list                         # 所有任务
omh show 12                      # 任务 12 的详情与各设置取值来源
omh approve 12                   # 批准合并
omh set worker.agent omp         # 任务代理用 omp
omh update && omh doctor         # 更新 + 体检
```

全部命令一页说明：[docs/roles-and-commands.md](docs/roles-and-commands.md)。

## 配置

一切皆设置，`omh set <键> <值>` 即可；取值就近生效：**任务 → 仓库条目 → 你 → 仓库文件 → 内置**（`omh show 12` 看每个值从哪来）。常用：

| 设置 | 说明 | 默认 |
|---|---|---|
| `worker.agent` | 任务代理程序（`omp`、`codex`、`claude`…18 种） | claude |
| `reviewer.agent` | 评审代理 | codex |
| `merge.by` | `you` 合并前等你点头；`itself` 自动合并 | you |
| `board.agents` | 同时最多几个 agent，超出排队 | 4 |
| `limit.time` / `limit.tokens` | 单任务预算，到限停下等你 | none |

完整设置清单：[docs/settings.md](docs/settings.md)。各 agent 程序的能力差异（谁能读会话、谁能自动重试）：[docs/agents.md](docs/agents.md)。数据都在 `~/.omh`。

## 文档

- 入门：[快速上手](docs/get-started.md) · [安装详情](docs/install.md) · [工作是怎么完成的](docs/how-work-gets-done.md) · [故障排查](docs/troubleshooting.md)
- 看板：[看板](docs/board.md) · [按键](docs/keys.md) · [任务面板](docs/task-panel.md)
- 任务形态：[评审门](docs/gate.md) · [先计划](docs/plan.md) · [只研究](docs/research.md) · [并行尝试](docs/attempts.md) · [任务链](docs/after.md)
- 自动化：[自动驾驶](docs/autopilot.md) · [自动化任务](docs/automations.md) · [多机](docs/other-machines.md) · [语音](docs/voice.md)

更多见 [docs/](docs/) 目录。

## 致谢与沿革

基于 [Herdr Projects](https://github.com/eliasstravik/shepherdr)（Elias Stravik，MIT 许可）1.2.27 分叉维护，感谢原项目。

**分叉初衷：增强对 omp（oh-my-pi）的支持。** 本版把 omp 从「只能读屏的二等程序」提升到与 Claude Code / Codex / pi 同级：看板直接读它的会话——回合结束、token 用量（含侧调用）、聊天转录、会话发现与恢复、收尾钩子，`limit.tokens` 等能力一并生效。

**与上游后续版本（Shepherdr 2.0）的理念区别**，也是本 fork 存在的理由：

- **人是合并闸门**：默认 `merge.by you`，评审通过后仍等你点头才合并；2.0 默认自动合并，人退到「被通知」。
- **评审门独立裁决**：评审者只列发现，看板按固定规则判定通过与否；2.0 改为任务 agent 自评自判。
- **控制优先于规模**：保留 `limit.time` / `limit.tokens` 限额、并行尝试（attempts）、先计划沙盒、终端看板与键盘操作；2.0 面向手机端放手跑量。
