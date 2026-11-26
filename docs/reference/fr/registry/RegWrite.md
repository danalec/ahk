# `RegWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Les API du registre

## Description

Writes a value into the registry of the store of the runtime; the key is made when it is not there, with its ancestors.

## Syntaxe

```ahk
RegWrite Value, KeyName, ValueName
```

## Paramètres

- **Value** — The value.
- **KeyName** — The key.
- **ValueName** — The value name.

## Valeur de retour

the empty text

## Erreurs

"Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

## Exemple

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```

## Voir aussi

[`RegRead`](RegRead.md), [`RegDelete`](RegDelete.md)
