# `WinGetClientPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · GUI 창과 창 자동화

## 설명

The same place, of the part of the window the program draws in. A Wayland client does not draw its own frame -- the compositor does -- so the geometry the compositor reports is the window the user sees, and it is the answer of both commands.

## 구문

```ahk
WinGetClientPos …
```

## 매개변수

[WindowTitle] [, WinText, &X, &Y, &Width, &Height, ExcludeTitle, ExcludeText]

## 반환 값

nothing; the place and the size are written into the variables of the caller

## 오류

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## 예제

```ahk
WinGetClientPos "Untitled", , &x, &y, &w, &h
```
