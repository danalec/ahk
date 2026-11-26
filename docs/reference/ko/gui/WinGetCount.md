# `WinGetCount`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · GUI 창과 창 자동화

## 설명

How many windows match the title that was given.

## 구문

```ahk
WinGetCount …
```

## 매개변수

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 반환 값

the number of the windows, a number

## 오류

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## 예제

```ahk
count := WinGetCount("Untitled")
```
