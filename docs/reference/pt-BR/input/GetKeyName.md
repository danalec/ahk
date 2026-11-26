# `GetKeyName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings e entrada

## Descrição

O nome da tecla, como v2 o dá. Os nomes são os da lista de teclas de v2: `Esc`, `Enter`, `CtrlRight`, `Numpad0`, e as grafias `vkXX` e `scXXX`.

## Sintaxe

```ahk
GetKeyName …
```

## Parâmetros

KeyName

## Valor de retorno

o nome da tecla

## Erros

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Exemplo

```ahk
name := GetKeyName("Enter")
```
