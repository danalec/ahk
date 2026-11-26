# `A_Cursor`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · Hotkeys, hotstrings y entrada

## Descripción

The shape of the cursor of the pointer. The shape belongs to the machine, and the machine is asked for it.

## Sintaxis

```ahk
A_Cursor 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the name of the shape, as a text ("Arrow", "IBeam", "Wait", ...)

## Errores

the read fails with the reason of the desktop when the desktop cannot tell the shape of the cursor; the write is refused, because the name is a built-in variable that the dialect does not let a script write.

## Ejemplo

```ahk
FileAppend A_Cursor, "*"
```
