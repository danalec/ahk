# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Operacje na plikach i katalogach

## Opis

Answers how big a file is.

## Składnia

```ahk
FileGetSize …
```

## Parametry

FileName, [Unit]

## Wartość zwracana

the size, rounded down; a folder answers zero.

## Błędy

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## Przykład

```ahk
n := FileGetSize("out.txt", "K")
```
