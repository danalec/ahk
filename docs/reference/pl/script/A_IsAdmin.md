# `A_IsAdmin`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · variable · Parser i wykonanie skryptu

## Opis

Whether the script runs with the rights of an administrator: the effective user id, and the root is the one that is zero.

## Składnia

```ahk
A_IsAdmin 
```

## Parametry

nothing; it is a variable, not a call

## Wartość zwracana

1 when the script is elevated, 0 when it is not

## Błędy

the read fails when the machine cannot say; the write is refused.

## Przykład

```ahk
FileAppend A_IsAdmin, "*"
```
