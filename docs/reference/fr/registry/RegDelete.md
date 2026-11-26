# `RegDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Les API du registre

## Description

Removes a value of the registry of the store of the runtime; the key itself stays.

## Syntaxe

```ahk
RegDelete …
```

## Paramètres

KeyName, ValueName

## Valeur de retour

the empty text

## Erreurs

"Parameter #1 of RegDelete must not be omitted in this case." when the key is left out; a key without a hive of the ten is an invalid parameter; "(2) The system cannot find the file specified." when the key is not there.

## Exemple

```ahk
RegDelete "HKCU\Software\MyApp", "Setting"
```
