# `WinShow`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Puts the window of the script back on the screen, after a `WinHide`.

## 구문

```ahk
WinShow …
```

## 매개변수

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 반환 값

nothing

## 오류

a window that is not there answers 'Target window not found.'; a window of another program cannot be shown this way.

## 예제

```ahk
WinShow "Untitled"
```
