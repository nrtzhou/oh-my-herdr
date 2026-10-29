# 安装、首次运行与卸载

**一条命令安装，然后打开一次看板。**简版见[上手](get-started.md)。本页是细节。出问题看[排查问题](troubleshooting.md)。

```sh
herdr plugin install nrtzhou/oh-my-herdr
```

## 前置条件

- Linux 或 macOS，x86_64 或 aarch64。
- Herdr 0.9.3 或更新。你**正在运行**的 Herdr 服务器也必须是 0.9.3 或更新：旧的会拒绝该插件，直到重启。
- `git`，以及 `curl` 或 `wget`（用来下载程序）。
- 可选：`gh`（PR）和 `tailscale`（私有评审链接）。

不需要 Node，不需要 npm：看板就是一个程序。

## 安装做了什么

Herdr 把仓库的 main 分支克隆进自己的插件目录（`herdr plugin list` 显示位置），在那里运行插件的构建步骤，下载程序。`--ref v1.2.27` 可以取指定发布版。

想自己留一份检出：

```sh
git clone https://github.com/nrtzhou/oh-my-herdr && cd omh
herdr plugin link "$PWD"          # hooks, the board's tab and the actions
sh bin/run.sh install             # gets the program, writes its launcher, says how to put it on your PATH
```

## 首次运行

打开看板：用插件动作 "Open the work board"，或在任意面板敲 `omh`。首次在 Herdr 内打开时会显示一个界面，列出两项可以写进 Herdr `config.toml` 的内容。每项都由你决定：

| 按键 | 作用 |
|---|---|
| `1`、`2` | 开关每一项 |
| `k` | 换一个按键 |
| `enter` | 保存 |
| `esc` | 下次再问 |

修改前先备份文件；每个块位于两行标记之间；Herdr 拒绝的改动会回滚。

**1. 打开看板的按键。**它建议 `prefix+a`，Herdr 自己不用这个键。

- 如果你的配置已经绑定 `prefix+a`，它会建议下一个空闲键（`prefix+y`，然后 `prefix+u`、`prefix+i`、`prefix+m`），并说明原因。
- Herdr 默认绑定的键、或你的配置已绑定的键，绝不会占用。界面会拒绝并说明该键现在的用途（比如 `prefix+w` 是 Herdr 自己的工作区选择器）。对建议的键和你用 `k` 输入的键都如此，无论配置里按键怎么写（单键、键列表、旧式 `[keys.indexed]`）。
- 已为看板设置的键保持原样，无论配置怎么写（`[[keys.command]]`，或 `[keys]` 下的 `command = [{…}]`）。
- 该键在独立标签页里打开看板。想以浮层方式盖在 Herdr 窗口上打开（再按同一键关闭）：`omh set board.open popup`（[看板](board.md)）。按键本身不变。

**2. 侧栏行**，让代理在 Herdr 侧栏（sidebar）里的条目成为其看板任务行的小缩影（[Herdr 的侧栏](sidebar.md)）：

- 任务编号与标题；
- 谁在做、进展到哪，含评审轮次和 PR（`agent · working · R#1 PR#113`）；
- 百分比与剩余时间（`8% 16m`）。

没有它们时，Herdr 只显示前两项（标题后跟标签页名 `Agent`），不显示第三项：默认侧栏不显示自定义值。旧版插件写入的行会在同样的标记行之间自动更新。你自己写的行绝不动：若配置已设置 `ui.sidebar`（无论 `[ui.sidebar.agents]`、`[ui]` 下的 `sidebar = {…}`，还是点号键），就不会提供插件的行。首次运行会用一行说明保留了你的行，并显示插件的行，供你手动合并（小窗口里 `↑` `↓` 和 `pgup` `pgdn` 滚动，按其他键继续）。

其余什么都不需要：不用 Herdr 集成（integration），不用在你自己的代理设置里加钩子（hook）。

### 手动添加这些块

```toml
[[keys.command]]
key = "prefix+a"
type = "plugin_action"
command = "omh.open"

[ui.sidebar.agents]
rows = [["state_icon", "machine", "workspace", { token = "tab", rules = [{ equals = "Agent", hide = true }, { starts_with = "Reviewer #", hide = true }] }], ["agent"], [{ token = "$summary", dim = true }, "$progress"]]

[ui.sidebar.spaces]
rows = [["state_icon", "workspace"], ["branch", "git_status"], [{ token = "$summary", dim = true }, "$progress"]]
```

**小心按键。**`[[keys.command]]` 会无警告地覆盖 Herdr 对同键的绑定。手写在 Herdr 在用的键上，会关掉 Herdr 的动作。（本页早期版本在这里写了 `prefix+w`，夺走了 Herdr 的工作区选择器。）`herdr --default-config` 列出 Herdr 绑定的大部分按键（不含复制模式 `prefix+[` 和交换面板的 `prefix+shift+h` `j` `k` `l`）。要移动看板的按键，改 `key` 行并运行 `herdr server reload-config`。

