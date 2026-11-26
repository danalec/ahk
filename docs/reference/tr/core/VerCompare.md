# `VerCompare`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

Two version strings, compared. The numbers of the two versions are compared one by one, and the component that is missing is a zero. A version with a pre-release suffix is older than the same version without it.

## Sözdizimi

```ahk
VerCompare …
```

## Parametreler

Version1, Version2

## Dönüş değeri

the order of the two versions as -1, 0 or 1; when the second version carries an operator -- <, <=, >, >= or = -- the answer is whether the relation holds

## Hatalar

none.

## Örnek

```ahk
VerCompare("1.2.0", "1.10.0")
```
