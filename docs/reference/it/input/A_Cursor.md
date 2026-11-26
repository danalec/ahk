# `A_Cursor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · Tasti rapidi, stringhe di attivazione e input

## Descrizione

The shape of the cursor of the pointer. The shape belongs to the machine, and the machine is asked for it.

## Sintassi

```ahk
A_Cursor 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the name of the shape, as a text ("Arrow", "IBeam", "Wait", ...)

## Errori

the read fails with the reason of the desktop when the desktop cannot tell the shape of the cursor; the write is refused, because the name is a built-in variable that the dialect does not let a script write.

## Esempio

```ahk
FileAppend A_Cursor, "*"
```
