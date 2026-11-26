# `DirMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 文件与目录操作

## 描述

移动一个目录及其中的全部内容。目标已经存在时与来源合并：来源里的文件优先。

## 语法

```ahk
DirMove …
```

## 参数

Source, Dest, [Overwrite]

## 返回值

无返回值

## 错误

Error `Failed` when the source is not a directory; a name that holds a wildcard is refused, and the reference fails with `Failed` on a wildcard too.

## 示例

```ahk
DirMove "src", "dst"
```
