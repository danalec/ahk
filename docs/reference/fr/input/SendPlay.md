# `SendPlay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Raccourcis, chaînes de déclenchement et saisie

## Description

Refused. The play method of v2 does not send keystrokes: it posts the messages to the active window, and there is no window here to post them to.

## Syntaxe

```ahk
SendPlay …
```

## Paramètres

Keys

## Valeur de retour

nothing; the statement fails

## Erreurs

the play method of v2 posts the messages to the active window, and there is no window here to post them to.

## Exemple

```ahk
SendPlay "{Enter}"
```
