# `FileCreateShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ファイルとディレクトリの操作

## 説明

Makes a shortcut to a program. On this desktop a shortcut is a `.desktop` file of the freedesktop specification, and it is made even for a target that is not there.

## 構文

```ahk
FileCreateShortcut …
```

## パラメータ

Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]

## 戻り値

nothing

## エラー

Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

## 例

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```
