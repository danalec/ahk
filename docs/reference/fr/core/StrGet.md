# `StrGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

The text of a buffer, or the text at an address. The source is a buffer or a number; the length and the encoding come after it.

## Syntaxe

```ahk
StrGet …
```

## Paramètres

Source, [Length], [Encoding]

## Valeur de retour

the text; an address that names no buffer and no text answers the empty text

## Erreurs

a text as the source is refused with "Parameter #1 invalid."; an encoding the dialect does not know is refused with "Invalid Encoding.".

## Exemple

```ahk
StrGet(Buffer(16), 4, "UTF-8")
```
