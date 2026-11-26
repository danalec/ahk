# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Where the window is and how big it is, as the compositor reports it.

## 構文

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## パラメータ

- **Output*** — Variables.
- **WinTitle** — The criteria.

## 戻り値

nothing; the place and the size are written into the variables of the caller

## エラー

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## 例

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## 関連

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
