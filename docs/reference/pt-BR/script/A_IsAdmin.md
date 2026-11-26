# `A_IsAdmin`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · variable · O analisador e a execução do script

## Descrição

Se o script roda com direitos de administrador: o id de usuário efetivo, e o root é o que é zero.

## Sintaxe

```ahk
A_IsAdmin 
```

## Parâmetros

nothing; it is a variable, not a call

## Valor de retorno

1 quando o script está elevado, 0 quando não está

## Erros

the read fails when the machine cannot say; the write is refused.

## Exemplo

```ahk
FileAppend A_IsAdmin, "*"
```
