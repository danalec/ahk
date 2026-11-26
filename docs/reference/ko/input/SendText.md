# `SendText`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 핫키, 핫스트링, 입력

## 설명

Types the text as it stands: the braces of v2 are not read here, and the text goes to the keyboard one character at a time.

## 구문

```ahk
SendText …
```

## 매개변수

Text

## 반환 값

nothing

## 오류

the keyboard of the backend is a Linux uinput device, so on a host that is not Linux the text is not typed.

## 예제

```ahk
SendText "a literal {braced} text"
```
