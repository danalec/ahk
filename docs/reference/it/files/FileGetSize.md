# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Operazioni su file e directory

## Descrizione

Answers how big a file is.

## Sintassi

```ahk
FileGetSize …
```

## Parametri

FileName, [Unit]

## Valore di ritorno

the size, rounded down; a folder answers zero.

## Errori

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## Esempio

```ahk
n := FileGetSize("out.txt", "K")
```
