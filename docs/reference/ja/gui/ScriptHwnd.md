# `ScriptHwnd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · variable · GUIウィンドウとウィンドウの自動化

## 説明

The handle of the hidden window of the script, the name the v2 gives it. The window is the one that owns the messages of the session, and the number changes with the run.

## 構文

```ahk
ScriptHwnd 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the handle of the window, as a number

## エラー

the read fails with the reason of the desktop when the desktop has no window of the script; the write is refused.

## 例

```ahk
FileAppend A_ScriptHwnd, "*"
```
