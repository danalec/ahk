# `IniWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

Writes a key of an ini file. The value comes first and the name of the file second, the way the binary of v2.0.31 takes them; a file that is not there is made.

## 語法

```ahk
IniWrite Value, Filename, Section, Key
```

## 參數

- **Value** — The text.
- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key.

## 傳回值

nothing

## 錯誤

Error when the file cannot be written.

## 範例

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

## 相關

[`IniRead`](IniRead.md), [`IniDelete`](IniDelete.md)
