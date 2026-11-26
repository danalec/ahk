# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*KeySharp* · function · 解析器與腳本執行

## 描述

Starts a script in a new process and answers a ScriptProcess (ks).

## 語法

```ahk
P := RunScript(Path, Args)
```

## 參數

- **Path** — The script.
- **Args** — The arguments.

## 傳回值

A ScriptProcess.

## 範例

```ahk
p := RunScript("child.ahk")
```

## 相關

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
