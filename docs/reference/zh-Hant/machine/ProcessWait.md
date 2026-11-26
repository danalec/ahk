# `ProcessWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 機器、行程與環境

## 描述

Waits for a process to be there; the box checks every hundred milliseconds, the way the binary of v2 does.

## 語法

```ahk
ProcessWait …
```

## 參數

ProcessNameOrPID, [Timeout]

## 傳回值

the pid of the process when it appears, 0 at the timeout

## 錯誤

a call without a timeout waits for ever; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## 範例

```ahk
ProcessWait "firefox", 10
```
