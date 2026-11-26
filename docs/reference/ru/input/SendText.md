# `SendText`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Горячие клавиши, горячие строки и ввод

## Описание

Types the text as it stands: the braces of v2 are not read here, and the text goes to the keyboard one character at a time.

## Синтаксис

```ahk
SendText …
```

## Параметры

Text

## Возвращаемое значение

nothing

## Ошибки

the keyboard of the backend is a Linux uinput device, so on a host that is not Linux the text is not typed.

## Пример

```ahk
SendText "a literal {braced} text"
```
