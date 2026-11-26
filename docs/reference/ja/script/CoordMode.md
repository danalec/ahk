# `CoordMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · パーサとスクリプトの実行

## 説明

The way the coordinates are read. The five targets of v2 are the mouse, the pixels, the caret, the menu and the tooltip; the tree has no windows, so the modes of the caret, the menu and the tooltip are kept all the same and nothing uses them.

## 構文

```ahk
CoordMode …
```

## パラメータ

Target [, Relative]

## 戻り値

the mode that was there before

## エラー

a target or a relative that v2 does not know is refused as an invalid parameter.

## 例

```ahk
CoordMode "Mouse", "Screen"
```
