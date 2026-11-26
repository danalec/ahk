# `RegRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 注册表 API

## 描述

给出注册表中的值；这台机器没有注册表，树把 API 移植到自己的存储上——蜂巢是文件夹，键是文件，放在用户的配置目录下。

## 语法

```ahk
V := RegRead(KeyName, ValueName)
```

## 参数

- **KeyName** — The key.
- **ValueName** — The value; omitted = default.

## 返回值

该值的内容；值不存在时给出默认值

## 错误

"Parameter #1 of RegRead must not be omitted in this case." when the key is left out; "Parameter #1 of RegRead is invalid." when the name does not begin with a hive of the ten; "(2) The system cannot find the file specified." when the key or the value is not there and no default is given; a value with no name is "(161) The specified path is invalid."

## 示例

```ahk
v := RegRead("HKCU\Software\MyApp", "Setting", "default")
```

## 相关

[`RegWrite`](RegWrite.md), [`RegDelete`](RegDelete.md)
