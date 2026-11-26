# `EnvSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Разбор и выполнение скрипта

## Описание

Sets an environment variable of the runner and answers the value; the script runs on one thread, so nothing else reads the environment while the change happens.

## Синтаксис

```ahk
EnvSet Name, Value
```

## Параметры

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## Возвращаемое значение

the value that was set, as text

## Ошибки

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## Пример

```ahk
EnvSet "MY_VAR", "value"
```

## См. также

[`EnvGet`](EnvGet.md)
