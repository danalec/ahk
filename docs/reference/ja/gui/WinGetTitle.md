# `WinGetTitle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

The title of the active window, or of the window that matches the text that was given.

## 構文

```ahk
T := WinGetTitle(WinTitle, WinText)
```

## パラメータ

- **WinTitle** — The criteria; omitted = last found.

## 戻り値

the title of the window

## エラー

a window that was named and is not there answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## 例

```ahk
title := WinGetTitle()
```

## 関連

[`WinGetText`](WinGetText.md), [`WinGetPos`](WinGetPos.md)
