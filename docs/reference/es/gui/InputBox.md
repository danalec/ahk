# `InputBox`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Asks the user for a text in a dialog of the desktop and answers an object with the text in its Value and the way the dialog was closed in its Result.

## Sintaxis

```ahk
Result := InputBox(Prompt, Title, Options, Default)
```

## Parámetros

- **Prompt** — The question.
- **Title** — The window title.
- **Options** — Password/size/timeout.
- **Default** — The initial text.

## Valor de retorno

an object with the properties Value and Result, where Result is "OK", "Cancel" or "Timeout"

## Errores

an option the dialect does not know is refused with "Invalid option.".

## Ejemplo

```ahk
answer := InputBox("your name?", "hello")
MsgBox answer.Value
```

## Relacionados

[`MsgBox`](MsgBox.md), [`FileSelect`](FileSelect.md)
