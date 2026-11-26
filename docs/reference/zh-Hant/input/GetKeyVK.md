# `GetKeyVK`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 熱鍵、熱字串與輸入

## 描述

The virtual key code of Windows of the key. It is the number of the key of the keyboard of Windows, and the tree keeps it for the scripts that ask for it.

## 語法

```ahk
GetKeyVK …
```

## 參數

KeyName

## 傳回值

the virtual key code, a number

## 錯誤

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## 範例

```ahk
vk := GetKeyVK("Enter")
```
