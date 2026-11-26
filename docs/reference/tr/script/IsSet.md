# `IsSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Whether the variable of the caller has a value. The tree carries it besides `IsSetRef`, and it asks for the variable and not for its value.

## Sözdizimi

```ahk
IsSet …
```

## Parametreler

the variable, not its value

## Dönüş değeri

1 when the variable has a value, 0 when it has none

## Hatalar

an argument that is not a variable is refused with "IsSet requires a variable.".

## Örnek

```ahk
x := 1
FileAppend IsSet(x), "*"
```
