# `ScriptHwnd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · variable · GUI 視窗與視窗自動化

## 描述

The handle of the hidden window of the script, the name the v2 gives it. The window is the one that owns the messages of the session, and the number changes with the run.

## 語法

```ahk
ScriptHwnd 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the handle of the window, as a number

## 錯誤

the read fails with the reason of the desktop when the desktop has no window of the script; the write is refused.

## 範例

```ahk
FileAppend A_ScriptHwnd, "*"
```
