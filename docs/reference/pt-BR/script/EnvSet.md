# `EnvSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O analisador e a execução do script

## Descrição

Define uma variável de ambiente do runner e devolve o valor; o script roda em uma thread só, e nada mais lê o ambiente durante a mudança.

## Sintaxe

```ahk
EnvSet Name, Value
```

## Parâmetros

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## Valor de retorno

o valor que foi definido, como texto

## Erros

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## Exemplo

```ahk
EnvSet "MY_VAR", "value"
```

## Relacionados

[`EnvGet`](EnvGet.md)
