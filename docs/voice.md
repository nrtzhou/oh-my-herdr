# 语音通话

**一通电话就是和看板语音代理的实时对话，围绕聊天正在交谈的对象。** 你说话，它回应，你可以打断它。你要它做事时，它把你的请求发给聊天指向的那个代理（协调者、选中任务的代理、或机器/仓库的协调者），并说明已发送。那个代理回答后，它用一两句话告诉你要点。它说你的语言，你切换它跟着切。

语音代理运行在 [ElevenLabs](https://elevenlabs.io/agents)。它自己绝不动看板：一切都发给看板自己的代理，就像你在聊天里打字一样。

## 发起通话

- **web 看板：** 聊天框里的 **Call** 按钮（电话图标），就在听写用的麦克风旁（[web 看板的聊天](web-chat.md)）：聊天停靠在 web UI 上时在 web UI 自己的框里，否则在看板自己的框里。麦克风把你的话写进框里；Call 和语音代理通话。悬停或读屏时显示 `Start a voice call`。电话拨号或通话中，同一个按钮变红并挂断（`Cancel the call`、`End the call`），通话行在聊天栏下面，其 **End**（拨号中是 **Cancel**）也挂断。它使用打开页面的手机或电脑的麦克风和扬声器，带浏览器的回声消除。
- **终端看板：** `Ctrl+T`（t for talk）。再按挂断。通话在你坐着的电脑上进行，不在看板运行的机器上（[下文](#从终端看板)）。

第一通电话会向浏览器要一次麦克风。通话是私密的：需要看板的私有链接（Tailscale），页面拿到的是一次通话有效的一次性 token，绝不是 API key。

## 通话中它做什么

- **它跟随聊天。** `Space`、在任务间移动、或点聊天的切换器，都会改变通话交谈的对象。你也可以说出来：「talk to HP-52」、「the voice task」、「back to the Coordinator」。
- **清晰的消息。** 语音代理亲自为编码代理撰写每条消息：一条自包含的指令或问题，只含你对那个代理的要求，保留你的意图和关键措辞，名字、数字、文件名和任务标签原样保留。闲聊、自言自语、你对语音本身说的话、以及说给另一个代理的内容都被剔除。不添任何东西：代理收到的、聊天里显示的，就是那条指令，就像你打的一样（一个例外见下）。语音的语气标记（句前的「[neutral]」）只出现在它说的话里，绝不进聊天或消息；你自己文字里的方括号（「items[index]」、「[dependencies] must stay」）保留。
- **绝不做你没要求的动作。** 语音代理会说明每条消息要求什么：批准、合并、停止或删除任务，或什么都不要。这些动作只有在你要求之后、语音代理向你确认过（点名动作和任务：「Merge HP-52, right?」）、而你最后一句是明确的 yes 时才发出。看板自己从通话内容里核对：一个问题（「what does merge mean?」）、一个 no（「no, don't do it」）、对别的事情说的 yes（「yes, explain it」）或关于另一个任务的 yes 都不算数，语音代理声称你说了 yes 也不算。no 会收回你之前的要求，提供别的东西的问题（「would you like an explanation of merge?」）不算确认。看板只懂英语和瑞典语的这些词：别的语言里，要求批准、合并、停止或删除时请用英语说 yes（其他内容任何语言都行）。
- **每条消息一个动作，用看板的话说。** 确认过的动作按看板的写法发出，只有动作和它的任务（「Approve HP-52.」），绝不用语音代理的措辞：它写的其他内容一概不带。提到另一个动作的（哪怕换种说法：「approve HP-52 and ship it」）不发出；语音代理只发批准，再问合并的事。
- **其他内容照原样发出。** 看板不试图解析一句话里藏着什么命令（问题可能点到动作，命令可能藏在长句里）：语音代理发来的不带动作的消息原样送出。只有当它的话点名或可能命令那四者之一（merge、ship it、get rid of、kill……）时，后面才跟一行短句：「(Voice call: nothing was confirmed, so ask the human before you approve, merge, stop or delete.)」。说了什么保留在看板上供代理查，绝不粘进消息里。而且只有语音代理一句话绝不发东西：从上条消息以来你的话没有一个字到达看板时，消息不发。
- **回答会被读出来。** 通话对象的代理回答后、文字静止片刻，语音代理就说要点。代码、路径和链接略过。
- **只在停顿时播报更新。** 通话开始后有任务需要你、变成待批准或失败，会说一次，方式和 `board.notify` 一样。绝不在你说话时，绝不盖过语音代理：等下一个停顿。「Be quiet」在本次通话余下时间关掉这些更新；你发出的内容的回答照来。
- **快问快答**（「what needs me?」）由看板立即回答，不问代理。
- **聊天显示通话：** 你交谈对象下面有一行说通话在进行，带最后一句说了什么。发出去的内容在代理的聊天里，就像你打的一样。web 看板上这行有呼吸点、通话时长、和谁、状态（Listening、Speaking、Muted）、通话所在电脑（不是页面自己的浏览器时）、Mute 和 End。拨号中的通话显示「Calling…」加黄点和 Cancel。
- **结束后什么都不留。** 通话一结束行就消失，两种看板都是。web 看板用一行小字说明：「You hung up · 2 min · 19:20」（对方挂断则是「Call ended」），几秒后淡出；通话没能开始的原因（没有麦克风、未获允许）同样方式显示，红色，留得更久。终端看板在底部说一次「Hung up.」，和它报告刚发生的一切一样。
- 两分钟没人说话的通话自动挂断。没有通话超过一小时。

## 从终端看板

终端看板常运行在别的机器上（你通过 herdr 自己的窗口看它），有时就在你这台。无论哪种，通话的声音都在你坐着的电脑上：`Ctrl+T`（t for talk）在那里振铃，**通话助手（call helper）**就在终端所在的电脑上持有着通话，用它的麦克风和扬声器。不用浏览器，不用链接，不开窗口。

- 助手是 omh 的一部分。在装了 omh 且 herdr 有已保存机器（herdr 侧栏里的那些）的电脑上它自己运行，并像看板连接其他机器那样通过 SSH 在每台上监听（[其他机器](other-machines.md)），也在这台电脑本身上监听。`voice.helper none` 关掉它。
- **哪台电脑接。** herdr 窗口在同一台电脑上的看板（herdr 的 Local，正在使用：macOS 上最近两分钟有输入，Hyprland 上有终端在最前）由那台电脑自己的助手接听，其他电脑都不得而知。否则每个监听的电脑都报出你有多确定在它跟前，最有把握的接：Hyprland 上 herdr 窗口显示着看板的机器在最前，其次是有近期输入的终端在最前，再次是有近期输入。Mac 只算最近两分钟有输入的，绝不认只是终端留在最前的。
- **API key 留在看板的机器上。** 看板的机器发起通话，把一个签名地址交给助手用于一次 ElevenLabs WebSocket 对话；语音代理的工具、听到的内容和要读回的内容在助手与看板之间经助手的 SSH 连接传输。
- **声音。** Linux 上用声音服务器自己的工具（PipeWire 的 `pw-record` 和 `pw-play`，或 `parec` 和 `pacat`，PipeWire 也认它们），经它的 WebRTC 回声消除（`module-echo-cancel`，通话时对在用的麦克风和扬声器加载，16 kHz 单声道带增益控制，用完卸载；需要 `pactl`）。安静的麦克风会稍微提一点，按你的语音提，绝不按房间的底噪提。macOS 上用一个 Apple 语音处理小程序消除回声；首次使用时用 Xcode 命令行工具编译一次（`xcode-select --install`），macOS 会问一次 herdr 所在的终端是否可以用麦克风（系统设置，隐私与安全性，麦克风）。
- **需要耳机**的地方是没法消回声的（Linux 没有 `pactl`、macOS 语音处理不可用）：聊天会在通话那行说明（`on laptop · headphones needed: no echo cancelling there`）。麦克风持续十秒几乎无声也一样（`your microphone is very quiet: the voice may not hear you`）。
- `omh voice test` 在你坐着的电脑上试通话的声音：放一小段音，听一会儿麦克风，然后说回声消没消掉。
- **几秒后没有助手接**：聊天会说明。助手接不了的通话（没有声音工具、没有连接）会结束，看板说明原因。

## 配置

- **ElevenLabs API key** 留在看板运行的机器上。在 ElevenLabs 的 Developers → API Keys（https://elevenlabs.io/app/developers/api-keys）创建，**ElevenAgents** 权限设为 **Write**：它创建语音代理和它的工具，并发起每通电话。也给 **Voices** 权限 **Write**，这样通话的声音能加进你的账号（见下）。然后把它放进两个位置之一，按此顺序查找：
  - omh 运行环境的 `ELEVENLABS_API_KEY` 环境变量；
  - `~/.omh/settings/elevenlabs.json`，写成 `{"api_key": "…"}`（只有你能读：`chmod 600`）。

  没有 key 的通话会这么说，`omh voice` 也一样。
- **声音**是 Warren（"Effortless and Cool"，`7QN34D2r3hCNwbOYIeK0`），来自 ElevenLabs 的 Voice Library，是 ElevenLabs 指定接替其默认声音 Will 的（Will 在 2026 年底退役，2026 年 3 月之后开的账号从来没有它）。代理只能用你账号里的声音说话，所以第一通电话会把 Warren 加进你的声音，这需要 Voices 权限。没有该权限的 key 在 2026 年 3 月之前开的账号里用 Will 说话，一天后再试 Warren；两者都没有的账号，通话会说怎么手动加 Warren（Voice Library，add to My Voices）或怎么选你自己的声音（`voice.voice`）。ElevenLabs 可能不允许免费套餐的 API 用 Voice Library 的声音：那就选你自己的声音。其余不用手动配置：语音代理、它的提示词和工具在第一通电话前创建在你的 ElevenLabs 账号里，此后随本程序保持同步。
- **终端看板的通话**需要你坐着的电脑上有 omh（它的通话助手），以及从那里 SSH 到看板机器且 PATH 上有 `omh`，和其他机器的要求一样。
- `omh voice` 说明现状：key 在哪找到（绝不显示 key）、助手、上一通电话的声音如何、是否有通话在进行。`omh voice end` 挂断。

| 设置 | 作用 |
|---|---|
| `voice.language` | `auto`：以英语开始，随你进入 ElevenLabs 代理会说的任何语言；或它们的语言代码之一（`en`、`de`、`es`、`fr`、`ja`、`pt-br`……）只用那一种，此时语音等你先开口 |
| `voice.llm` | 语音代理在 ElevenLabs 用的语言模型，或 `none` 用内置的 DeepSeek V4.1 Flash（`deepseek-v41-flash`）；语音始终是 `eleven_v4_turbo` |
| `voice.voice` | 代理可用的 ElevenLabs 声音 id（带实时审核的不行），或 `none` 用内置的 Warren（[上文](#配置)）。ElevenLabs 拒绝的声音会在通话里说明，并说怎么换一个 |
| `voice.calls` | `on`：通话照常；`off`：完全没有通话（[下文](#关闭通话)） |
| `voice.helper` | `auto`：通话助手在有已保存机器的电脑上运行；`none`：从不 |

## 关闭通话

`omh set voice.calls off` 在本机关闭语音通话，用于你不能或不想用它们的时候。关闭后语音整个消失，不只是坏掉：

- web 看板没有 Call 按钮，它的页面不被允许连 ElevenLabs；
- 终端看板上按 `Ctrl+T` 只说 "Voice calls are off (voice.calls)"，别无动作；`?` 菜单在它的行上同样说明；
- 看板的前门拒绝发起通话或运行语音代理的工具；
- 通话助手不在这台电脑上运行，无论 `voice.helper` 说什么，它都不在任何机器上通过 SSH 监听；
- 什么都不联系 ElevenLabs：不创建也不同步语音代理，不往你的账号加声音。
- 关闭时正在进行的通话在几秒内结束，两种看板都一样，放开麦克风和 ElevenLabs。

`omh voice` 会说通话已关闭以及怎么再打开：`omh set voice.calls on`（或 `omh unset voice.calls`）。往聊天输入框里的听写不是通话，有自己的开关：web UI（Ovis）的 Settings → Voice input。

## 各部分如何协作

- **web 看板的页面**持有它的通话：浏览器里的 ElevenLabs 客户端走 WebRTC，内置于程序（`crates/web/src/webui/page/vendor/`，由 `scripts/voice-sdk.sh` 生成），带自己的 audio worklet，让 web UI 对脚本的规则保持原样。web UI 的页面只被允许连 ElevenLabs 的通话服务器，别的什么都不多连。
- **前门**（`/omh/api/voice/`）发起 web 看板的通话（给 web UI 允许驾驶的页面发一个 token），在通话所属的看板上运行语音代理的工具，并给出要读回的内容。终端看板的通话在看板机器上的监听器里跑同一份代码（`omh voice listen`，由助手经 SSH 启动）：它用一次性票据发起通话并保管通话的 key。
- **助手**（`omh voice helper`）持有着终端看板的通话：它的声音、它到 ElevenLabs 的 WebSocket（Rust 里的 TLS，无需安装）、它对看板的请求。
- **通话记录**（`data/voice/call.json`）：一台机器同时只有一通电话，和谁、说了什么。它的票据和 key 只以哈希保存。
- 费用：ElevenLabs 按通话开着的分钟数计费，走你的套餐。
