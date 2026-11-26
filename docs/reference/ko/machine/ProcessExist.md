# `ProcessExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 머신, 프로세스, 환경

## 설명

Answers whether a process is there, and with which pid; the processes of the machine are read out of /proc.

## 구문

```ahk
PID := ProcessExist(NameOrPID)
```

## 매개변수

- **NameOrPID** — The name or a PID.

## 반환 값

the pid of the process, or 0 when there is none

## 오류

a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none"; a process that is not there is zero and not a failure.

## 예제

```ahk
if ProcessExist("firefox")
```

## 관련

[`ProcessClose`](ProcessClose.md), [`ProcessWait`](ProcessWait.md), [`Run`](Run.md)
