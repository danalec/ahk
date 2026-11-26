# `A_SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · Hotkeys, hotstrings e entrada

## Descrição

O método que um `Send` usa: `Input`, `Event`, `Play` ou `InputThenPlay`.

## Sintaxe

```ahk
A_SendMode 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o nome do modo, como um texto

## Erros

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## Exemplo

```ahk
FileAppend A_SendMode, "*"
```
