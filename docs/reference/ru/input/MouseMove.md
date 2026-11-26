# `MouseMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Горячие клавиши, горячие строки и ввод

## Описание

Moves the pointer of the backend to a place on the screen. The box moves the pointer to the place, and not from where it is.

## Синтаксис

```ahk
MouseMove X, Y, Speed
```

## Параметры

- **X/Y** — Screen coords (or relative with R).
- **Speed** — 0..100.

## Возвращаемое значение

nothing

## Ошибки

the relative place of the move and the speed of the move are refused as not carried yet.

## Пример

```ahk
MouseMove 400, 300
```

## См. также

[`Click`](Click.md), [`MouseGetPos`](MouseGetPos.md)
