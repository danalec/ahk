# `Trim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

O texto sem os caracteres das pontas.

## Sintaxe

```ahk
Trimmed := Trim(String, OmitChars)
```

## Parâmetros

- **String** — The text.
- **OmitChars** — Default space+tab.

## Valor de retorno

o texto sem os caracteres iniciais e finais do conjunto; o conjunto é o espaço e o tab quando quem chama não nomeia um

## Erros

none.

## Observações

LTrim/RTrim trim one end.

## Exemplo

```ahk
Trim("  hello  ")
```

## Relacionados

[`LTrim`](LTrim.md), [`RTrim`](RTrim.md)
