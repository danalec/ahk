# `Sort`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*KeySharp* · method · `Array` · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Os itens de um texto em ordem. O segundo parâmetro são as opções, lidas letra a letra, e o terceiro é um callback que compara dois itens.

## Sintaxe

```ahk
S := Sort(String, Options)
```

## Parâmetros

- **String** — The lines.
- **Options** — N (numeric), R (reverse), D delim, C case, etc.

## Valor de retorno

o texto com os itens em ordem

## Erros

the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".

## Exemplo

```ahk
Sort("b`na`n", "N")
```

## Relacionados

[`StrSplit`](StrSplit.md), [`Array`](Array.md)
