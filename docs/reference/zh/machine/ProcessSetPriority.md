# `ProcessSetPriority`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 机器、进程与环境

## 描述

Sets the priority of a process.

## 语法

```ahk
ProcessSetPriority Priority, NameOrPID
```

## 参数

- **Priority** — L/B/N/A/H/R or a number.
- **NameOrPID** — The target.

## 返回值

Nothing.

## 示例

```ahk
ProcessSetPriority "H", pid
```

## 相关

[`Process`](Process.md)
