# `ProcessClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · マシン、プロセス、環境

## 説明

Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.

## 構文

```ahk
ProcessClose NameOrPID
```

## パラメータ

- **NameOrPID** — The name or PID.

## 戻り値

the pid of the process that was closed, 0 when it was not there or could not be closed

## エラー

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## 例

```ahk
ProcessClose "firefox"
```

## 関連

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
