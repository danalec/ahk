# `Clipboard`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 剪贴板

## 描述

剪贴板的变量；v2 写作 A_Clipboard，v1 写作 Clipboard，两者指的是同一份文本。Wayland 会话用 wl-paste 读，X11 会话用 xclip 或 xsel 读，用 wl-copy 或 xclip 写。

## 语法

```ahk
Clipboard 
```

## 参数

--

## 返回值

剪贴板上的文本

## 错误

an empty clipboard is a value and not a failure; an X11 session with neither xclip nor xsel answers "the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine"; a write calls the function registered for OnClipboardChange with 1, because the change the runtime can see is its own.

## 示例

```ahk
text := A_Clipboard
```
