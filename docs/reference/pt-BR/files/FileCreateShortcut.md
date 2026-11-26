# `FileCreateShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Cria um atalho para um programa. Neste desktop um atalho é um arquivo `.desktop` da especificação freedesktop, e ele é criado mesmo quando o alvo não existe.

## Sintaxe

```ahk
FileCreateShortcut …
```

## Parâmetros

Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]

## Valor de retorno

nada

## Erros

Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

## Exemplo

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```
