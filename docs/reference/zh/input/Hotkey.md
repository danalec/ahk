# `Hotkey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

注册一个热键：监视的按键和要运行的函数。注册表归运行时，监视器归机器：脚本说明热键运行哪个函数，机器说明按下了哪个键。

## 语法

```ahk
Hotkey KeyName, Action, Options
```

## 参数

- **KeyName** — e.g. ^!a, ~LButton.
- **Action** — A function/label, or On/Off/Toggle/AltTab etc.
- **Options** — On/Off/UseErrorLevel/MaxThreadsPerHotkey.

## 返回值

无返回值

## 错误

a hotkey without an action is 'Nonexistent hotkey.'; the watcher reads the physical keyboard of the machine, so on a host that is not Linux the registration fails with 'the hotkeys of this box watch a Linux device; this is not Linux'.

## 备注

On Linux the keyboard is watched (/dev/input or X11); on Windows RegisterHotKey or the hook.

## 示例

```ahk
Hotkey "^s", SaveTheFile
```

## 相关

[`Hotstring`](Hotstring.md), [`#HotkeyInterval`](#HotkeyInterval.md), [`KeyWait`](KeyWait.md)
