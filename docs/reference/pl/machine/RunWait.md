# `RunWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Maszyna, procesy i środowisko

## Opis

Runs a command and waits for it, the same way Run does. The name is the function form of the same handler.

## Składnia

```ahk
ExitCode := RunWait(Target, WorkingDir, Options, OutputVarPID)
```

## Parametry

- **Target** — The program or document.
- **WorkingDir** — The child's directory.
- **Options** — Min/Max/Hide.
- **OutputVarPID** — Receives the PID.

## Wartość zwracana

the exit code of the command

## Błędy

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Uwagi

Blocks the script until the child exits; timers do not run during the wait on this tree.

## Przykład

```ahk
RunWait "ls"
```

## Powiązane

[`Run`](Run.md), [`Process`](Process.md)
