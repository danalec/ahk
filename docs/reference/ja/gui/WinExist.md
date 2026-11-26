# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · GUIウィンドウとウィンドウの自動化

## 説明

Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.

## 構文

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## パラメータ

- **WinTitle** — The criteria.

## 戻り値

the identifier of the window that is there, zero when there is none

## エラー

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## 備考

Sets the last-found window for the Win* family.

## 例

```ahk
if WinExist("Untitled")
```

## 関連

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
