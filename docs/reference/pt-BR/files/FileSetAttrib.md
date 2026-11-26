# `FileSetAttrib`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Altera os atributos de um arquivo. Os atributos vêm primeiro e o nome do arquivo depois, como o binário do v2.0.31 os recebe. O nome do arquivo pode ser um padrão, e os atributos mudam em todo arquivo que o padrão nomeia.

## Sintaxe

```ahk
FileSetAttrib …
```

## Parâmetros

Attributes, FileName, [Recurse]

## Valor de retorno

nada

## Erros

ValueError when the attributes are not one of v2; OSError when the attributes of the file cannot be changed. The third parameter is the recursion: the "R" walks into the subfolders, and any other letter is refused.

## Exemplo

```ahk
FileSetAttrib "+H", "*.txt"
```
