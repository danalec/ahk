# `IsSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · L'analizzatore e l'esecuzione dello script

## Descrizione

Whether the variable of the caller has a value. The tree carries it besides `IsSetRef`, and it asks for the variable and not for its value.

## Sintassi

```ahk
IsSet …
```

## Parametri

the variable, not its value

## Valore di ritorno

1 when the variable has a value, 0 when it has none

## Errori

an argument that is not a variable is refused with "IsSet requires a variable.".

## Esempio

```ahk
x := 1
FileAppend IsSet(x), "*"
```
