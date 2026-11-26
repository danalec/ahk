# `RegDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · APIs do registro

## Descrição

Remove um valor do registro do store da runtime; a chave em si fica.

## Sintaxe

```ahk
RegDelete …
```

## Parâmetros

KeyName, ValueName

## Valor de retorno

o texto vazio

## Erros

"Parameter #1 of RegDelete must not be omitted in this case." when the key is left out; a key without a hive of the ten is an invalid parameter; "(2) The system cannot find the file specified." when the key is not there.

## Exemplo

```ahk
RegDelete "HKCU\Software\MyApp", "Setting"
```
