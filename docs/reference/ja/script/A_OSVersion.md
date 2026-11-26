# `A_OSVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · variable · パーサとスクリプトの実行

## 説明

The version of the operating system of the machine: the release of the kernel, read from `/proc/sys/kernel/osrelease`.

## 構文

```ahk
A_OSVersion 
```

## パラメータ

nothing; it is a variable, not a call

## 戻り値

the version, as a text

## エラー

the read fails when the release of the kernel cannot be read; the write is refused.

## 例

```ahk
FileAppend A_OSVersion, "*"
```
