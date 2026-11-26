# `A_UserName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · variable · Maszyna, procesy i środowisko

## Opis

The name of the user of the session: the `USER` of the environment.

## Składnia

```ahk
A_UserName 
```

## Parametry

nothing; it is a variable, not a call

## Wartość zwracana

the name, as a text

## Błędy

the read fails when the session carries no such name: "the session does not carry the name of the user: the environment has no USER."; the write is refused.

## Przykład

```ahk
FileAppend A_UserName, "*"
```
