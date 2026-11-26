# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Apaga um arquivo, ou todos os arquivos que um padrão nomeia. Um padrão que não encontra nenhum arquivo não é uma falha.

## Sintaxe

```ahk
FileDelete Pattern
```

## Parâmetros

- **Pattern** — A path or wildcard.

## Valor de retorno

nada

## Erros

OSError when the file cannot be taken away.

## Exemplo

```ahk
FileDelete "out.txt"
```

## Relacionados

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
