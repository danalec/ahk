# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*KeySharp* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

A statement that raises a value. The value is usually an error object, and it may be any value.

## Sözdizimi

```ahk
Throw 
```

## Parametreler

Value

## Dönüş değeri

nothing; the statement leaves the flow of the script

## Hatalar

none; the value is caught by the catch that names its class.

## Örnek

```ahk
throw ValueError("boom")
```
