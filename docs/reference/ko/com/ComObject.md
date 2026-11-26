# `ComObject`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · class · COM 자동화 (ComObject)

## 설명

Creates or gets a COM object (Windows).

## 구문

```ahk
Obj := ComObject(CLSID, IID)
```

## 매개변수

- **CLSID** — The class id or ProgID.
- **IID** — The interface (default IDispatch).

## 반환 값

A COM object.

## 설명

On the Linux box the object model of the session is D-Bus (DBusObject); the COM dispatch by index and the event sink refuse with the reason.

## 예제

```ahk
shell := ComObject("Shell.Application")
```

## 관련

[`ComValue`](ComValue.md), [`ObjGetBase`](ObjGetBase.md)
