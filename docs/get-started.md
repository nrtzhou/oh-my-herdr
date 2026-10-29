# 上手

你需要 macOS 或 Linux 上的 [Herdr](https://herdr.dev) 0.9.3 或更新版本，以及 Claude Code、Codex、pi 或 Herdr 支持的其他代理程序。

## 选项 A：自己动手

1. 安装：

   ```sh
   herdr plugin install nrtzhou/oh-my-herdr
   ```

2. 在 Herdr 里运行动作 **Open the work board**（打开工作看板）。它给出的按键（`prefix+a`）选 yes。
3. 可选：把 `~/.omh/data/bin` 加进 PATH（fish：`fish_add_path ~/.omh/data/bin`），就能直接敲 `omh`。启动器已自动写好。

## 选项 B：让代理来做

把下面这段贴进运行在 Herdr 里的 Claude Code、Codex 或 pi：

```text
Install the omh plugin for Herdr and set it up for me.
1. Check `herdr --version` is 0.9.3 or newer, and that git and curl (or wget) exist. Stop and tell me if not.
2. Run `herdr plugin install nrtzhou/oh-my-herdr`.
3. Add ~/.omh/data/bin to my PATH in my shell's start-up file (fish: fish_add_path ~/.omh/data/bin), so `omh` works in new shells. Its launcher is written by the install itself.
4. Run `omh doctor` and fix anything it reports.
5. Open the board: `herdr plugin action invoke open --plugin omh`.
Then tell me in two lines: it is ready, and to say yes to the key it offers (prefix+a) on its first screen.
Details if you need them: https://github.com/nrtzhou/oh-my-herdr/blob/main/docs/install.md
```

## 启动

- 在 Herdr 里按 `prefix+a`，或在任意面板里敲 `omh`。
- 输入要做的事，按 `Enter`。协调者（Coordinator）会把它变成任务。
- 按 `Ctrl+S` 代替 `Enter`，立刻派发（delegate）代理去做。
- 按 `?` 看其余全部功能。

以后更新用 `omh update`。

更多细节（从源码构建、旧安装、卸载）：[安装](install.md)。
