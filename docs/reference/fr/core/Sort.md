# `Sort`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*KeySharp* · method · `Array` · Le cœur du langage : texte, mathématiques et motifs

## Description

The items of a text in order. The second parameter is the options, read letter by letter, and the third is a callback that compares two items.

## Syntaxe

```ahk
S := Sort(String, Options)
```

## Paramètres

- **String** — The lines.
- **Options** — N (numeric), R (reverse), D delim, C case, etc.

## Valeur de retour

the text with the items in order

## Erreurs

the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".

## Exemple

```ahk
Sort("b`na`n", "N")
```

## Voir aussi

[`StrSplit`](StrSplit.md), [`Array`](Array.md)
