# `SendPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 핫키, 핫스트링, 입력

## 설명

Refused. The play method of v2 does not send keystrokes: it posts the messages to the active window, and there is no window here to post them to.

## 구문

```ahk
SendPlay …
```

## 매개변수

Keys

## 반환 값

nothing; the statement fails

## 오류

the play method of v2 posts the messages to the active window, and there is no window here to post them to.

## 예제

```ahk
SendPlay "{Enter}"
```
