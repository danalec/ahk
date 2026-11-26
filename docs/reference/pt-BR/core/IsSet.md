# `IsSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*KeySharp* · function · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Se a variável de quem chama tem valor. A árvore carrega este além de `IsSetRef`, e ele pede a variável e não o valor dela.

## Sintaxe

```ahk
IsSet 
```

## Parâmetros

the variable, not its value

## Valor de retorno

1 quando a variável tem valor, 0 quando não tem

## Erros

an argument that is not a variable is refused with "IsSet requires a variable.".

## Exemplo

```ahk
x := 1
FileAppend IsSet(x), "*"
```
