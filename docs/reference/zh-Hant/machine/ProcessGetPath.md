# `ProcessGetPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 機器、行程與環境

## 描述

Answers the whole path of the executable of a process; the path is the target of the link /proc/<pid>/exe, and the " (deleted)" the kernel adds is cut.

## 語法

```ahk
ProcessGetPath …
```

## 參數

[ProcessNameOrPID]

## 傳回值

the path of the executable, as text

## 錯誤

"Target process not found." when the process is not there; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## 範例

```ahk
path := ProcessGetPath(1234)
```
