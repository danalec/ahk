# `A_KeyDelay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · Hotkeys, hotstrings e entrada

## Descrição

O atraso entre as teclas de um `Send`, em milissegundos. É uma configuração da execução, e vale dez por padrão.

## Sintaxe

```ahk
A_KeyDelay 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o atraso, como um número

## Erros

the write takes the number of the value, and a text that holds no number is zero.

## Exemplo

```ahk
FileAppend A_KeyDelay, "*"
```
