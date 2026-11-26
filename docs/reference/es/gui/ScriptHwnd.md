# `ScriptHwnd`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · variable · Ventanas de la GUI y automatización de ventanas

## Descripción

The handle of the hidden window of the script, the name the v2 gives it. The window is the one that owns the messages of the session, and the number changes with the run.

## Sintaxis

```ahk
ScriptHwnd 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the handle of the window, as a number

## Errores

the read fails with the reason of the desktop when the desktop has no window of the script; the write is refused.

## Ejemplo

```ahk
FileAppend A_ScriptHwnd, "*"
```
