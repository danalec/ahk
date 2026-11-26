# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Dosya ve dizin işlemleri

## Açıklama

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## Sözdizimi

```ahk
FileDelete Pattern
```

## Parametreler

- **Pattern** — A path or wildcard.

## Dönüş değeri

nothing

## Hatalar

OSError when the file cannot be taken away.

## Örnek

```ahk
FileDelete "out.txt"
```

## İlgili

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
