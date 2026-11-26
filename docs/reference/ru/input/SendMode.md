# `SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Горячие клавиши, горячие строки и ввод

## Описание

Says how the keys are sent. The keys of this runtime go through the keyboard of the box, one at a time, and that is the mode of the box; the other modes of v2 are the ones of Windows.

## Синтаксис

```ahk
SendMode …
```

## Параметры

Method

## Возвращаемое значение

the number 1 for the mode of the box

## Ошибки

a mode that is not 'Input' fails with 'the keys of this runtime go through the keyboard of the box, and that is the only mode there is.'

## Пример

```ahk
SendMode "Input"
```
