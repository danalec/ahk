# `A_ScreenHeight`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · Captura de tela e funções de pixel

## Descrição

A altura da tela da área de trabalho, em pixels.

## Sintaxe

```ahk
A_ScreenHeight 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

a altura, como um número

## Erros

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## Exemplo

```ahk
FileAppend A_ScreenHeight, "*"
```
