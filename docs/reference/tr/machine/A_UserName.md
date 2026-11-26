# `A_UserName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Makine, süreçler ve ortam

## Açıklama

The name of the user of the session: the `USER` of the environment.

## Sözdizimi

```ahk
A_UserName 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the name, as a text

## Hatalar

the read fails when the session carries no such name: "the session does not carry the name of the user: the environment has no USER."; the write is refused.

## Örnek

```ahk
FileAppend A_UserName, "*"
```
