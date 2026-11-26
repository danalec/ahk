# `WinClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Closes a window. It closes the window of the script itself: the listing of the compositor publishes no close request for a window of another program.

## 構文

```ahk
WinClose WinTitle, WinText, SecondsToWait
```

## パラメータ

- **WinTitle** — The criteria.
- **SecondsToWait** — Then force.

## 戻り値

nothing

## エラー

a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.

## 例

```ahk
WinClose "Untitled"
```

## 関連

[`WinKill`](WinKill.md), [`WinActivate`](WinActivate.md)
