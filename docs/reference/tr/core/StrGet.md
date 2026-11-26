# `StrGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

The text of a buffer, or the text at an address. The source is a buffer or a number; the length and the encoding come after it.

## Sözdizimi

```ahk
StrGet …
```

## Parametreler

Source, [Length], [Encoding]

## Dönüş değeri

the text; an address that names no buffer and no text answers the empty text

## Hatalar

a text as the source is refused with "Parameter #1 invalid."; an encoding the dialect does not know is refused with "Invalid Encoding.".

## Örnek

```ahk
StrGet(Buffer(16), 4, "UTF-8")
```
