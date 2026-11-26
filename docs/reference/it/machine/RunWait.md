# `RunWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · La macchina, i processi e l'ambiente

## Descrizione

Runs a command and waits for it, the same way Run does. The name is the function form of the same handler.

## Sintassi

```ahk
ExitCode := RunWait(Target, WorkingDir, Options, OutputVarPID)
```

## Parametri

- **Target** — The program or document.
- **WorkingDir** — The child's directory.
- **Options** — Min/Max/Hide.
- **OutputVarPID** — Receives the PID.

## Valore di ritorno

the exit code of the command

## Errori

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Osservazioni

Blocks the script until the child exits; timers do not run during the wait on this tree.

## Esempio

```ahk
RunWait "ls"
```

## Correlati

[`Run`](Run.md), [`Process`](Process.md)
