# `HasBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · method · `Object` · 言語の中核: テキスト、数値、正規表現

## 説明

Whether the base of an object is another object.

## 構文

```ahk
HasBase 
```

## パラメータ

the object, and the base

## 戻り値

1 when the base is that one, 0 when it is not

## エラー

a base that is not an object is refused before the question of the object is asked.

## 例

```ahk
FileAppend HasBase({a: 1}, {}), "*"
```
