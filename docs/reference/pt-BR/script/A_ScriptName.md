# `A_ScriptName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · O analisador e a execução do script

## Descrição

O nome do script. O script do runner vem pela entrada padrão e não tem arquivo, então o nome é um asterisco. Um script pode escrever o nome, e o valor escrito é o que ele guarda.

## Sintaxe

```ahk
A_ScriptName 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

o texto "*", ou o texto que foi escrito

## Erros

nothing to read; the write takes any value.

## Exemplo

```ahk
FileAppend A_ScriptName, "*"
```
