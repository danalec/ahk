# `RegRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · API реестра

## Описание

Answers the value of the registry; this machine has no registry, and the tree carries the API over a store of its own -- the hives are folders and the keys are files, under the config folder of the user.

## Синтаксис

```ahk
V := RegRead(KeyName, ValueName)
```

## Параметры

- **KeyName** — The key.
- **ValueName** — The value; omitted = default.

## Возвращаемое значение

the value of the value, or the default when the value is not there

## Ошибки

"Parameter #1 of RegRead must not be omitted in this case." when the key is left out; "Parameter #1 of RegRead is invalid." when the name does not begin with a hive of the ten; "(2) The system cannot find the file specified." when the key or the value is not there and no default is given; a value with no name is "(161) The specified path is invalid."

## Пример

```ahk
v := RegRead("HKCU\Software\MyApp", "Setting", "default")
```

## См. также

[`RegWrite`](RegWrite.md), [`RegDelete`](RegDelete.md)
