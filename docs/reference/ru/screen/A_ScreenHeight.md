# `A_ScreenHeight`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · variable · Захват экрана и пиксельные функции

## Описание

The height of the screen of the desktop, in pixels.

## Синтаксис

```ahk
A_ScreenHeight 
```

## Параметры

nothing; it is a variable, not a call

## Возвращаемое значение

the height, as a number

## Ошибки

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## Пример

```ahk
FileAppend A_ScreenHeight, "*"
```
