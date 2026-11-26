# `A_Clipboard`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · Le presse-papiers

## Description

The text of the clipboard of the desktop. A script may write it, and the concatenation adds to what is there.

## Syntaxe

```ahk
A_Clipboard 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the text of the clipboard

## Erreurs

the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.

## Exemple

```ahk
A_Clipboard := "hello"
FileAppend A_Clipboard, "*"
```
