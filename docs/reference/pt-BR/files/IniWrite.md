# `IniWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Escreve uma chave de um arquivo ini. O valor vem primeiro e o nome do arquivo depois, como o binário do v2.0.31 os recebe; um arquivo que não existe é criado.

## Sintaxe

```ahk
IniWrite Value, Filename, Section, Key
```

## Parâmetros

- **Value** — The text.
- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key.

## Valor de retorno

nada

## Erros

Error when the file cannot be written.

## Exemplo

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

## Relacionados

[`IniRead`](IniRead.md), [`IniDelete`](IniDelete.md)
