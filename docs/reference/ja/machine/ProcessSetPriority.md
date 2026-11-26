# `ProcessSetPriority`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · マシン、プロセス、環境

## 説明

Sets the priority of a process.

## 構文

```ahk
ProcessSetPriority Priority, NameOrPID
```

## パラメータ

- **Priority** — L/B/N/A/H/R or a number.
- **NameOrPID** — The target.

## 戻り値

Nothing.

## 例

```ahk
ProcessSetPriority "H", pid
```

## 関連

[`Process`](Process.md)
