# `A_Is64bitOS`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Whether the machine is a 64-bit one. The tree answers with the width of its own pointer: 1 on a 64-bit build of the runtime, 0 on a 32-bit one.

## Sözdizimi

```ahk
A_Is64bitOS 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

1 or 0

## Hatalar

nothing to read; the write is refused.

## Örnek

```ahk
FileAppend A_Is64bitOS, "*"
```
