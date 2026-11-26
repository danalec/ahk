# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파서와 스크립트 실행

## 설명

Ends the script with a code. The code goes into the exit_code of the report.

## 구문

```ahk
ExitApp ExitCode
```

## 매개변수

- **ExitCode** — The code the process exits with (default 0).

## 반환 값

the code

## 오류

none.

## 설명

OnExit callbacks run before the process ends.

## 예제

```ahk
ExitApp 3
```

## 관련

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
