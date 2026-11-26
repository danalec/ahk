# `Send`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

通过后端自建的 uinput 键盘在机器上输入文本并按下按键。

## 语法

```ahk
Send Keys
```

## 参数

- **Keys** — The keystrokes, with {Enter}, ^c, !tab, {Blind}, etc.

## 返回值

无返回值

## 错误

the keyboard is a Linux uinput device, so on a host that is not Linux every key action fails; a character the US layout has no key for fails with 'the keyboard of this box has no key for'.

## 备注

SendMode chooses Input/Play/Event. On Linux it is the uinput/XTEST keyboard.

## 示例

```ahk
Send "hello{Enter}"
```

## 相关

[`SendText`](SendText.md), [`ControlSend`](ControlSend.md), [`Click`](Click.md), [`SendMode`](SendMode.md)
