# `URLDownloadToFile`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · A máquina, os processos e o ambiente

## Descrição

Downloads a URL to a file.

## Sintaxe

```ahk
URLDownloadToFile URL, Filename
```

## Parâmetros

- **URL** — The address.
- **Filename** — The destination.

## Valor de retorno

Nothing; throws on failure.

## Observações

On Linux curl; on Windows URLMon.

## Exemplo

```ahk
URLDownloadToFile "https://x/a", "a"
```

## Relacionados

[`Download`](Download.md), [`FileRead`](FileRead.md)
