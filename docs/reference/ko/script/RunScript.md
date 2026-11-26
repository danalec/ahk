# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*KeySharp* · function · 파서와 스크립트 실행

## 설명

Starts a script in a new process and answers a ScriptProcess (ks).

## 구문

```ahk
P := RunScript(Path, Args)
```

## 매개변수

- **Path** — The script.
- **Args** — The arguments.

## 반환 값

A ScriptProcess.

## 예제

```ahk
p := RunScript("child.ahk")
```

## 관련

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
