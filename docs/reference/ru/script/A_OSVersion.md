# `A_OSVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · variable · Разбор и выполнение скрипта

## Описание

The version of the operating system of the machine: the release of the kernel, read from `/proc/sys/kernel/osrelease`.

## Синтаксис

```ahk
A_OSVersion 
```

## Параметры

nothing; it is a variable, not a call

## Возвращаемое значение

the version, as a text

## Ошибки

the read fails when the release of the kernel cannot be read; the write is refused.

## Пример

```ahk
FileAppend A_OSVersion, "*"
```
