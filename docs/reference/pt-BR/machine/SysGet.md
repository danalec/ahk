# `SysGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · A máquina, os processos e o ambiente

## Descrição

Devolve uma métrica do sistema; os números são os de GetSystemMetrics, e só as da tela são carregadas.

## Sintaxe

```ahk
SysGet …
```

## Parâmetros

Metric

## Valor de retorno

a métrica, como inteiro

## Erros

a metric of the window manager of Windows is refused with "SysGet(n) is a metric of the window manager of Windows, and this desktop does not have one."; a compositor that published no monitor answers "the compositor published no monitor, so the metric n of SysGet is not known."; a number that is not a metric at all is zero.

## Exemplo

```ahk
w := SysGet(0)
```
