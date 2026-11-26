# `FileAppend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Operações de arquivos e diretórios

## Descrição

Acrescenta texto ao fim de um arquivo. O nome do arquivo pode ser `*`, e o texto vai então para a saída do relatório; dentro do corpo de um `Loop Read` o nome pode ser omitido, e o arquivo do laço é o que é escrito.

## Sintaxe

```ahk
FileAppend Text, Filename, Options
```

## Parâmetros

- **Text** — The text or Buffer.
- **Filename** — The path; * is stdout.
- **Options** — Encoding, `n, RAW.

## Valor de retorno

nada

## Erros

ValueError when the name of the file is empty or when an option is not one of v2; OSError when the file cannot be opened for writing. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and a linefeed; a code page the tree does not carry is refused with the reason.

## Observações

Writing to * goes to the report output on this tree. A Buffer writes its bytes.

## Exemplo

```ahk
FileAppend "hello`n", "out.txt"
```

## Relacionados

[`FileRead`](FileRead.md), [`FileOpen`](FileOpen.md)
