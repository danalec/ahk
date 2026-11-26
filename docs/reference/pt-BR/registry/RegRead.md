# `RegRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · APIs do registro

## Descrição

Devolve o valor do registro; esta máquina não tem registro, e a árvore carrega a API sobre um store próprio: as hives são pastas e as chaves são arquivos, na pasta de configuração do usuário.

## Sintaxe

```ahk
V := RegRead(KeyName, ValueName)
```

## Parâmetros

- **KeyName** — The key.
- **ValueName** — The value; omitted = default.

## Valor de retorno

o valor, ou o padrão quando o valor não está lá

## Erros

"Parameter #1 of RegRead must not be omitted in this case." when the key is left out; "Parameter #1 of RegRead is invalid." when the name does not begin with a hive of the ten; "(2) The system cannot find the file specified." when the key or the value is not there and no default is given; a value with no name is "(161) The specified path is invalid."

## Exemplo

```ahk
v := RegRead("HKCU\Software\MyApp", "Setting", "default")
```

## Relacionados

[`RegWrite`](RegWrite.md), [`RegDelete`](RegDelete.md)
