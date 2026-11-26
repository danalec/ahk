# `FileRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Lê um arquivo inteiro e devolve o seu texto. A marca da codificação decide a codificação do arquivo, e a opção do script decide quando o arquivo não tem marca. O valor do comando se pega com o parênteteses: o jeito de comando dentro de uma expressão falha, e o referência do v2.0.31 falha do mesmo jeito.

## Sintaxe

```ahk
Text := FileRead(Filename, Options)
```

## Parâmetros

- **Filename** — The path.
- **Options** — `n (LF), `r`n, RAW, encoding (UTF-8/UTF-16/CPnnn), m<n> limit.

## Valor de retorno

o texto do arquivo; um Buffer quando a opção `RAW` está presente.

## Erros

OSError when the file cannot be read; ValueError when an option is not one of v2. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and `m<n>`, the number of bytes to read.

## Observações

A missing file throws an OSError. The encoding is detected from a BOM, else the option, else the default.

## Exemplo

```ahk
text := FileRead("out.txt")
```

## Relacionados

[`FileAppend`](FileAppend.md), [`FileOpen`](FileOpen.md), [`LoopRead`](LoopRead.md)
