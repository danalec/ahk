# `StrPut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

Writes a text into a buffer. Without a target the call answers the number of bytes the text needs, terminator included.

## Sözdizimi

```ahk
StrPut …
```

## Parametreler

Text, [Target], [Encoding], [Length]

## Dönüş değeri

the number of bytes written, or the number of bytes the text needs when there is no target

## Hatalar

a target that is not a buffer is refused with "Parameter #2 of StrPut is invalid."; a text that does not fit in the size the caller names is refused with "Invalid Length.".

## Örnek

```ahk
StrPut("abc", Buffer(16), "UTF-8")
```
