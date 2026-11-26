# `A_SendMode`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · Hotkeys, hotstrings y entrada

## Descripción

The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.

## Sintaxis

```ahk
A_SendMode 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the name of the mode, as a text

## Errores

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## Ejemplo

```ahk
FileAppend A_SendMode, "*"
```
