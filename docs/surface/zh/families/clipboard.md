# 剪贴板

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

语言: 中文（简体）

三个方言中属于 **剪贴板** 类的名称，以及这棵树对每个名称给出的回答。分类是脚本思考的领域，这些名称即是它的全部。

| 名称 | 方言 | 种类 | 类 | 参数 | 状态 | 调用探测 |
|---|---|---|---|---|---|---|
| `#ClipboardTimeout` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `A_Clipboard` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `Clipboard` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Clipboard` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `ClipboardAll` | AutoHotkey v2.0.19 | class | -- | -- | 值 | -- |
| `OnClipboardChange` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | 值 | -- |
| `ClipboardAll.Ptr` | AutoHotkey v2.0.19 | method | ClipboardAll | -- | 值 | -- |
| `ClipboardAll.Size` | AutoHotkey v2.0.19 | method | ClipboardAll | -- | 值 | -- |

## 这棵树对它做什么

### A_Clipboard

桌面剪贴板中的文本。脚本可以写它，连接运算会在已有内容后追加。

- **参数:** nothing; it is a variable, not a call
- **返回:** 剪贴板中的文本
- **错误:** the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.

```ahk
A_Clipboard := "hello"
FileAppend A_Clipboard, "*"
```

### Clipboard

剪贴板的变量；v2 写作 A_Clipboard，v1 写作 Clipboard，两者指的是同一份文本。Wayland 会话用 wl-paste 读，X11 会话用 xclip 或 xsel 读，用 wl-copy 或 xclip 写。

- **参数:** --
- **返回:** 剪贴板上的文本
- **错误:** an empty clipboard is a value and not a failure; an X11 session with neither xclip nor xsel answers "the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine"; a write calls the function registered for OnClipboardChange with 1, because the change the runtime can see is its own.

```ahk
text := A_Clipboard
```

### Clipboard

剪贴板的变量；v2 写作 A_Clipboard，v1 写作 Clipboard，两者指的是同一份文本。Wayland 会话用 wl-paste 读，X11 会话用 xclip 或 xsel 读，用 wl-copy 或 xclip 写。

- **参数:** --
- **返回:** 剪贴板上的文本
- **错误:** an empty clipboard is a value and not a failure; an X11 session with neither xclip nor xsel answers "the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine"; a write calls the function registered for OnClipboardChange with 1, because the change the runtime can see is its own.

```ahk
text := A_Clipboard
```
