# `WinGetClass`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

The class of the window that matches the title that was given.

## 構文

```ahk
WinGetClass …
```

## パラメータ

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 戻り値

the class of the window

## エラー

a title that matches no window answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## 例

```ahk
class := WinGetClass("Untitled")
```
