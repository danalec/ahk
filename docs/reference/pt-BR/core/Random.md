# `Random`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Um número entre os dois que quem chama nomeia. Sem argumentos é um número entre zero e um; com um é um inteiro abaixo dele; com dois é um inteiro entre eles, as duas pontas contadas.

## Sintaxe

```ahk
N := Random(Min, Max)
```

## Parâmetros

- **Min/Max** — The range; omitted = float 0..1.
- **Seed** — With a single arg, reseeds.

## Valor de retorno

o número

## Erros

none.

## Exemplo

```ahk
Random(1, 6)
```

## Relacionados

[`Sort`](Sort.md)
