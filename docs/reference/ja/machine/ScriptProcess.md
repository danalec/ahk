# `ScriptProcess`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*KeySharp* · class · マシン、プロセス、環境

## 説明

The class of a running script process (ks): HasExited/ExitCode/ExitTime/Kill/StdOut/StdErr/StdIn.

## 構文

```ahk
P := RunScript(...)
```

## パラメータ

?.

## 戻り値

A ScriptProcess object.

## 例

```ahk
p := RunScript("c.ahk")
```

## 関連

[`RunScript`](RunScript.md), [`HasExited`](HasExited.md), [`Kill`](Kill.md)
