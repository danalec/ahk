# `EnvGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O analisador e a execução do script

## Descrição

Devolve o valor de uma variável de ambiente do runner; o ambiente é o do processo em que o script roda.

## Sintaxe

```ahk
V := EnvGet(Name)
```

## Parâmetros

- **Name** — The variable.

## Valor de retorno

o valor da variável, ou o texto vazio quando o ambiente não a tem

## Erros

nothing; a name that is not there is the empty text and not a failure.

## Exemplo

```ahk
home := EnvGet("HOME")
```

## Relacionados

[`EnvSet`](EnvSet.md)
