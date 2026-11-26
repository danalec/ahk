# `ScriptProcess`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*KeySharp* · class · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

The class of a running script process (ks): HasExited/ExitCode/ExitTime/Kill/StdOut/StdErr/StdIn.

## Syntax

```ahk
P := RunScript(...)
```

## Parameter

?.

## Rückgabewert

A ScriptProcess object.

## Beispiel

```ahk
p := RunScript("c.ahk")
```

## Verwandt

[`RunScript`](RunScript.md), [`HasExited`](HasExited.md), [`Kill`](Kill.md)
