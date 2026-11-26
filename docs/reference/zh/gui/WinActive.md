# `WinActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · GUI 窗口与窗口自动化

## 描述

取得拥有焦点的窗口。后端是 Wayland 会话，合成器才知道有哪些窗口以及哪个在前。

## 语法

```ahk
WinActive …
```

## 参数

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 返回值

活动窗口的标识，没有时为 0

## 错误

the text of the window and the exclusions are not carried yet; a compositor that does not publish the focus answers with the reason, and not with a guess.

## 示例

```ahk
id := WinActive()
```
