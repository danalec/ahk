# `ControlSetStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · GUIウィンドウとウィンドウの自動化

## 説明

Writes the style of a control of a window. The style is absolute, the way the style of a window is: the text may carry a `+`, a `-` or a `^` before the number, and the runtime computes the new style out of the one that is there.

## 構文

```ahk
ControlSetStyle …
```

## パラメータ

Style, Control, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## 戻り値

the empty text

## エラー

an empty text refuses with 'Parameter #1 of ControlSetStyle is invalid.'; a window that is not there refuses with 'Target window not found.', and a control that is not there with 'Target control not found.'.

## 例

```ahk
ControlSetStyle("+0x10", "Edit1", "ZZZ_NO_WINDOW_ZZZ")
```
