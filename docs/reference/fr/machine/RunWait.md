# `RunWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · La machine, les processus et l'environnement

## Description

Runs a command and waits for it, the same way Run does. The name is the function form of the same handler.

## Syntaxe

```ahk
ExitCode := RunWait(Target, WorkingDir, Options, OutputVarPID)
```

## Paramètres

- **Target** — The program or document.
- **WorkingDir** — The child's directory.
- **Options** — Min/Max/Hide.
- **OutputVarPID** — Receives the PID.

## Valeur de retour

the exit code of the command

## Erreurs

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Remarques

Blocks the script until the child exits; timers do not run during the wait on this tree.

## Exemple

```ahk
RunWait "ls"
```

## Voir aussi

[`Run`](Run.md), [`Process`](Process.md)
