# `A_TickCount`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · A máquina, os processos e o ambiente

## Descrição

O número de milissegundos que o runtime conta desde uma origem arbitrária. A contagem nunca volta atrás.

## Sintaxe

```ahk
A_TickCount 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o número de milissegundos, como um número

## Erros

the write is refused; the name is a built-in variable that the dialect does not let a script write.

## Exemplo

```ahk
FileAppend A_TickCount, "*"
```
