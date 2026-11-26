# `SetWorkingDir`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 解析器與腳本執行

## 描述

Sets the working directory of the script. A name that is not absolute is a folder under the working directory, the way the dialect says it.

## 語法

```ahk
SetWorkingDir …
```

## 參數

Directory

## 傳回值

the integer 1

## 錯誤

an empty name is refused with "the name of the directory is missing.", and a folder that cannot be entered fails with the reason of the desktop.

## 範例

```ahk
SetWorkingDir "."
```
