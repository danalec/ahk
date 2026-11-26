# `MouseClick`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Tasti rapidi, stringhe di attivazione e input

## Descrizione

Clicks with the pointer of the backend. The button is the first parameter, the way the function of v2 takes it, and a button that is not there is the left one.

## Sintassi

```ahk
MouseClick …
```

## Parametri

Button, X, Y [Count] [Options]

## Valore di ritorno

nothing

## Errori

the wheel, the fourth and the fifth buttons are refused as not carried yet.

## Esempio

```ahk
MouseClick "Right", 100, 200
```
