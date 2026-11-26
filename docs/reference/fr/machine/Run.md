# `Run`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · La machine, les processus et l'environnement

## Description

Runs a command and waits for it. What the command wrote on its output goes into the output of the report.

## Syntaxe

```ahk
Run Target, WorkingDir, Options, OutputVarPID
```

## Paramètres

- **Target** — A program, document, URL or verb.
- **WorkingDir** — The working directory of the child.
- **Options** — Min/Max/Hide/UseErrorLevel.
- **OutputVarPID** — A variable that receives the process id.

## Valeur de retour

the exit code of the command

## Erreurs

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Remarques

On Linux it is sh -c; on Windows ShellExecute/CreateProcess. The child outlives the script unless killed.

## Exemple

```ahk
Run "ls"
```

## Voir aussi

[`RunWait`](RunWait.md), [`Process`](Process.md), [`WinWait`](WinWait.md)
