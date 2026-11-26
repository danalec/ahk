# `WinRestore`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Puts the window of the script back from the minimized state, and only that window.

## 構文

```ahk
WinRestore …
```

## パラメータ

WinTitle

## 戻り値

the empty text

## エラー

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## 例

```ahk
WinRestore "MyWindow"
```
