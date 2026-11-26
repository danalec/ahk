# `ProcessClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 머신, 프로세스, 환경

## 설명

Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.

## 구문

```ahk
ProcessClose NameOrPID
```

## 매개변수

- **NameOrPID** — The name or PID.

## 반환 값

the pid of the process that was closed, 0 when it was not there or could not be closed

## 오류

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## 예제

```ahk
ProcessClose "firefox"
```

## 관련

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
