# `RegDeleteKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · 注册表 API

## 描述

Deletes a registry key (Windows).

## 语法

```ahk
RegDeleteKey KeyName
```

## 参数

- **KeyName** — The key.

## 返回值

Nothing.

## 示例

```ahk
RegDeleteKey "HKCU\Software\X"
```

## 相关

[`RegCreateKey`](RegCreateKey.md), [`RegDelete`](RegDelete.md)
