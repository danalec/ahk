# `Trim`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

The text without the characters at its ends.

## Sintaxis

```ahk
Trimmed := Trim(String, OmitChars)
```

## Parámetros

- **String** — The text.
- **OmitChars** — Default space+tab.

## Valor de retorno

the text without the leading and the trailing characters of the set; the set is the space and the tab when the caller does not name one

## Errores

none.

## Observaciones

LTrim/RTrim trim one end.

## Ejemplo

```ahk
Trim("  hello  ")
```

## Relacionados

[`LTrim`](LTrim.md), [`RTrim`](RTrim.md)
