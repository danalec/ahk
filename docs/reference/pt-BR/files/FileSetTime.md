# `FileSetTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Altera o horário de um arquivo. O tempo vem primeiro e o nome do arquivo depois, como o binário do v2.0.31 os recebe; um tempo em branco é o momento da máquina. O nome do arquivo pode ser um padrão, e o horário muda em todo arquivo que o padrão nomeia.

## Sintaxe

```ahk
FileSetTime …
```

## Parâmetros

TimeStamp, FileName, [Which], [Recurse]

## Valor de retorno

nada

## Erros

ValueError when the stamp is not one of v2, or when the time that is wanted is not the "M", the "C" or the "A"; OSError when the times of the file cannot be changed. The fourth parameter is the recursion: the "R" walks into the subfolders, and any other letter is refused.

## Exemplo

```ahk
FileSetTime "20000101000000", "out.txt", "M"
```
