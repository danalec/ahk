# `FileGetSize`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · File and directory operations

## Description

Answers how big a file is.

## Syntax

```ahk
FileGetSize …
```

## Parameters

FileName, [Unit]

## Return value

the size, rounded down; a folder answers zero.

## Errors

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## Example

```ahk
n := FileGetSize("out.txt", "K")
```
