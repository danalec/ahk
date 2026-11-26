# `RegWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 레지스트리 API

## 설명

Writes a value into the registry of the store of the runtime; the key is made when it is not there, with its ancestors.

## 구문

```ahk
RegWrite Value, KeyName, ValueName
```

## 매개변수

- **Value** — The value.
- **KeyName** — The key.
- **ValueName** — The value name.

## 반환 값

the empty text

## 오류

"Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

## 예제

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```

## 관련

[`RegRead`](RegRead.md), [`RegDelete`](RegDelete.md)
