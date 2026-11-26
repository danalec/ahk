# `ProcessSetPriority`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 機器、行程與環境

## 描述

Sets the priority of a process.

## 語法

```ahk
ProcessSetPriority Priority, NameOrPID
```

## 參數

- **Priority** — L/B/N/A/H/R or a number.
- **NameOrPID** — The target.

## 傳回值

Nothing.

## 範例

```ahk
ProcessSetPriority "H", pid
```

## 相關

[`Process`](Process.md)
