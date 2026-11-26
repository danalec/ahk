# `HasBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · method · `Object` · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

Whether the base of an object is another object.

## Sözdizimi

```ahk
HasBase 
```

## Parametreler

the object, and the base

## Dönüş değeri

1 when the base is that one, 0 when it is not

## Hatalar

a base that is not an object is refused before the question of the object is asked.

## Örnek

```ahk
FileAppend HasBase({a: 1}, {}), "*"
```
