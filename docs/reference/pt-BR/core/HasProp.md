# `HasProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · method · `Object` · O núcleo da linguagem: texto, matemática e padrões

## Descrição

Se um objeto tem uma propriedade.

## Sintaxe

```ahk
B := HasProp(Obj, Name)
```

## Parâmetros

- **Obj** — The object.
- **Name** — The property.

## Valor de retorno

1 quando o objeto tem essa propriedade, 0 quando não tem

## Erros

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## Exemplo

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## Relacionados

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
