# `GetKeyVK`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Горячие клавиши, горячие строки и ввод

## Описание

The virtual key code of Windows of the key. It is the number of the key of the keyboard of Windows, and the tree keeps it for the scripts that ask for it.

## Синтаксис

```ahk
GetKeyVK …
```

## Параметры

KeyName

## Возвращаемое значение

the virtual key code, a number

## Ошибки

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Пример

```ahk
vk := GetKeyVK("Enter")
```
