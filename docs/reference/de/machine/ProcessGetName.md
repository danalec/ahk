# `ProcessGetName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

Answers the name of the executable of a process; the name is the file name of /proc/<pid>/exe, and the name the kernel gives is the fallback.

## Syntax

```ahk
ProcessGetName …
```

## Parameter

[ProcessNameOrPID]

## Rückgabewert

the name of the executable, as text

## Fehler

"Target process not found." when the process is not there; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## Beispiel

```ahk
name := ProcessGetName(1234)
```
