# `Download`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Operações de arquivos e diretórios

## Descrição

Busca um arquivo da rede e o escreve no disco. A caixa faz isso com `curl`; a flag de cache que o v2 escreve na frente da url é aceita e descartada.

## Sintaxe

```ahk
Download …
```

## Parâmetros

URL, FileName

## Valor de retorno

nada

## Erros

Error when the url cannot be reached at all, or when the machine has no `curl`; a file that is not there on the network is not a failure, and the page of the error of the server is what is written.

## Exemplo

```ahk
Download "https://example.com/x", "x.html"
```
