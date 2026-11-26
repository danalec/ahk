# `A_SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · Tasti rapidi, stringhe di attivazione e input

## Descrizione

The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.

## Sintassi

```ahk
A_SendMode 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the name of the mode, as a text

## Errori

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## Esempio

```ahk
FileAppend A_SendMode, "*"
```
