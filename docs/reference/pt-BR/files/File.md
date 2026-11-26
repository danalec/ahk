# `File`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · method · `Error` · Operações de arquivos e diretórios

## Descrição

A classe de um arquivo aberto da máquina. A classe não é criada: `File(...)` responde "Invalid base."; o arquivo é criado pelo embutido `FileOpen`, e é o único jeito de ter um.

## Sintaxe

```ahk
File 
```

## Parâmetros

nothing that makes it; the arguments of `FileOpen` open the file

## Valor de retorno

o objeto da classe `File`

## Erros

the call of the class is refused with "Invalid base.".

## Exemplo

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```
