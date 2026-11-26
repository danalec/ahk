# `InStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Onde um texto está em outro texto. A posição conta a partir de um e é zero quando o trecho procurado não está lá.

## Sintaxe

```ahk
Pos := InStr(Haystack, Needle, CaseSense, Start, Occurrence)
```

## Parâmetros

- **Haystack** — The text to search.
- **Needle** — The text to find.
- **CaseSense** — On/Off/Locale.
- **Start** — 1-based, negative from end.
- **Occurrence** — Which hit.

## Valor de retorno

a posição da ocorrência, ou 0 quando não há nenhuma

## Erros

a needle that is empty is an invalid parameter; the logical case sense is an invalid parameter; a start or an occurrence of zero is an invalid parameter.

## Observações

The sensitive search uses the vectorized substring scan (see docs/simd.md).

## Exemplo

```ahk
InStr("hello", "ll")
```

## Relacionados

[`StrReplace`](StrReplace.md), [`SubStr`](SubStr.md), [`RegExMatch`](RegExMatch.md)
