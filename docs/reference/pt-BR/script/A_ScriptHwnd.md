# `A_ScriptHwnd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · O analisador e a execução do script

## Descrição

O handle da janela escondida do script: a janela que dona as mensagens da sessão. O número muda com a execução -- o v2.0.31 respondeu 6557770 numa execução e 59116662 em outra.

## Sintaxe

```ahk
A_ScriptHwnd 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o handle da janela, como um número

## Erros

the read fails with the reason of the desktop when the desktop has no window of the script; the write is refused.

## Exemplo

```ahk
FileAppend A_ScriptHwnd, "*"
```
