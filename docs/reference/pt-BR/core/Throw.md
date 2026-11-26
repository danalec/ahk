# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*KeySharp* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Uma instrução que levanta um valor. O valor costuma ser um objeto de erro, e pode ser qualquer valor.

## Sintaxe

```ahk
Throw 
```

## Parâmetros

Value

## Valor de retorno

nada; a instrução muda o fluxo do script

## Erros

none; the value is caught by the catch that names its class.

## Exemplo

```ahk
throw ValueError("boom")
```
