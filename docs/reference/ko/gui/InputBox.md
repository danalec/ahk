# `InputBox`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Asks the user for a text in a dialog of the desktop and answers an object with the text in its Value and the way the dialog was closed in its Result.

## 구문

```ahk
Result := InputBox(Prompt, Title, Options, Default)
```

## 매개변수

- **Prompt** — The question.
- **Title** — The window title.
- **Options** — Password/size/timeout.
- **Default** — The initial text.

## 반환 값

an object with the properties Value and Result, where Result is "OK", "Cancel" or "Timeout"

## 오류

an option the dialect does not know is refused with "Invalid option.".

## 예제

```ahk
answer := InputBox("your name?", "hello")
MsgBox answer.Value
```

## 관련

[`MsgBox`](MsgBox.md), [`FileSelect`](FileSelect.md)
