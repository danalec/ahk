# `A_ThisHotkey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · Hotkeys, hotstrings e entrada

## Descrição

O nome da hotkey que está rodando. As hotkeys são as do sistema de hotkeys do runtime, e o nome é o texto vazio quando nenhuma hotkey roda.

## Sintaxe

```ahk
A_ThisHotkey 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o nome da hotkey, como um texto; o texto vazio quando nenhuma hotkey roda

## Erros

nothing to read; the write is refused.

## Exemplo

```ahk
FileAppend A_ThisHotkey, "*"
```
