# `A_Clipboard`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · A área de transferência

## Descrição

O texto da área de transferência da área de trabalho. Um script pode escrevê-la, e a concatenação soma ao que já está lá.

## Sintaxe

```ahk
A_Clipboard 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o texto da área de transferência

## Erros

the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.

## Exemplo

```ahk
A_Clipboard := "hello"
FileAppend A_Clipboard, "*"
```
