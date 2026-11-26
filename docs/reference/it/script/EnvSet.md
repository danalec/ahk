# `EnvSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · L'analizzatore e l'esecuzione dello script

## Descrizione

Sets an environment variable of the runner and answers the value; the script runs on one thread, so nothing else reads the environment while the change happens.

## Sintassi

```ahk
EnvSet Name, Value
```

## Parametri

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## Valore di ritorno

the value that was set, as text

## Errori

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## Esempio

```ahk
EnvSet "MY_VAR", "value"
```

## Correlati

[`EnvGet`](EnvGet.md)
