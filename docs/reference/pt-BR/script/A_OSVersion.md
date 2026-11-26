# `A_OSVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · O analisador e a execução do script

## Descrição

A versão do sistema operacional da máquina: a release do kernel, lida de `/proc/sys/kernel/osrelease`.

## Sintaxe

```ahk
A_OSVersion 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

a versão, como um texto

## Erros

the read fails when the release of the kernel cannot be read; the write is refused.

## Exemplo

```ahk
FileAppend A_OSVersion, "*"
```
