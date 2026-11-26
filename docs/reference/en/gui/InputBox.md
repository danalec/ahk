# `InputBox`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Asks the user for a text in a dialog of the desktop and answers an object with the text in its Value and the way the dialog was closed in its Result.

## Syntax

```ahk
Result := InputBox(Prompt, Title, Options, Default)
```

## Parameters

- **Prompt** — The question.
- **Title** — The window title.
- **Options** — Password/size/timeout.
- **Default** — The initial text.

## Return value

an object with the properties Value and Result, where Result is "OK", "Cancel" or "Timeout"

## Errors

an option the dialect does not know is refused with "Invalid option.".

## Example

```ahk
answer := InputBox("your name?", "hello")
MsgBox answer.Value
```

## Related

[`MsgBox`](MsgBox.md), [`FileSelect`](FileSelect.md)
