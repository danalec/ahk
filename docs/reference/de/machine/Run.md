# `Run`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

Runs a command and waits for it. What the command wrote on its output goes into the output of the report.

## Syntax

```ahk
Run Target, WorkingDir, Options, OutputVarPID
```

## Parameter

- **Target** — A program, document, URL or verb.
- **WorkingDir** — The working directory of the child.
- **Options** — Min/Max/Hide/UseErrorLevel.
- **OutputVarPID** — A variable that receives the process id.

## Rückgabewert

the exit code of the command

## Fehler

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Bemerkungen

On Linux it is sh -c; on Windows ShellExecute/CreateProcess. The child outlives the script unless killed.

## Beispiel

```ahk
Run "ls"
```

## Verwandt

[`RunWait`](RunWait.md), [`Process`](Process.md), [`WinWait`](WinWait.md)
