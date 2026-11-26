# `A_Index`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · O analisador e a execução do script

## Descrição

O número da volta do laço que está rodando. É zero antes do primeiro laço, e cada laço guarda o valor que tinha e o devolve quando termina.

## Sintaxe

```ahk
A_Index 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o número da volta, como um número

## Erros

nothing to read; a script may write the name, and the loop goes on from the value that was written.

## Exemplo

```ahk
for item in [1, 2, 3]
    FileAppend A_Index, "*"
```
