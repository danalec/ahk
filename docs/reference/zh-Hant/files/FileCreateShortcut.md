# `FileCreateShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

Makes a shortcut to a program. On this desktop a shortcut is a `.desktop` file of the freedesktop specification, and it is made even for a target that is not there.

## 語法

```ahk
FileCreateShortcut …
```

## 參數

Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]

## 傳回值

nothing

## 錯誤

Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

## 範例

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```
