# `SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Tasti rapidi, stringhe di attivazione e input

## Descrizione

Says how the keys are sent. The keys of this runtime go through the keyboard of the box, one at a time, and that is the mode of the box; the other modes of v2 are the ones of Windows.

## Sintassi

```ahk
SendMode …
```

## Parametri

Method

## Valore di ritorno

the number 1 for the mode of the box

## Errori

a mode that is not 'Input' fails with 'the keys of this runtime go through the keyboard of the box, and that is the only mode there is.'

## Esempio

```ahk
SendMode "Input"
```
