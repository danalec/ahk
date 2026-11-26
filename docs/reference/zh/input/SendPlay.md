# `SendPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 热键、热字符串与输入

## 描述

被拒绝。v2 的 play 方式不发送按键，而是向活动窗口投递消息，而这里没有可以投递的窗口。

## 语法

```ahk
SendPlay …
```

## 参数

Keys

## 返回值

无返回值；语句失败

## 错误

the play method of v2 posts the messages to the active window, and there is no window here to post them to.

## 示例

```ahk
SendPlay "{Enter}"
```