也可以绑定 `omh.approve-current`。对拥有当前焦点面板的任务，它会在看板上打开该任务并弹出合并（merge）问题（`y` = yes，`n` 或 `esc` = no）。其他位置没有作用。

## PATH 里的命令

代理和钩子拿到的是启动器 `~/.omh/data/bin/omh`，它运行真正的程序。

- 看板打开时会写入它。用 `herdr plugin install` 的话，先用插件动作 "Open the work board" 打开一次看板。
- `omh install` 会重新写入，并提示 PATH 里是否有别的同名 `omh` 排在前面。
- 想在 shell 里敲 `omh`，把那个目录加进 PATH（`~/.profile` 或你 shell 的启动文件）。

**在会被其他机器索取任务的机器上**（[其他机器](other-machines.md)），`omh` 必须能被无人交互的登录 shell 找到，也就是 SSH 启动的那种 shell。只在 `~/.bashrc` 设 PATH 往往不够。`omh machines` 会在本机检查并告诉你怎么修。从另一台机器这样测；一切正常时末行是一行 JSON：

```sh
ssh <machine> 'exec $SHELL -lc "omh remote list"' | tail -c 200
```

## 更新

```sh
omh update           # the newest release
omh update 0.3.1     # a given version
```

- 一步把发布版放进下载程序的位置。
- 打开的看板和看守（keeper）会发现文件变化并自行加载（[看板的尺寸](board-size.md)）。
- 拉取更新的插件（`git pull`）会在 Herdr 下次运行命令时做同样的事：`bin/run.sh` 下载那个版本。
- 自己构建的程序用 `git pull` 和 `cargo build --release` 更新，绝不下载覆盖。

**看守会自我更新。**新文件停止变化后（还在链接的构建会等它完成），若没有打开的看板、没有运行中的代理，看守就在新文件上启动后继者，等后继者跑起来才退出。新文件起不来就留下旧看守继续跑，稍后重试（数据目录里的 `keeper.log` 有记录）。

## 从源码构建

在检出目录里运行 `cargo build --release`（Rust：https://rustup.rs）。`bin/run.sh` 会运行那个构建，之后的构建也会被同样拾取。

## 从旧版本升级

- **从 herdr-superwork 或旧 omh 0.2：**首次运行会自动接管（数据迁移或搁置、Herdr 配置里的附加内容改好、代理的设置和技能（skills）目录改好、旧进程替换），并说明做了什么。`omh doctor` 会再查一遍并汇报现状（[旧安装](old-installs.md)）。
- **从旧的 Node 版本：**`npm unlink -g omh` 把 Node 程序从 PATH 移除。本版首次运行会在改动前原样保留一份数据目录副本（`~/.omh/data-0.2.0`）。数据格式相同，不做转换。
- **从文件放在 `~/.local/state`、`~/.config` 和 `~/.local/share` 的版本：**什么都不用做。首次运行会把它们移过来并用一行说明。

看板为你保存的一切都在一个目录 `~/.omh/` 里：[主目录](home-folder.md)。

## 卸载

**解除插件链接前，先运行 `omh remove-extras`。**Herdr 不给插件卸载钩子，所以由它把标记块从 `config.toml` 移除，并删掉 how-to 文件（[角色与命令](roles-and-commands.md)）。装了 Web UI（Ovis）时，它还会把 Web UI 的共享链接指回 Web UI，关掉看板的前门（[Web UI（Ovis）里的看板](web-ui.md)）。

## 细节：运行的是哪个程序

程序每平台一个文件：`omh-linux-x86_64`、`-linux-aarch64`、`-macos-x86_64`、`-macos-aarch64`。Linux 版在任何 Linux 上都能跑。

Herdr 为插件运行的每条命令都经过 `bin/run.sh`，它按顺序运行找到的第一个：

1. 本检出构建的程序（`target/release/omh`），且没有比它新的源文件；
2. `bin/` 里下载的那个，当它是本检出的版本（`Cargo.toml`）或更新时；
3. 现在从项目的 GitHub 发布页下载本检出版本的该文件：校验 checksum，一步就位，同时只跑一个下载；
4. 以上都不行时，用旧构建或旧下载，并输出一行说明。失败的下载十分钟后重试，不逐条命令重试（否则 Herdr 的每个钩子都要等它）。

一个都没有时，它说明怎么办并立即退出，绝不拖住 Herdr。

- 插件的构建步骤（`sh bin/run.sh --fetch`）只获取程序，下载不了才从源码构建。
- `OMH_RELEASES` 指定发布版的另一个来源（fork、镜像、`file:///…` 形式的目录）。
- 发布文件由 `.github/workflows/release.yml` 为每个版本标签构建。
