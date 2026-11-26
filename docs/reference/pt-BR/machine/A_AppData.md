# `A_AppData`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · A máquina, os processos e o ambiente

## Descrição

A pasta de dados do usuário. Em uma máquina Linux é `$XDG_DATA_HOME`, e `$HOME/.local/share` quando a variável não existe.

## Sintaxe

```ahk
A_AppData 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o caminho da pasta, como um texto

## Erros

the read fails with "the machine has no such folder." when the machine has no such folder; the write is refused.

## Exemplo

```ahk
FileAppend A_AppData, "*"
```
