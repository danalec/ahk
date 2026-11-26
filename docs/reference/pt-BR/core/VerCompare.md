# `VerCompare`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Duas strings de versão, comparadas. Os números das duas versões são comparados um a um, e o componente que falta vale zero. Uma versão com sufixo de pré-lançamento é mais antiga que a mesma versão sem ele.

## Sintaxe

```ahk
VerCompare …
```

## Parâmetros

Version1, Version2

## Valor de retorno

a ordem das duas versões como -1, 0 ou 1; quando a segunda versão traz um operador -- <, <=, >, >= ou = -- a resposta é se a relação vale

## Erros

none.

## Exemplo

```ahk
VerCompare("1.2.0", "1.10.0")
```
