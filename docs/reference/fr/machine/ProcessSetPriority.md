# `ProcessSetPriority`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · La machine, les processus et l'environnement

## Description

Sets the priority of a process.

## Syntaxe

```ahk
ProcessSetPriority Priority, NameOrPID
```

## Paramètres

- **Priority** — L/B/N/A/H/R or a number.
- **NameOrPID** — The target.

## Valeur de retour

Nothing.

## Exemple

```ahk
ProcessSetPriority "H", pid
```

## Voir aussi

[`Process`](Process.md)
