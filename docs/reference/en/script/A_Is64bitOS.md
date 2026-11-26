# `A_Is64bitOS`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · Parser and script execution

## Description

Whether the machine is a 64-bit one. The tree answers with the width of its own pointer: 1 on a 64-bit build of the runtime, 0 on a 32-bit one.

## Syntax

```ahk
A_Is64bitOS 
```

## Parameters

nothing; it is a variable, not a call

## Return value

1 or 0

## Errors

nothing to read; the write is refused.

## Example

```ahk
FileAppend A_Is64bitOS, "*"
```
