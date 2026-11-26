# `EnvGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · L'analizzatore e l'esecuzione dello script

## Descrizione

Answers the value of an environment variable of the runner; the environment is the one of the process the script runs in.

## Sintassi

```ahk
V := EnvGet(Name)
```

## Parametri

- **Name** — The variable.

## Valore di ritorno

the value of the variable, or the empty text when the environment does not have it

## Errori

nothing; a name that is not there is the empty text and not a failure.

## Esempio

```ahk
home := EnvGet("HOME")
```

## Correlati

[`EnvSet`](EnvSet.md)
