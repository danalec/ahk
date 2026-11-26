# `OnError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O analisador e a execução do script

## Descrição

Registra a função que trata uma falha que o script não capturou.

## Sintaxe

```ahk
OnError Function, Add
```

## Parâmetros

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## Valor de retorno

o inteiro 1

## Erros

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Exemplo

```ahk
OnError((err, mode) => 0)
```

## Relacionados

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
