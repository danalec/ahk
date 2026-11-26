# `BlockInput`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ホットキー、ホット文字列、入力

## 説明

Turns the input of the whole machine off, and that is all it can do. Turning input off for the whole machine is a privilege this compositor does not give a client, and this tree will not claim it did.

## 構文

```ahk
BlockInput …
```

## パラメータ

Mode

## 戻り値

nothing for 'Off'

## エラー

any other mode fails with 'blocking the input of the whole machine is a privilege this compositor does not give a client'.

## 例

```ahk
BlockInput "Off"
```
