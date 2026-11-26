# `RunScript`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*KeySharp* · function · O analisador e a execução do script

## Descrição

Starts a script in a new process and answers a ScriptProcess (ks).

## Sintaxe

```ahk
P := RunScript(Path, Args)
```

## Parâmetros

- **Path** — The script.
- **Args** — The arguments.

## Valor de retorno

A ScriptProcess.

## Exemplo

```ahk
p := RunScript("child.ahk")
```

## Relacionados

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
