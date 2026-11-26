# `SendPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Skróty, ciągi wyzwalające i wprowadzanie

## Opis

Refused. The play method of v2 does not send keystrokes: it posts the messages to the active window, and there is no window here to post them to.

## Składnia

```ahk
SendPlay …
```

## Parametry

Keys

## Wartość zwracana

nothing; the statement fails

## Błędy

the play method of v2 posts the messages to the active window, and there is no window here to post them to.

## Przykład

```ahk
SendPlay "{Enter}"
```
