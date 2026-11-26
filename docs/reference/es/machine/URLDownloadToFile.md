# `URLDownloadToFile`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · La máquina, los procesos y el entorno

## Descripción

Downloads a URL to a file.

## Sintaxis

```ahk
URLDownloadToFile URL, Filename
```

## Parámetros

- **URL** — The address.
- **Filename** — The destination.

## Valor de retorno

Nothing; throws on failure.

## Observaciones

On Linux curl; on Windows URLMon.

## Ejemplo

```ahk
URLDownloadToFile "https://x/a", "a"
```

## Relacionados

[`Download`](Download.md), [`FileRead`](FileRead.md)
