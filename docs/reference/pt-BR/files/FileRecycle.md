# `FileRecycle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Manda um arquivo, uma pasta, ou os arquivos que um padrão casa, para a lixeira da máquina. A caixa usa a lixeira da especificação XDG.

## Sintaxe

```ahk
FileRecycle …
```

## Parâmetros

FileName

## Valor de retorno

nada

## Erros

Error `Failed` when the path is not there; a pattern that matches nothing is not a failure; ValueError when the name is empty; Error when the machine does not say where the home directory of the user is.

## Exemplo

```ahk
FileRecycle "out.txt"
```
