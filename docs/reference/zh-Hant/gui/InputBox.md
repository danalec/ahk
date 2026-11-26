# `InputBox`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Asks the user for a text in a dialog of the desktop and answers an object with the text in its Value and the way the dialog was closed in its Result.

## 語法

```ahk
Result := InputBox(Prompt, Title, Options, Default)
```

## 參數

- **Prompt** — The question.
- **Title** — The window title.
- **Options** — Password/size/timeout.
- **Default** — The initial text.

## 傳回值

an object with the properties Value and Result, where Result is "OK", "Cancel" or "Timeout"

## 錯誤

an option the dialect does not know is refused with "Invalid option.".

## 範例

```ahk
answer := InputBox("your name?", "hello")
MsgBox answer.Value
```

## 相關

[`MsgBox`](MsgBox.md), [`FileSelect`](FileSelect.md)
