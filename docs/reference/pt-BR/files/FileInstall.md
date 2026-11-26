# `FileInstall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Copia um arquivo como faz o instalador de um script compilado. Nada é compilado aqui: o arquivo é copiado.

## Sintaxe

```ahk
FileInstall …
```

## Parâmetros

Source, Dest, [Overwrite]

## Valor de retorno

nada

## Erros

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## Exemplo

```ahk
FileInstall "data.txt", "data.out.txt"
```
