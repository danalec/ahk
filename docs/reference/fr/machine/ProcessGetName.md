# `ProcessGetName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · La machine, les processus et l'environnement

## Description

Answers the name of the executable of a process; the name is the file name of /proc/<pid>/exe, and the name the kernel gives is the fallback.

## Syntaxe

```ahk
ProcessGetName …
```

## Paramètres

[ProcessNameOrPID]

## Valeur de retour

the name of the executable, as text

## Erreurs

"Target process not found." when the process is not there; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## Exemple

```ahk
name := ProcessGetName(1234)
```
