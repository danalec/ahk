# `DllCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Вызовы нативных функций (DllCall)

## Описание

Calls a function of a library of the machine; the marshalling is pure and in the language crate, the call is the capability of the desktop, and the box answers it with a trampoline.

## Синтаксис

```ahk
R := DllCall("Dll\Func", Args..., "CDecl ReturnType")
```

## Параметры

- **Dll\Func** — The library and symbol.
- **Args** — Type/value pairs.
- **ReturnType** — The return type.

## Возвращаемое значение

the value the function answers

## Ошибки

"Failed to load DLL.", "Call to nonexistent function.", "Invalid arg type." and "Invalid return type." are the reasons of v2; a library of Windows on a Linux box is called through the raven, and a box without one says "this box has no raven: the libraries of Windows cannot be called."

## Замечания

On Linux dlopen/dlsym; on Windows LoadLibraryW/GetProcAddress; refuses on macOS/FreeBSD.

## Пример

```ahk
pid := DllCall("libc.so.6\getpid", "Int")
```

## См. также

[`NumGet`](NumGet.md), [`Buffer`](Buffer.md), [`CallbackCreate`](CallbackCreate.md)
