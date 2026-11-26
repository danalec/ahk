# `RunWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 머신, 프로세스, 환경

## 설명

Runs a command and waits for it, the same way Run does. The name is the function form of the same handler.

## 구문

```ahk
ExitCode := RunWait(Target, WorkingDir, Options, OutputVarPID)
```

## 매개변수

- **Target** — The program or document.
- **WorkingDir** — The child's directory.
- **Options** — Min/Max/Hide.
- **OutputVarPID** — Receives the PID.

## 반환 값

the exit code of the command

## 오류

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## 설명

Blocks the script until the child exits; timers do not run during the wait on this tree.

## 예제

```ahk
RunWait "ls"
```

## 관련

[`Run`](Run.md), [`Process`](Process.md)
