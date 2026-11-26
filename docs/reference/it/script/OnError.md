# `OnError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · L'analizzatore e l'esecuzione dello script

## Descrizione

Registers the function that handles a failure the script did not catch.

## Sintassi

```ahk
OnError Function, Add
```

## Parametri

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## Valore di ritorno

the integer 1

## Errori

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Esempio

```ahk
OnError((err, mode) => 0)
```

## Correlati

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
