# `IniWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

写入 ini 文件的一个键。参数顺序与 v2.0.31 的二进制一致：先是值，后是文件名；文件不存在时会被创建。

## 语法

```ahk
IniWrite Value, Filename, Section, Key
```

## 参数

- **Value** — The text.
- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key.

## 返回值

无返回值

## 错误

Error when the file cannot be written.

## 示例

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

## 相关

[`IniRead`](IniRead.md), [`IniDelete`](IniDelete.md)
