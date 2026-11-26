# `Enumerator`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · class · 语言的对象：Array、Map 与 Func

## 描述

循环的行走者的类：枚举器一次交出一个序列的一段。这个类造不出来；值的 `__Enum` 造出它，对它的调用就是枚举的一轮。

## 语法

```ahk
Enumerator 
```

## 参数

nothing that makes it

## 返回值

枚举器的类，作为文本

## 错误

the call of the class is refused with "Invalid base.".

## 示例

```ahk
for item in [1, 2]
    FileAppend Type(item), "*"
```
