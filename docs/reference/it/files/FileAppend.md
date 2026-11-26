# `FileAppend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Operazioni su file e directory

## Descrizione

Adds text at the end of a file. The name of the file may be `*`, and the text then goes to the output of the report; inside the body of a `Loop Read` the name may be left out, and the file of the loop is the one that is written.

## Sintassi

```ahk
FileAppend Text, Filename, Options
```

## Parametri

- **Text** — The text or Buffer.
- **Filename** — The path; * is stdout.
- **Options** — Encoding, `n, RAW.

## Valore di ritorno

nothing

## Errori

ValueError when the name of the file is empty or when an option is not one of v2; OSError when the file cannot be opened for writing. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and a linefeed; a code page the tree does not carry is refused with the reason.

## Osservazioni

Writing to * goes to the report output on this tree. A Buffer writes its bytes.

## Esempio

```ahk
FileAppend "hello`n", "out.txt"
```

## Correlati

[`FileRead`](FileRead.md), [`FileOpen`](FileOpen.md)
