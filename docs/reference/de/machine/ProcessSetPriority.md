# `ProcessSetPriority`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

Sets the priority of a process.

## Syntax

```ahk
ProcessSetPriority Priority, NameOrPID
```

## Parameter

- **Priority** — L/B/N/A/H/R or a number.
- **NameOrPID** — The target.

## Rückgabewert

Nothing.

## Beispiel

```ahk
ProcessSetPriority "H", pid
```

## Verwandt

[`Process`](Process.md)
