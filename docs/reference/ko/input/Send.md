# `Send`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Types text and presses keys on the machine, through the uinput keyboard of the backend.

## 구문

```ahk
Send Keys
```

## 매개변수

- **Keys** — The keystrokes, with {Enter}, ^c, !tab, {Blind}, etc.

## 반환 값

nothing

## 오류

the keyboard is a Linux uinput device, so on a host that is not Linux every key action fails; a character the US layout has no key for fails with 'the keyboard of this box has no key for'.

## 설명

SendMode chooses Input/Play/Event. On Linux it is the uinput/XTEST keyboard.

## 예제

```ahk
Send "hello{Enter}"
```

## 관련

[`SendText`](SendText.md), [`ControlSend`](ControlSend.md), [`Click`](Click.md), [`SendMode`](SendMode.md)
