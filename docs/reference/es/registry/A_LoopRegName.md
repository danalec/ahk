# `A_LoopRegName`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · API del registro

## Descripción

The name of the registry key the registry loop is on. The registry is a thing of Windows and this runtime has none: the registry loop fails with "the registry is a Windows thing and this box has none.", and the name keeps the empty text.

## Sintaxis

```ahk
A_LoopRegName 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the empty text; the name is never written by the tree

## Errores

nothing to read; the write is refused.

## Ejemplo

```ahk
FileAppend A_LoopRegName, "*"
```
