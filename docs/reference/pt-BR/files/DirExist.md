# `DirExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Operações de arquivos e diretórios

## Descrição

Devolve os atributos da primeira pasta que o nome ou o padrão encontra.

## Sintaxe

```ahk
A := DirExist(Path)
```

## Parâmetros

- **Path** — A path or wildcard.

## Valor de retorno

`D` quando a pasta existe, e o texto vazio quando não existe; um arquivo que casa com o padrão não é uma pasta.

## Erros

nothing fails.

## Exemplo

```ahk
x := DirExist("out")
```

## Relacionados

[`FileExist`](FileExist.md)
