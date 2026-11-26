# `FileExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 檔案與目錄操作

## 描述

Answers the attributes of the first entry that the name or the pattern matches. The value of the command is taken with the parenthesis, the way `FileExist("a.txt")` is written.

## 語法

```ahk
Attrib := FileExist(Path)
```

## 參數

- **Path** — A path or a wildcard pattern.

## 傳回值

the attributes as letters: `A` for a file, `D` for a folder, `RA` for a file that cannot be written, and the empty text when there is no such file.

## 錯誤

nothing fails: a name that matches nothing answers the empty text.

## 備註

A wildcard answers the first match in the folder's enumeration order.

## 範例

```ahk
x := FileExist("out.txt")
```

## 相關

[`DirExist`](DirExist.md), [`FileGetAttrib`](FileGetAttrib.md)
