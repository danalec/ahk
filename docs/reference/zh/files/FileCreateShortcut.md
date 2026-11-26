# `FileCreateShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

为一个程序创建快捷方式。在这个桌面上，快捷方式是一个 freedesktop 规范的 `.desktop` 文件；即使目标不存在也会创建。

## 语法

```ahk
FileCreateShortcut …
```

## 参数

Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]

## 返回值

无返回值

## 错误

Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

## 示例

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```
