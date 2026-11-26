# `RegWrite`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 注册表 API

## 描述

把一个值写进运行器自己的注册表存储；键不存在时会被连同它的祖先一起建立。

## 语法

```ahk
RegWrite Value, KeyName, ValueName
```

## 参数

- **Value** — The value.
- **KeyName** — The key.
- **ValueName** — The value name.

## 返回值

空文本

## 错误

"Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

## 示例

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```

## 相关

[`RegRead`](RegRead.md), [`RegDelete`](RegDelete.md)
