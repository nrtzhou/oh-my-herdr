# 被服务商中断的回合

**模型的服务商在回合中途出错时（满载、过载、限流、断连），看板自己等待并重试**：隔 1、2、4、8 分钟，十五分钟内五次。然后才放弃并告诉你。等待修不好的错误立刻交给你。

这样的回合既没完成也没失败：不是代理停了，是它的服务商停了。

## 看板怎么知道

代理自己保存的会话记录里记着该回合的错误，看板在那里读到：

| 程序 | 错误在哪 |
|---|---|
| Codex | `task_complete` 上的 `error` |
| Claude Code | 带 `isApiErrorMessage` 的条目 |
| pi | `stopReason` 为 `error` 的消息 |

程序自己的错误代码决定它算会过去（Codex 的 `codex_error_info`、Claude Code 的 `error` 和 HTTP 状态）。没有的话，看它说的话。

## 评审者

看板关掉评审，等待，重新开始。

- 期间行上写明原因和下次时间：`Reviewer's model is at capacity, trying again in 2m`。
- **被掐断的评审不算评审。** 不耗任务的 `rounds`，没有编号（下一个仍是“1st”），也不进评审列表。
- **最后一次也被掐断**，任务带着真实原因和服务商原话失败：`Review failed: the reviewer's model is at capacity, still after 5 tries over 15m. Its error: Selected model is at capacity…`。`Ctrl+L` 重审。

## 工作代理

同样的等待。然后看板让代理从断处继续，算插件自己的一个回合：计数器和已批准的合并都能活过它。

- 行仍在 `working` 下，写着 `Agent's connection dropped, trying again in 2m`。
- **你可以跳过等待。** 回复、`Ctrl+R` 或 `Ctrl+L` 立即继续。停任务结束等待。
- 最后一次之后，任务是“Agent stopped”，带真实原因。
- 一次回复让计数重新开始。
- 程序因错误退出的代理，在同一会话里重启，被告知同样的事。

## 等待修不好的错误

绝不重试，立刻交给你：登录、模型不存在或不提供、用量上限、服务商拒绝的请求，以及看板不认识的错误。评审者撞上一个，评审以“the reviewer stopped:”加错误失败；这也不进评审列表。

## 配合备用代理

设了 `worker.fallback` 或 `reviewer.fallback` 时，看板要放弃或交给你之前由另一个代理接手：会过去的错误在最后一次重试后；等待修不好的立刻。顺序是：**先重试，再备用代理，然后你**（[备用代理](fallback.md)）。

## 细节

等待是固定的。真实会话记录里看到的东西在 [Herdr 上的实测](checked-on-herdr.md)。

- `crates/core/src/agents/provider_error.rs` 里的 `provider_error`：哪些错误会过去；
- `crates/core/src/store/retries.rs`：等待；
- `crates/flow/src/finish/turn_ended.rs` 和 `crates/flow/src/finish/review.rs`：重试；
- 测试：`tests/retry.rs`、`tests/retry_agent.rs`、`tests/endings.rs`，录制的结局在 `tests/fixtures/endings`。
