# `HasBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · method · `Object` · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Se a base de um objeto é outro objeto.

## Sintaxe

```ahk
HasBase 
```

## Parâmetros

the object, and the base

## Valor de retorno

1 quando a base é aquela, 0 quando não é

## Erros

a base that is not an object is refused before the question of the object is asked.

## Exemplo

```ahk
FileAppend HasBase({a: 1}, {}), "*"
```
