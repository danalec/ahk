# `RunWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 機器、行程與環境

## 描述

Runs a command and waits for it, the same way Run does. The name is the function form of the same handler.

## 語法

```ahk
ExitCode := RunWait(Target, WorkingDir, Options, OutputVarPID)
```

## 參數

- **Target** — The program or document.
- **WorkingDir** — The child's directory.
- **Options** — Min/Max/Hide.
- **OutputVarPID** — Receives the PID.

## 傳回值

the exit code of the command

## 錯誤

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## 備註

Blocks the script until the child exits; timers do not run during the wait on this tree.

## 範例

```ahk
RunWait "ls"
```

## 相關

[`Run`](Run.md), [`Process`](Process.md)
