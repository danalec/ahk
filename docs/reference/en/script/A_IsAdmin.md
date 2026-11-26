# `A_IsAdmin`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · Parser and script execution

## Description

Whether the script runs with the rights of an administrator: the effective user id, and the root is the one that is zero.

## Syntax

```ahk
A_IsAdmin 
```

## Parameters

nothing; it is a variable, not a call

## Return value

1 when the script is elevated, 0 when it is not

## Errors

the read fails when the machine cannot say; the write is refused.

## Example

```ahk
FileAppend A_IsAdmin, "*"
```
