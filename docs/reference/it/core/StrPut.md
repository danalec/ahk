# `StrPut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

Writes a text into a buffer. Without a target the call answers the number of bytes the text needs, terminator included.

## Sintassi

```ahk
StrPut …
```

## Parametri

Text, [Target], [Encoding], [Length]

## Valore di ritorno

the number of bytes written, or the number of bytes the text needs when there is no target

## Errori

a target that is not a buffer is refused with "Parameter #2 of StrPut is invalid."; a text that does not fit in the size the caller names is refused with "Invalid Length.".

## Esempio

```ahk
StrPut("abc", Buffer(16), "UTF-8")
```
