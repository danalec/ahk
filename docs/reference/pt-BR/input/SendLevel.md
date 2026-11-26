# `SendLevel`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Guarda o nível dos eventos que o script envia e devolve o nível anterior. O nível diz aos hotkeys quais eventos artificiais ignorar, e nada o lê ainda.

## Sintaxe

```ahk
SendLevel …
```

## Parâmetros

Level

## Valor de retorno

o nível anterior, um número

## Erros

a level outside 0 to 100 is refused as an invalid parameter.

## Exemplo

```ahk
SendLevel 1
```
