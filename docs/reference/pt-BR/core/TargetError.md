# `TargetError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · class · O núcleo da linguagem: texto, matemática e padrões

## Descrição

A window/control/pixel that was not found.

## Sintaxe

```ahk
e := TargetError(Message, What, Extra)
```

## Parâmetros

?.

## Valor de retorno

An error object (Message/What/Extra/File/Line/Stack).

## Exemplo

```ahk
throw TargetError("x")
```

## Relacionados

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
