# `ControlGetExStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · GUIウィンドウとウィンドウの自動化

## 説明

The extended style of a control of a window: the `WS_EX_*` bits of Windows, the way the extended style of a window is.

## 構文

```ahk
ControlGetExStyle …
```

## パラメータ

Control, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## 戻り値

the extended style, as a number

## エラー

a window that is not there refuses with 'Target window not found.', and a control that is not there with 'Target control not found.'; the text of the window and the two exclusions are not carried yet.

## 例

```ahk
FileAppend ControlGetExStyle("Edit1", "ZZZ_NO_WINDOW_ZZZ"), "*"
```
