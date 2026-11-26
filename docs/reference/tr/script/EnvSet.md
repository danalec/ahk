# `EnvSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Sets an environment variable of the runner and answers the value; the script runs on one thread, so nothing else reads the environment while the change happens.

## Sözdizimi

```ahk
EnvSet Name, Value
```

## Parametreler

- **Name** — The variable.
- **Value** — The value; omitted deletes.

## Dönüş değeri

the value that was set, as text

## Hatalar

an empty name and a name that carries an equal sign are refused with the reason of the machine.

## Örnek

```ahk
EnvSet "MY_VAR", "value"
```

## İlgili

[`EnvGet`](EnvGet.md)
