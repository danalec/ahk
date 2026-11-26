# `HasMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · method · `Object` · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Se um objeto tem um método.

## Sintaxe

```ahk
HasMethod 
```

## Parâmetros

the object, the name of the method, and the number of parameters it would take

## Valor de retorno

1 quando o objeto tem esse método, 0 quando não tem

## Erros

nothing; a value that is not an object answers zero, and the question with no name asks whether the value is a class.

## Exemplo

```ahk
o := {a: 1}
FileAppend HasMethod(o, "a"), "*"
```
