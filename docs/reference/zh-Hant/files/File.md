# `File`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · method · `Error` · 檔案與目錄操作

## 描述

The class of an open file of the machine. The class is not made: `File(...)` answers "Invalid base."; the file is made by the `FileOpen` built-in, and it is the only way to have one.

## 語法

```ahk
File 
```

## 參數

nothing that makes it; the arguments of `FileOpen` open the file

## 傳回值

the object of the class `File`

## 錯誤

the call of the class is refused with "Invalid base.".

## 範例

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```
