# `A_ComputerName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · A máquina, os processos e o ambiente

## Descrição

O nome do computador, como a rede o vê: o kernel, e o `HOSTNAME` do ambiente quando o kernel não tem nome.

## Sintaxe

```ahk
A_ComputerName 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o nome, como um texto

## Erros

the read fails when neither the kernel nor the session carries the name; the write is refused.

## Exemplo

```ahk
FileAppend A_ComputerName, "*"
```
