# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*KeySharp* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

A statement that raises a value. The value is usually an error object, and it may be any value.

## Sintassi

```ahk
Throw 
```

## Parametri

Value

## Valore di ritorno

nothing; the statement leaves the flow of the script

## Errori

none; the value is caught by the catch that names its class.

## Esempio

```ahk
throw ValueError("boom")
```
