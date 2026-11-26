# `InputBox`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

在桌面的对话框里向用户要一段文本，返回一个对象：Value 是输入的文本，Result 是对话框被关闭的方式。

## 语法

```ahk
Result := InputBox(Prompt, Title, Options, Default)
```

## 参数

- **Prompt** — The question.
- **Title** — The window title.
- **Options** — Password/size/timeout.
- **Default** — The initial text.

## 返回值

带 Value 和 Result 属性的对象，Result 是 "OK"、"Cancel" 或 "Timeout"

## 错误

an option the dialect does not know is refused with "Invalid option.".

## 示例

```ahk
answer := InputBox("your name?", "hello")
MsgBox answer.Value
```

## 相关

[`MsgBox`](MsgBox.md), [`FileSelect`](FileSelect.md)
