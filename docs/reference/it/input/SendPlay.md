# `SendPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · Tasti rapidi, stringhe di attivazione e input

## Descrizione

Refused. The play method of v2 does not send keystrokes: it posts the messages to the active window, and there is no window here to post them to.

## Sintassi

```ahk
SendPlay …
```

## Parametri

Keys

## Valore di ritorno

nothing; the statement fails

## Errori

the play method of v2 posts the messages to the active window, and there is no window here to post them to.

## Esempio

```ahk
SendPlay "{Enter}"
```
