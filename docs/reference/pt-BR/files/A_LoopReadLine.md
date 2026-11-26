# `A_LoopReadLine`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · Operações de arquivos e diretórios

## Descrição

A linha do arquivo em que o laço de leitura está.

## Sintaxe

```ahk
A_LoopReadLine 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

a linha, como um texto; o texto vazio antes do primeiro laço de leitura

## Erros

nothing to read; the write is refused.

## Exemplo

```ahk
Loop Read "notes.txt"
    FileAppend A_LoopReadLine, "*"
```
