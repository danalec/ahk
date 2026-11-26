# `Struct`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*KeySharp* · class · Les objets du langage : Array, Map et Func

## Description

A view of memory at a place, of a type asked for (ks).

## Syntaxe

```ahk
s := Struct(Ptr, Type)
```

## Paramètres

?.

## Valeur de retour

A Struct with At/Size.

## Exemple

```ahk
s := Struct(p, "Int")
```

## Voir aussi

[`Buffer`](Buffer.md), [`NumGet`](NumGet.md)
