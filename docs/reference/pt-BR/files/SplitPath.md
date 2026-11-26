# `SplitPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Corta um caminho em suas partes e as escreve nas variáveis de quem chama. Um caminho de arquivo é cortado na barra invertida e uma url na barra, e a unidade de uma url é o seu esquema e o seu host.

## Sintaxe

```ahk
SplitPath …
```

## Parâmetros

Path, &OutFileName, &OutDir, &OutExt, &OutNameNoExt, &OutDrive

## Valor de retorno

nada

## Erros

TypeError when a piece is not a variable to write into.

## Exemplo

```ahk
SplitPath "C:\dir\file.txt", &name, &dir, &ext
```
