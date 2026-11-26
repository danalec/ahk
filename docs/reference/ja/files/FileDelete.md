# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ファイルとディレクトリの操作

## 説明

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## 構文

```ahk
FileDelete Pattern
```

## パラメータ

- **Pattern** — A path or wildcard.

## 戻り値

nothing

## エラー

OSError when the file cannot be taken away.

## 例

```ahk
FileDelete "out.txt"
```

## 関連

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
