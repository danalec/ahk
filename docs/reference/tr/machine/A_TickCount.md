# `A_TickCount`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Makine, süreçler ve ortam

## Açıklama

The number of milliseconds the runtime has counted since an arbitrary origin. The count never goes backwards.

## Sözdizimi

```ahk
A_TickCount 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the number of milliseconds, as a number

## Hatalar

the write is refused; the name is a built-in variable that the dialect does not let a script write.

## Örnek

```ahk
FileAppend A_TickCount, "*"
```
