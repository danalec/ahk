# `WinHide`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Takes the window of the script off the screen.

## 構文

```ahk
WinHide …
```

## パラメータ

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 戻り値

nothing

## エラー

a window that is not there answers 'Target window not found.'; a window of another program cannot be hidden this way.

## 例

```ahk
WinHide "Untitled"
```
