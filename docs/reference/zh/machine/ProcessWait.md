# `ProcessWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 机器、进程与环境

## 描述

等待某个进程出现；盒子每百毫秒检查一次，和 v2 的二进制一样。

## 语法

```ahk
ProcessWait …
```

## 参数

ProcessNameOrPID, [Timeout]

## 返回值

出现时给出该进程的 pid，超时则为 0

## 错误

a call without a timeout waits for ever; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## 示例

```ahk
ProcessWait "firefox", 10
```
