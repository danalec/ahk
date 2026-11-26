# `HasProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · method · `Object` · 言語の中核: テキスト、数値、正規表現

## 説明

Whether an object has a property.

## 構文

```ahk
B := HasProp(Obj, Name)
```

## パラメータ

- **Obj** — The object.
- **Name** — The property.

## 戻り値

1 when the object has such a property, 0 when it has not

## エラー

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## 例

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## 関連

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
