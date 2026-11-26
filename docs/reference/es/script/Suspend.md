# `Suspend`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · El analizador y la ejecución del script

## Descripción

Turns the hotkeys of the script off or on. With no parameter it turns them the other way. The state is visible in A_IsSuspended and in the enabled flag of the hotkey table.

## Sintaxis

```ahk
Suspend …
```

## Parámetros

[OnOff]

## Valor de retorno

the empty text

## Errores

a parameter that is not 0, 1 or -1 is an invalid parameter.

## Ejemplo

```ahk
Suspend true
```
