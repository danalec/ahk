# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*KeySharp* · function · Parser i wykonanie skryptu

## Opis

Starts a script in a new process and answers a ScriptProcess (ks).

## Składnia

```ahk
P := RunScript(Path, Args)
```

## Parametry

- **Path** — The script.
- **Args** — The arguments.

## Wartość zwracana

A ScriptProcess.

## Przykład

```ahk
p := RunScript("child.ahk")
```

## Powiązane

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
