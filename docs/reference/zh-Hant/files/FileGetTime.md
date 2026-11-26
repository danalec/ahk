# `FileGetTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

Answers when a file was made, when it was changed, or when it was read.

## 語法

```ahk
FileGetTime …
```

## 參數

FileName, [Which]

## 傳回值

the stamp of v2, the fourteen digits of `YYYYMMDDHH24MISS`, in the local time of the machine.

## 錯誤

ValueError when `Which` is not `M`, `C`, `A` or `CUT`; OSError when the file cannot be read.

## 範例

```ahk
t := FileGetTime("out.txt", "M")
```
