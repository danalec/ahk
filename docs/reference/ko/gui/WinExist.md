# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · GUI 창과 창 자동화

## 설명

Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.

## 구문

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## 매개변수

- **WinTitle** — The criteria.

## 반환 값

the identifier of the window that is there, zero when there is none

## 오류

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## 설명

Sets the last-found window for the Win* family.

## 예제

```ahk
if WinExist("Untitled")
```

## 관련

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
