# `FileGetVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ファイルとディレクトリの操作

## 説明

The version of a file.

## 構文

```ahk
S := FileGetVersion(Filename)
```

## パラメータ

1..255.

## 戻り値

The version.

## 例

```ahk
v := FileGetVersion("a.exe")
```

## 関連

[`FileExist`](FileExist.md)
