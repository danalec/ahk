# `Download`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · Operazioni su file e directory

## Descrizione

Fetches a file of the network and writes it on the disk. The box does it with `curl`; the flag of the cache that v2 writes in front of the url is accepted and dropped.

## Sintassi

```ahk
Download …
```

## Parametri

URL, FileName

## Valore di ritorno

nothing

## Errori

Error when the url cannot be reached at all, or when the machine has no `curl`; a file that is not there on the network is not a failure, and the page of the error of the server is what is written.

## Esempio

```ahk
Download "https://example.com/x", "x.html"
```
