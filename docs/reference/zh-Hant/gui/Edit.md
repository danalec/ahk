# `Edit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Opens the file of the script in the program that the machine opens a file of that kind with -- the editor of a script.

## 語法

```ahk
Edit …
```

## 參數

none

## 傳回值

the empty text

## 錯誤

the call fails when the script has no file: a script that comes from the standard input has none, and the reason is "the script of this runtime is read from the standard input, so it has no file for an editor to open".

## 範例

```ahk
Edit()
```
