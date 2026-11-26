# `SendEvent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · Tasti rapidi, stringhe di attivazione e input

## Descrizione

Sends the keys the way the box sends them. The event method of v2 is the method of Windows, and the tree has one keyboard, so the keys go through it the way `Send` sends them.

## Sintassi

```ahk
SendEvent …
```

## Parametri

Keys

## Valore di ritorno

nothing

## Errori

the keyboard of the backend is a Linux uinput device, and it fails on a host that is not Linux.

## Esempio

```ahk
SendEvent "^a"
```
