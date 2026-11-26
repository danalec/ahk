# `ProcessGetParent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · マシン、プロセス、環境

## 説明

The parent PID of a process.

## 構文

```ahk
N := ProcessGetParent(NameOrPID)
```

## パラメータ

- **NameOrPID** — The target.

## 戻り値

The parent PID.

## 例

```ahk
pp := ProcessGetParent(pid)
```

## 関連

[`ProcessExist`](ProcessExist.md)
