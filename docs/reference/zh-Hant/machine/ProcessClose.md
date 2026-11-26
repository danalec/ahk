# `ProcessClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 機器、行程與環境

## 描述

Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.

## 語法

```ahk
ProcessClose NameOrPID
```

## 參數

- **NameOrPID** — The name or PID.

## 傳回值

the pid of the process that was closed, 0 when it was not there or could not be closed

## 錯誤

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## 範例

```ahk
ProcessClose "firefox"
```

## 相關

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
