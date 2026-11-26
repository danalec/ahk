# `BlockInput`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Raccourcis, chaînes de déclenchement et saisie

## Description

Turns the input of the whole machine off, and that is all it can do. Turning input off for the whole machine is a privilege this compositor does not give a client, and this tree will not claim it did.

## Syntaxe

```ahk
BlockInput …
```

## Paramètres

Mode

## Valeur de retour

nothing for 'Off'

## Erreurs

any other mode fails with 'blocking the input of the whole machine is a privilege this compositor does not give a client'.

## Exemple

```ahk
BlockInput "Off"
```
