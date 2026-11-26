# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ファイルとディレクトリの操作

## 説明

Answers how big a file is.

## 構文

```ahk
FileGetSize …
```

## パラメータ

FileName, [Unit]

## 戻り値

the size, rounded down; a folder answers zero.

## エラー

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## 例

```ahk
n := FileGetSize("out.txt", "K")
```
