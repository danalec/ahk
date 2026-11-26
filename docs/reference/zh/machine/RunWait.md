# `RunWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 机器、进程与环境

## 描述

运行一条命令并等待它结束，与 Run 相同。这个名字是同一个处理器的函数形式。

## 语法

```ahk
ExitCode := RunWait(Target, WorkingDir, Options, OutputVarPID)
```

## 参数

- **Target** — The program or document.
- **WorkingDir** — The child's directory.
- **Options** — Min/Max/Hide.
- **OutputVarPID** — Receives the PID.

## 返回值

命令的退出码

## 错误

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## 备注

Blocks the script until the child exits; timers do not run during the wait on this tree.

## 示例

```ahk
RunWait "ls"
```

## 相关

[`Run`](Run.md), [`Process`](Process.md)
