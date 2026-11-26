# `WinGetStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · GUIウィンドウとウィンドウの自動化

## 説明

The style of a window: the number of the bits of Windows, the `WS_*` ones that say how the window is decorated and what it may do. The style of the window of the script is 80674816.

## 構文

```ahk
WinGetStyle …
```

## パラメータ

WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## 戻り値

the style, as a number

## エラー

a window that is not there refuses with 'Target window not found.', and the title that was asked about is in the `Extra`; the text of the window and the two exclusions are not carried yet.

## 例

```ahk
FileAppend WinGetStyle("ZZZ_NO_WINDOW_ZZZ"), "*"
```
