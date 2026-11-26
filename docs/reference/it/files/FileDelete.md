# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Operazioni su file e directory

## Descrizione

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## Sintassi

```ahk
FileDelete Pattern
```

## Parametri

- **Pattern** — A path or wildcard.

## Valore di ritorno

nothing

## Errori

OSError when the file cannot be taken away.

## Esempio

```ahk
FileDelete "out.txt"
```

## Correlati

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
