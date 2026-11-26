# `FileGetShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Operazioni su file e directory

## Descrizione

Reads a shortcut's fields.

## Sintassi

```ahk
FileGetShortcut Path, OutTarget, OutDir, OutArgs, OutDesc, OutIcon, OutIconNum, OutRunState
```

## Parametri

1..255.

## Valore di ritorno

Nothing; the vars are filled.

## Esempio

```ahk
FileGetShortcut "a.lnk", &t
```

## Correlati

[`FileCreateShortcut`](FileCreateShortcut.md)
