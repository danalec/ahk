# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Where the window is and how big it is, as the compositor reports it.

## 구문

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## 매개변수

- **Output*** — Variables.
- **WinTitle** — The criteria.

## 반환 값

nothing; the place and the size are written into the variables of the caller

## 오류

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## 예제

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## 관련

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
