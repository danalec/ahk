# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Devolve o tamanho de um arquivo.

## Sintaxe

```ahk
FileGetSize …
```

## Parâmetros

FileName, [Unit]

## Valor de retorno

o tamanho, arredondado para baixo; uma pasta devolve zero.

## Erros

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## Exemplo

```ahk
n := FileGetSize("out.txt", "K")
```
