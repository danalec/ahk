# `ProcessGetPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 机器、进程与环境

## 描述

给出进程可执行文件的完整路径；路径是 /proc/<pid>/exe 这个链接指向的地方，内核加上的 " (deleted)" 会被截掉。

## 语法

```ahk
ProcessGetPath …
```

## 参数

[ProcessNameOrPID]

## 返回值

可执行文件的路径（文本）

## 错误

"Target process not found." when the process is not there; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## 示例

```ahk
path := ProcessGetPath(1234)
```
