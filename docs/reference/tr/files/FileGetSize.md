# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Dosya ve dizin işlemleri

## Açıklama

Answers how big a file is.

## Sözdizimi

```ahk
FileGetSize …
```

## Parametreler

FileName, [Unit]

## Dönüş değeri

the size, rounded down; a folder answers zero.

## Hatalar

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## Örnek

```ahk
n := FileGetSize("out.txt", "K")
```
