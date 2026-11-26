# `DllCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 네이티브 함수 호출 (DllCall)

## 설명

Calls a function of a library of the machine; the marshalling is pure and in the language crate, the call is the capability of the desktop, and the box answers it with a trampoline.

## 구문

```ahk
R := DllCall("Dll\Func", Args..., "CDecl ReturnType")
```

## 매개변수

- **Dll\Func** — The library and symbol.
- **Args** — Type/value pairs.
- **ReturnType** — The return type.

## 반환 값

the value the function answers

## 오류

"Failed to load DLL.", "Call to nonexistent function.", "Invalid arg type." and "Invalid return type." are the reasons of v2; a library of Windows on a Linux box is called through the raven, and a box without one says "this box has no raven: the libraries of Windows cannot be called."

## 설명

On Linux dlopen/dlsym; on Windows LoadLibraryW/GetProcAddress; refuses on macOS/FreeBSD.

## 예제

```ahk
pid := DllCall("libc.so.6\getpid", "Int")
```

## 관련

[`NumGet`](NumGet.md), [`Buffer`](Buffer.md), [`CallbackCreate`](CallbackCreate.md)
