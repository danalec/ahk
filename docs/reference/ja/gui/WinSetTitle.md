# `WinSetTitle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Renames the window the script made; renaming the window of another program is not something a Wayland client can do.

## 構文

```ahk
WinSetTitle …
```

## パラメータ

NewTitle, WinTitle

## 戻り値

the empty text

## エラー

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## 例

```ahk
WinSetTitle "New Title", "MyWindow"
```
