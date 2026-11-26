# `DirCopy`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Operações de arquivos e diretórios

## Descrição

Copia um diretório e tudo o que há nele. O que é copiado é o conteúdo do diretório para o destino.

## Sintaxe

```ahk
DirCopy …
```

## Parâmetros

Source, Dest, [Overwrite]

## Valor de retorno

nada

## Erros

Error `Failed` when the source is not a directory, or when the destination is there and the overwrite is not asked for; a name that holds a wildcard is refused, and the reference fails with `Failed` on a wildcard too.

## Exemplo

```ahk
DirCopy "src", "dst"
```
