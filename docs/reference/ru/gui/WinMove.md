# `WinMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Окна интерфейса и автоматизация окон

## Описание

Moves and resizes the window the script made; a Wayland compositor places the windows itself and need not honour the place a client asks for.

## Синтаксис

```ahk
WinMove X, Y, W, H, WinTitle
```

## Параметры

- **X/Y/W/H** — New geometry; omitted keeps.
- **WinTitle** — The criteria.

## Возвращаемое значение

the empty text

## Ошибки

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## Пример

```ahk
WinMove 100, 100, 800, 600, "MyWindow"
```

## См. также

[`WinGetPos`](WinGetPos.md), [`WinMinimize`](WinMinimize.md)
