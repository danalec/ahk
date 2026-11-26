# `DirExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · Operazioni su file e directory

## Descrizione

Answers the attributes of the first folder that the name or the pattern matches.

## Sintassi

```ahk
A := DirExist(Path)
```

## Parametri

- **Path** — A path or wildcard.

## Valore di ritorno

`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.

## Errori

nothing fails.

## Esempio

```ahk
x := DirExist("out")
```

## Correlati

[`FileExist`](FileExist.md)
