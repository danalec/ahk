# `ScriptProcess`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*KeySharp* · class · 머신, 프로세스, 환경

## 설명

The class of a running script process (ks): HasExited/ExitCode/ExitTime/Kill/StdOut/StdErr/StdIn.

## 구문

```ahk
P := RunScript(...)
```

## 매개변수

?.

## 반환 값

A ScriptProcess object.

## 예제

```ahk
p := RunScript("c.ahk")
```

## 관련

[`RunScript`](RunScript.md), [`HasExited`](HasExited.md), [`Kill`](Kill.md)
