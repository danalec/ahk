# `URLDownloadToFile`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · La machine, les processus et l'environnement

## Description

Downloads a URL to a file.

## Syntaxe

```ahk
URLDownloadToFile URL, Filename
```

## Paramètres

- **URL** — The address.
- **Filename** — The destination.

## Valeur de retour

Nothing; throws on failure.

## Remarques

On Linux curl; on Windows URLMon.

## Exemple

```ahk
URLDownloadToFile "https://x/a", "a"
```

## Voir aussi

[`Download`](Download.md), [`FileRead`](FileRead.md)
