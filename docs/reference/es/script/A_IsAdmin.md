# `A_IsAdmin`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · El analizador y la ejecución del script

## Descripción

Whether the script runs with the rights of an administrator: the effective user id, and the root is the one that is zero.

## Sintaxis

```ahk
A_IsAdmin 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

1 when the script is elevated, 0 when it is not

## Errores

the read fails when the machine cannot say; the write is refused.

## Ejemplo

```ahk
FileAppend A_IsAdmin, "*"
```
