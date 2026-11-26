# `A_OSVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

The version of the operating system of the machine: the release of the kernel, read from `/proc/sys/kernel/osrelease`.

## Sözdizimi

```ahk
A_OSVersion 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the version, as a text

## Hatalar

the read fails when the release of the kernel cannot be read; the write is refused.

## Örnek

```ahk
FileAppend A_OSVersion, "*"
```
