# `A_Is64bitOS`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · O analisador e a execução do script

## Descrição

Se a máquina é de 64 bits. A árvore responde com a largura do próprio ponteiro: 1 numa compilação de 64 bits do runtime, 0 numa de 32 bits.

## Sintaxe

```ahk
A_Is64bitOS 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

1 ou 0

## Erros

nothing to read; the write is refused.

## Exemplo

```ahk
FileAppend A_Is64bitOS, "*"
```
