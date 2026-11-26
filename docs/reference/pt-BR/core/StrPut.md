# `StrPut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Escreve um texto num buffer. Sem alvo, a chamada devolve quantos bytes o texto precisa, com o terminador incluído.

## Sintaxe

```ahk
StrPut …
```

## Parâmetros

Text, [Target], [Encoding], [Length]

## Valor de retorno

os bytes escritos, ou os bytes que o texto precisa quando não há alvo

## Erros

a target that is not a buffer is refused with "Parameter #2 of StrPut is invalid."; a text that does not fit in the size the caller names is refused with "Invalid Length.".

## Exemplo

```ahk
StrPut("abc", Buffer(16), "UTF-8")
```
