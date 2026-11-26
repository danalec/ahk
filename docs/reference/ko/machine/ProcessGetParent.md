# `ProcessGetParent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 머신, 프로세스, 환경

## 설명

The parent PID of a process.

## 구문

```ahk
N := ProcessGetParent(NameOrPID)
```

## 매개변수

- **NameOrPID** — The target.

## 반환 값

The parent PID.

## 예제

```ahk
pp := ProcessGetParent(pid)
```

## 관련

[`ProcessExist`](ProcessExist.md)
