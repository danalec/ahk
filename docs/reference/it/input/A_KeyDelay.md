# `A_KeyDelay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · Tasti rapidi, stringhe di attivazione e input

## Descrizione

The delay between the keystrokes of a `Send`, in milliseconds. It is a setting of the run, and it is ten by default.

## Sintassi

```ahk
A_KeyDelay 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the delay, as a number

## Errori

the write takes the number of the value, and a text that holds no number is zero.

## Esempio

```ahk
FileAppend A_KeyDelay, "*"
```
