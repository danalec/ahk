# `ProcessExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 机器、进程与环境

## 描述

回答某个进程是否存在、它的 pid 是多少；机器上的进程是从 /proc 读出来的。

## 语法

```ahk
PID := ProcessExist(NameOrPID)
```

## 参数

- **NameOrPID** — The name or a PID.

## 返回值

该进程的 pid，不存在则为 0

## 错误

a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none"; a process that is not there is zero and not a failure.

## 示例

```ahk
if ProcessExist("firefox")
```

## 相关

[`ProcessClose`](ProcessClose.md), [`ProcessWait`](ProcessWait.md), [`Run`](Run.md)
