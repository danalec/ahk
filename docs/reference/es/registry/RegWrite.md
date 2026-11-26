# `RegWrite`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · API del registro

## Descripción

Writes a value into the registry of the store of the runtime; the key is made when it is not there, with its ancestors.

## Sintaxis

```ahk
RegWrite Value, KeyName, ValueName
```

## Parámetros

- **Value** — The value.
- **KeyName** — The key.
- **ValueName** — The value name.

## Valor de retorno

the empty text

## Errores

"Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

## Ejemplo

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```

## Relacionados

[`RegRead`](RegRead.md), [`RegDelete`](RegDelete.md)
