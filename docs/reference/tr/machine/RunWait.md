# `RunWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Makine, süreçler ve ortam

## Açıklama

Runs a command and waits for it, the same way Run does. The name is the function form of the same handler.

## Sözdizimi

```ahk
ExitCode := RunWait(Target, WorkingDir, Options, OutputVarPID)
```

## Parametreler

- **Target** — The program or document.
- **WorkingDir** — The child's directory.
- **Options** — Min/Max/Hide.
- **OutputVarPID** — Receives the PID.

## Dönüş değeri

the exit code of the command

## Hatalar

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Notlar

Blocks the script until the child exits; timers do not run during the wait on this tree.

## Örnek

```ahk
RunWait "ls"
```

## İlgili

[`Run`](Run.md), [`Process`](Process.md)
