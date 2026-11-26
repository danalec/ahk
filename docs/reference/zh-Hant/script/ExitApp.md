# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 解析器與腳本執行

## 描述

Ends the script with a code. The code goes into the exit_code of the report.

## 語法

```ahk
ExitApp ExitCode
```

## 參數

- **ExitCode** — The code the process exits with (default 0).

## 傳回值

the code

## 錯誤

none.

## 備註

OnExit callbacks run before the process ends.

## 範例

```ahk
ExitApp 3
```

## 相關

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
