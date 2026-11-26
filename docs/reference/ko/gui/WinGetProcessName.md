# `WinGetProcessName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · GUI 창과 창 자동화

## 설명

Answers the name of the program of a window; it rides the chain of WinGetPID -- the window names an application on the accessibility bus, the bus answers its process, and the process table answers the name.

## 구문

```ahk
WinGetProcessName …
```

## 매개변수

WinTitle

## 반환 값

the file name of the executable, as text

## 오류

"Target window not found." when no window has that title, "Target process not found." when the process is gone by then; an empty title is refused with "the window is found by its title, and an empty title is not carried yet."

## 예제

```ahk
name := WinGetProcessName("Calculator")
```
