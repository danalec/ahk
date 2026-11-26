# `RegWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · APIs do registro

## Descrição

Escreve um valor no registro do store da runtime; a chave é criada quando não está lá, com os seus ancestrais.

## Sintaxe

```ahk
RegWrite Value, KeyName, ValueName
```

## Parâmetros

- **Value** — The value.
- **KeyName** — The key.
- **ValueName** — The value name.

## Valor de retorno

o texto vazio

## Erros

"Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

## Exemplo

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```

## Relacionados

[`RegRead`](RegRead.md), [`RegDelete`](RegDelete.md)
