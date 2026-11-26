# `DirDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Operações de arquivos e diretórios

## Descrição

Apaga um diretório. Um diretório que não está vazio permanece, a não ser que a recursão seja pedida.

## Sintaxe

```ahk
DirDelete Path, Recurse
```

## Parâmetros

- **Path** — The directory.
- **Recurse** — 1 to delete contents.

## Valor de retorno

nada; o nome de um comando não responde nada.

## Erros

Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.

## Exemplo

```ahk
d := DirDelete("out", 1)
```

## Relacionados

[`DirCreate`](DirCreate.md), [`FileDelete`](FileDelete.md)
