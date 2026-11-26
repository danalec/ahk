# `FileExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Operações de arquivos e diretórios

## Descrição

Devolve os atributos da primeira entrada que o nome ou o padrão encontra.

## Sintaxe

```ahk
Attrib := FileExist(Path)
```

## Parâmetros

- **Path** — A path or a wildcard pattern.

## Valor de retorno

os atributos em letras: `A` para um arquivo, `D` para uma pasta, `RA` para um arquivo que não pode ser escrito, e o texto vazio quando não há esse arquivo.

## Erros

nothing fails: a name that matches nothing answers the empty text.

## Observações

A wildcard answers the first match in the folder's enumeration order.

## Exemplo

```ahk
x := FileExist("out.txt")
```

## Relacionados

[`DirExist`](DirExist.md), [`FileGetAttrib`](FileGetAttrib.md)
