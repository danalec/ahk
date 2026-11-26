# `WinMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Moves and resizes the window the script made; a Wayland compositor places the windows itself and need not honour the place a client asks for.

## 構文

```ahk
WinMove X, Y, W, H, WinTitle
```

## パラメータ

- **X/Y/W/H** — New geometry; omitted keeps.
- **WinTitle** — The criteria.

## 戻り値

the empty text

## エラー

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## 例

```ahk
WinMove 100, 100, 800, 600, "MyWindow"
```

## 関連

[`WinGetPos`](WinGetPos.md), [`WinMinimize`](WinMinimize.md)
