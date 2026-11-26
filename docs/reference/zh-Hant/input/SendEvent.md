# `SendEvent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 熱鍵、熱字串與輸入

## 描述

Sends the keys the way the box sends them. The event method of v2 is the method of Windows, and the tree has one keyboard, so the keys go through it the way `Send` sends them.

## 語法

```ahk
SendEvent …
```

## 參數

Keys

## 傳回值

nothing

## 錯誤

the keyboard of the backend is a Linux uinput device, and it fails on a host that is not Linux.

## 範例

```ahk
SendEvent "^a"
```
