# `IniRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Lê o valor de uma chave de um arquivo ini. Sem seção, devolve os nomes das seções; sem chave, devolve a seção inteira. Os arquivos ini são os do Windows: `[section]`, `key=value` e `;` para comentário.

## Sintaxe

```ahk
V := IniRead(Filename, Section, Key, Default)
```

## Parâmetros

- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key; omitted reads the whole section.
- **Default** — Returned when absent.

## Valor de retorno

o valor da chave; o padrão do script quando a chave não existe e um padrão foi dado.

## Erros

Error `The requested key, section or file was not found.` when the key is not there and no default is given.

## Exemplo

```ahk
v := IniRead("app.ini", "Main", "Name")
```

## Relacionados

[`IniWrite`](IniWrite.md), [`IniDelete`](IniDelete.md)
