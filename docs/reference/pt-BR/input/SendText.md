# `SendText`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings e entrada

## Descrição

Digita o texto como ele está: as chaves de v2 não são lidas aqui, e o texto vai ao teclado um caractere por vez.

## Sintaxe

```ahk
SendText …
```

## Parâmetros

Text

## Valor de retorno

nada

## Erros

the keyboard of the backend is a Linux uinput device, so on a host that is not Linux the text is not typed.

## Exemplo

```ahk
SendText "a literal {braced} text"
```
