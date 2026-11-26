# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*KeySharp* · function · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Starts a script in a new process and answers a ScriptProcess (ks).

## Sözdizimi

```ahk
P := RunScript(Path, Args)
```

## Parametreler

- **Path** — The script.
- **Args** — The arguments.

## Dönüş değeri

A ScriptProcess.

## Örnek

```ahk
p := RunScript("child.ahk")
```

## İlgili

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
