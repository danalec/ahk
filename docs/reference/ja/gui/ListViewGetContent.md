# `ListViewGetContent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · GUIウィンドウとウィンドウの自動化

## 説明

Reads the content of a ListView.

## 構文

```ahk
S := ListViewGetContent(Options, Control, WinTitle)
```

## パラメータ

0..6.

## 戻り値

The text.

## 例

```ahk
ListViewGetContent("", "SysListView321")
```

## 関連

[`Gui`](Gui.md), [`ControlGetText`](ControlGetText.md)
