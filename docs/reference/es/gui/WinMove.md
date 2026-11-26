# `WinMove`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Moves and resizes the window the script made; a Wayland compositor places the windows itself and need not honour the place a client asks for.

## Sintaxis

```ahk
WinMove X, Y, W, H, WinTitle
```

## Parámetros

- **X/Y/W/H** — New geometry; omitted keeps.
- **WinTitle** — The criteria.

## Valor de retorno

the empty text

## Errores

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## Ejemplo

```ahk
WinMove 100, 100, 800, 600, "MyWindow"
```

## Relacionados

[`WinGetPos`](WinGetPos.md), [`WinMinimize`](WinMinimize.md)
