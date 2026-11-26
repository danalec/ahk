# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*KeySharp* · function · Разбор и выполнение скрипта

## Описание

Starts a script in a new process and answers a ScriptProcess (ks).

## Синтаксис

```ahk
P := RunScript(Path, Args)
```

## Параметры

- **Path** — The script.
- **Args** — The arguments.

## Возвращаемое значение

A ScriptProcess.

## Пример

```ahk
p := RunScript("child.ahk")
```

## См. также

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
