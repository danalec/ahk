# `A_UserName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · A máquina, os processos e o ambiente

## Descrição

O nome do usuário da sessão: o `USER` do ambiente.

## Sintaxe

```ahk
A_UserName 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o nome, como um texto

## Erros

the read fails when the session carries no such name: "the session does not carry the name of the user: the environment has no USER."; the write is refused.

## Exemplo

```ahk
FileAppend A_UserName, "*"
```
