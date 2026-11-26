# `InputBox`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Окна интерфейса и автоматизация окон

## Описание

Asks the user for a text in a dialog of the desktop and answers an object with the text in its Value and the way the dialog was closed in its Result.

## Синтаксис

```ahk
Result := InputBox(Prompt, Title, Options, Default)
```

## Параметры

- **Prompt** — The question.
- **Title** — The window title.
- **Options** — Password/size/timeout.
- **Default** — The initial text.

## Возвращаемое значение

an object with the properties Value and Result, where Result is "OK", "Cancel" or "Timeout"

## Ошибки

an option the dialect does not know is refused with "Invalid option.".

## Пример

```ahk
answer := InputBox("your name?", "hello")
MsgBox answer.Value
```

## См. также

[`MsgBox`](MsgBox.md), [`FileSelect`](FileSelect.md)
