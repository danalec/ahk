# `FileOpen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Operações de arquivos e diretórios

## Descrição

Abre um arquivo e devolve o objeto dele, da classe `File`. Os modos são `r`, `w`, `a` e `rw`; o nome `*` é o console da máquina.

## Sintaxe

```ahk
File := FileOpen(Filename, Flags, Encoding)
```

## Parâmetros

- **Filename** — The path; * is the console.
- **Flags** — r/w/a/rw plus share mode.
- **Encoding** — UTF-8/UTF-16/CPnnn/RAW.

## Valor de retorno

um objeto `File`.

## Erros

ValueError when the mode is not one of the four; OSError when the file cannot be opened.

## Observações

The File object keeps the handle; Close releases it.

## Exemplo

```ahk
f := FileOpen("out.txt", "w")
```

## Relacionados

[`FileRead`](FileRead.md), [`FileAppend`](FileAppend.md)
