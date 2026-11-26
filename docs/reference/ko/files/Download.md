# `Download`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 파일과 디렉터리 작업

## 설명

Fetches a file of the network and writes it on the disk. The box does it with `curl`; the flag of the cache that v2 writes in front of the url is accepted and dropped.

## 구문

```ahk
Download …
```

## 매개변수

URL, FileName

## 반환 값

nothing

## 오류

Error when the url cannot be reached at all, or when the machine has no `curl`; a file that is not there on the network is not a failure, and the page of the error of the server is what is written.

## 예제

```ahk
Download "https://example.com/x", "x.html"
```
