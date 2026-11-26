# `FormatTime`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Dates and times

## Description

A moment written the way a format says. The date comes first and the format comes last; a text that carries no date at all is the date of the machine.

## Syntax

```ahk
S := FormatTime(YYYYMMDDHH24MISS, Format)
```

## Parameters

- **Time** — A timestamp; omitted = now.
- **Format** — The picture (yyyy-MM-dd etc.).

## Return value

the date written the way the format says

## Errors

a date the dialect cannot read is an invalid parameter.

## Example

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

## Related

[`DateAdd`](DateAdd.md), [`DateDiff`](DateDiff.md), [`A_Now`](A_Now.md)
