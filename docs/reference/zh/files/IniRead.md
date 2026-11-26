# `IniRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

读取 ini 文件中某个键的值。不给节名时回答所有节名；不给键名时回答整个节。ini 文件按 Windows 的写法：`[section]`、`key=value`、`;` 是注释。

## 语法

```ahk
V := IniRead(Filename, Section, Key, Default)
```

## 参数

- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key; omitted reads the whole section.
- **Default** — Returned when absent.

## 返回值

键的值；键不存在而脚本给了默认值时，回答默认值。

## 错误

Error `The requested key, section or file was not found.` when the key is not there and no default is given.

## 示例

```ahk
v := IniRead("app.ini", "Main", "Name")
```

## 相关

[`IniWrite`](IniWrite.md), [`IniDelete`](IniDelete.md)
