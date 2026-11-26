# `A_Cursor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · Hotkeys, hotstrings e entrada

## Descrição

A forma do cursor do ponteiro. A forma pertence à máquina, e a máquina é consultada.

## Sintaxe

```ahk
A_Cursor 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o nome da forma, como um texto ("Arrow", "IBeam", "Wait", ...)

## Erros

the read fails with the reason of the desktop when the desktop cannot tell the shape of the cursor; the write is refused, because the name is a built-in variable that the dialect does not let a script write.

## Exemplo

```ahk
FileAppend A_Cursor, "*"
```
