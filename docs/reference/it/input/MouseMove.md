# `MouseMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Tasti rapidi, stringhe di attivazione e input

## Descrizione

Moves the pointer of the backend to a place on the screen. The box moves the pointer to the place, and not from where it is.

## Sintassi

```ahk
MouseMove X, Y, Speed
```

## Parametri

- **X/Y** — Screen coords (or relative with R).
- **Speed** — 0..100.

## Valore di ritorno

nothing

## Errori

the relative place of the move and the speed of the move are refused as not carried yet.

## Esempio

```ahk
MouseMove 400, 300
```

## Correlati

[`Click`](Click.md), [`MouseGetPos`](MouseGetPos.md)
