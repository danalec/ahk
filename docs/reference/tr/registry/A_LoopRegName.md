# `A_LoopRegName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Kayıt defteri API'leri

## Açıklama

The name of the registry key the registry loop is on. The registry is a thing of Windows and this runtime has none: the registry loop fails with "the registry is a Windows thing and this box has none.", and the name keeps the empty text.

## Sözdizimi

```ahk
A_LoopRegName 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the empty text; the name is never written by the tree

## Hatalar

nothing to read; the write is refused.

## Örnek

```ahk
FileAppend A_LoopRegName, "*"
```
