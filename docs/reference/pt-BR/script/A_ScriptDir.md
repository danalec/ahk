# `A_ScriptDir`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · O analisador e a execução do script

## Descrição

O diretório onde a execução começou. O script do runner chega pela entrada padrão e não tem arquivo próprio, então o diretório do script é o diretório da execução.

## Sintaxe

```ahk
A_ScriptDir 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o caminho do diretório, como um texto

## Erros

the read fails when the runtime does not keep the directory where the run started; the write is refused.

## Exemplo

```ahk
FileAppend A_ScriptDir, "*"
```
