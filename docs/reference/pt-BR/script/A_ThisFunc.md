# `A_ThisFunc`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · O analisador e a execução do script

## Descrição

O nome da função que está rodando.

## Sintaxe

```ahk
A_ThisFunc 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o nome, como um texto; o texto vazio quando nenhuma função roda

## Erros

nothing to read; the write is refused.

## Exemplo

```ahk
FileAppend A_ThisFunc, "*"
```
