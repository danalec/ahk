# `WinWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Waits for a window whose title matches to exist, and answers the identifier of it; the identifier of a Wayland window is a token, so it is text.

## 構文

```ahk
WinWait WinTitle, WinText, Timeout
```

## パラメータ

- **WinTitle** — The criteria.
- **Timeout** — Seconds; omitted waits forever.

## 戻り値

the identifier of the window as text, or 0 when the timeout ran out

## エラー

a session that is not Wayland answers "this desktop waits for the windows of a Wayland session, and this session is not Wayland"; the two exclusions are refused with "the two exclusions are not carried yet."; the wait is capped at sixty seconds when the timeout is left out.

## 例

```ahk
WinWait "Calculator", , 5
```

## 関連

[`WinExist`](WinExist.md), [`WinActivate`](WinActivate.md)
