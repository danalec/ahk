# `WinShow`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Puts the window of the script back on the screen, after a `WinHide`.

## 構文

```ahk
WinShow …
```

## パラメータ

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 戻り値

nothing

## エラー

a window that is not there answers 'Target window not found.'; a window of another program cannot be shown this way.

## 例

```ahk
WinShow "Untitled"
```
