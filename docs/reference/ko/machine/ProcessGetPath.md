# `ProcessGetPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 머신, 프로세스, 환경

## 설명

Answers the whole path of the executable of a process; the path is the target of the link /proc/<pid>/exe, and the " (deleted)" the kernel adds is cut.

## 구문

```ahk
ProcessGetPath …
```

## 매개변수

[ProcessNameOrPID]

## 반환 값

the path of the executable, as text

## 오류

"Target process not found." when the process is not there; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## 예제

```ahk
path := ProcessGetPath(1234)
```
