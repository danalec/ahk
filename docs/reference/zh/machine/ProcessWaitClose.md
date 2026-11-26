# `ProcessWaitClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 机器、进程与环境

## 描述

等待一个已经在的进程结束；从未存在过的进程就是已经没了，等待立刻结束。

## 语法

```ahk
ProcessWaitClose …
```

## 参数

ProcessNameOrPID, [Timeout]

## 返回值

进程已消失为 0，超时时它还在则给出它的 pid

## 错误

a call without a timeout waits for ever; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## 示例

```ahk
ProcessWaitClose "firefox", 10
```
