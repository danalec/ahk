# `Send`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 熱鍵、熱字串與輸入

## 描述

Types text and presses keys on the machine, through the uinput keyboard of the backend.

## 語法

```ahk
Send Keys
```

## 參數

- **Keys** — The keystrokes, with {Enter}, ^c, !tab, {Blind}, etc.

## 傳回值

nothing

## 錯誤

the keyboard is a Linux uinput device, so on a host that is not Linux every key action fails; a character the US layout has no key for fails with 'the keyboard of this box has no key for'.

## 備註

SendMode chooses Input/Play/Event. On Linux it is the uinput/XTEST keyboard.

## 範例

```ahk
Send "hello{Enter}"
```

## 相關

[`SendText`](SendText.md), [`ControlSend`](ControlSend.md), [`Click`](Click.md), [`SendMode`](SendMode.md)
