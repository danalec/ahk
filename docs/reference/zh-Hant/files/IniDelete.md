# `IniDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

Takes a key of an ini file away. A key that is not given takes the whole section with it.

## 語法

```ahk
IniDelete …
```

## 參數

FileName, Section, [Key]

## 傳回值

nothing

## 錯誤

Error when the file cannot be written.

## 範例

```ahk
IniDelete "app.ini", "Main", "Name"
```
