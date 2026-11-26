# `OnMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'analyseur et l'exécution du script

## Description

Registers a function to handle a window message.

## Syntaxe

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## Paramètres

1..3.

## Valeur de retour

Nothing.

## Exemple

```ahk
OnMessage(0x100, Fn)
```

## Voir aussi

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
