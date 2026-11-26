# `WinWaitActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Waits for a window whose title matches to have the focus, and answers the identifier of it.

## 구문

```ahk
WinWaitActive …
```

## 매개변수

WinTitle, WinText, [Timeout], [ExcludeTitle], [ExcludeText]

## 반환 값

the identifier of the window as text, or 0 when the timeout ran out

## 오류

a session that is not Wayland answers "this desktop waits for the windows of a Wayland session, and this session is not Wayland"; the two exclusions are refused with "the two exclusions are not carried yet."

## 예제

```ahk
WinWaitActive "Calculator", , 5
```
