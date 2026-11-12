# The clipboard

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [Français](fr/index.md) | [Italiano](it/index.md) | [Polski](pl/index.md) | [Türkçe](tr/index.md) | [Deutsch](de/index.md) | [Русский](ru/index.md) | [日本語](ja/index.md) | [中文（简体）](zh/index.md) | [繁體中文](zh-Hant/index.md) | [한국어](ko/index.md) |

Language: English

The names of the family **The clipboard** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State | Probe |
|---|---|---|---|---|---|---|
| `#ClipboardTimeout` | AutoHotkey v1.1.37.02 | directive | -- | -- | value | -- |
| `A_Clipboard` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `Clipboard` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `Clipboard` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `ClipboardAll` | AutoHotkey v2.0.19 | class | -- | -- | value | -- |
| `OnClipboardChange` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value | -- |
| `ClipboardAll.Ptr` | AutoHotkey v2.0.19 | method | ClipboardAll | -- | value | -- |
| `ClipboardAll.Size` | AutoHotkey v2.0.19 | method | ClipboardAll | -- | value | -- |

## What the tree does with it

### A_Clipboard

The text of the clipboard of the desktop. A script may write it, and the concatenation adds to what is there.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the text of the clipboard
- **Errors:** the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.

```ahk
A_Clipboard := "hello"
FileAppend A_Clipboard, "*"
```

### Clipboard

The variable of the clipboard; the v2 spells it A_Clipboard and the v1 spells it Clipboard, and both are the same text. It is read with wl-paste on a Wayland session and with xclip or xsel on an X11 one, and written with wl-copy or xclip.

- **Parameters:** --
- **Answers:** the text on the clipboard
- **Errors:** an empty clipboard is a value and not a failure; an X11 session with neither xclip nor xsel answers "the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine"; a write calls the function registered for OnClipboardChange with 1, because the change the runtime can see is its own.

```ahk
text := A_Clipboard
```

### Clipboard

The variable of the clipboard; the v2 spells it A_Clipboard and the v1 spells it Clipboard, and both are the same text. It is read with wl-paste on a Wayland session and with xclip or xsel on an X11 one, and written with wl-copy or xclip.

- **Parameters:** --
- **Answers:** the text on the clipboard
- **Errors:** an empty clipboard is a value and not a failure; an X11 session with neither xclip nor xsel answers "the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine"; a write calls the function registered for OnClipboardChange with 1, because the change the runtime can see is its own.

```ahk
text := A_Clipboard
```
