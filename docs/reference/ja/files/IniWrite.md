# `IniWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ファイルとディレクトリの操作

## 説明

Writes a key of an ini file. The value comes first and the name of the file second, the way the binary of v2.0.31 takes them; a file that is not there is made.

## 構文

```ahk
IniWrite Value, Filename, Section, Key
```

## パラメータ

- **Value** — The text.
- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key.

## 戻り値

nothing

## エラー

Error when the file cannot be written.

## 例

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

## 関連

[`IniRead`](IniRead.md), [`IniDelete`](IniDelete.md)
