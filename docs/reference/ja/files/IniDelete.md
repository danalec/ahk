# `IniDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ファイルとディレクトリの操作

## 説明

Takes a key of an ini file away. A key that is not given takes the whole section with it.

## 構文

```ahk
IniDelete …
```

## パラメータ

FileName, Section, [Key]

## 戻り値

nothing

## エラー

Error when the file cannot be written.

## 例

```ahk
IniDelete "app.ini", "Main", "Name"
```
