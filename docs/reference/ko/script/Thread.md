# `Thread`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 파서와 스크립트 실행

## 설명

The state of the thread of the run. The command is "Priority", "Interrupt" or "NoTimers", and the word is read without regard to its case: "Priority" takes the priority of the thread, "Interrupt" takes the milliseconds and the lines of the window of the interruption, and "NoTimers" says whether the timers may run -- the double negative of the v2, where "NoTimers" is zero when the timers may run.

## 구문

```ahk
Thread …
```

## 매개변수

Command, [Value1], [Value2]

## 반환 값

the empty text

## 오류

a command that is not one of the three refuses with "Parameter #1 of Thread is invalid.", and the word is in the `Extra`; a value that is not there leaves the setting as it was.

## 예제

```ahk
Thread "Priority", 5
```
