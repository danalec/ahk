# `StrGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

O texto de um buffer, ou o texto num endereço. A origem é um buffer ou um número; o comprimento e a codificação vêm depois.

## Sintaxe

```ahk
StrGet …
```

## Parâmetros

Source, [Length], [Encoding]

## Valor de retorno

o texto; um endereço que não corresponde a buffer nem a texto devolve o texto vazio

## Erros

a text as the source is refused with "Parameter #1 invalid."; an encoding the dialect does not know is refused with "Invalid Encoding.".

## Exemplo

```ahk
StrGet(Buffer(16), 4, "UTF-8")
```
