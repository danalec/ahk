# `StrPut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

Writes a text into a buffer. Without a target the call answers the number of bytes the text needs, terminator included.

## Syntaxe

```ahk
StrPut …
```

## Paramètres

Text, [Target], [Encoding], [Length]

## Valeur de retour

the number of bytes written, or the number of bytes the text needs when there is no target

## Erreurs

a target that is not a buffer is refused with "Parameter #2 of StrPut is invalid."; a text that does not fit in the size the caller names is refused with "Invalid Length.".

## Exemple

```ahk
StrPut("abc", Buffer(16), "UTF-8")
```
