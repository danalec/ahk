# `FileRecycle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

把一个文件、一个目录，或某个模式匹配到的文件送进机器的回收站。这个盒子用的是 XDG 规范的回收站。

## 语法

```ahk
FileRecycle …
```

## 参数

FileName

## 返回值

无返回值

## 错误

Error `Failed` when the path is not there; a pattern that matches nothing is not a failure; ValueError when the name is empty; Error when the machine does not say where the home directory of the user is.

## 示例

```ahk
FileRecycle "out.txt"
```
