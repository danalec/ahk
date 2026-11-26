# `Critical`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*KeySharp* · method · `Thread` · O analisador e a execução do script

## Descrição

A árvore recusa este nome: este runtime executa um script numa thread só, e não pode torná-lo crítico.

## Sintaxe

```ahk
Critical 
```

## Parâmetros

[OnOff]

## Valor de retorno

nada; a chamada falha

## Erros

the refusal is "this runtime runs one script in one thread, and it cannot make it critical."

## Exemplo

```ahk
Critical
```
