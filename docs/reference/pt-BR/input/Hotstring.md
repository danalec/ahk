# `Hotstring`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Hotkeys, hotstrings e entrada

## Descrição

Registra um texto que roda uma função quando é digitado. O texto de v2 é escrito entre dois caracteres iguais, e o caractere não faz parte dele.

## Sintaxe

```ahk
Hotstring …
```

## Parâmetros

Text, Action [, Options]

## Valor de retorno

nada

## Erros

a text that is not a string is refused as an invalid parameter; the watcher reads the physical keyboard, and the device of the box is skipped so that a hotstring does not fire on what the script types.

## Exemplo

```ahk
Hotstring "::btw::", ShowTheDefinition
```
