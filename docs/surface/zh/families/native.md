# 原生函数调用 (DllCall)

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

语言: 中文（简体）

三个方言中属于 **原生函数调用 (DllCall)** 类的名称，以及这棵树对每个名称给出的回答。分类是脚本思考的领域，这些名称即是它的全部。

| 名称 | 方言 | 种类 | 类 | 参数 | 状态 | 调用探测 |
|---|---|---|---|---|---|---|
| `Buffer.__New` | AutoHotkey v2.0.19 | method | Buffer | -- | 值 | -- |
| `Buffer` | AutoHotkey v2.0.19 | class | -- | -- | 值 | -- |
| `DllCall` | AutoHotkey v2.0.19 | function | -- | up to 1 | 值 | -- |
| `DllCall` | AutoHotkey v1.1.37.02 | function | -- | up to 1 | 值 | -- |
| `LastError` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `NumGet` | AutoHotkey v2.0.19 | function | -- | 2 to 3 | 值 | -- |
| `NumGet` | AutoHotkey v1.1.37.02 | function | -- | 1 to 3 | 值 | -- |
| `NumPut` | AutoHotkey v2.0.19 | function | -- | up to 3 | 值 | -- |
| `NumPut` | AutoHotkey v1.1.37.02 | function | -- | 2 to 4 | 值 | -- |
| `ObjCreateWithDataPtr` | KeySharp | function | -- | -- | 值 | -- |
| `ObjGetDataPtr` | KeySharp | function | -- | -- | 值 | -- |
| `ObjGetDataSize` | KeySharp | function | -- | -- | 值 | -- |
| `ObjSetDataPtr` | KeySharp | function | -- | -- | 值 | -- |
| `Buffer.Ptr` | AutoHotkey v2.0.19 | method | Buffer | -- | 值 | -- |
| `PtrSize` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Buffer.Size` | AutoHotkey v2.0.19 | method | Buffer | -- | 值 | -- |
| `VarSetStrCapacity` | AutoHotkey v2.0.19 | function | -- | 1 to 2 | 值 | -- |

## 这棵树对它做什么

### DllCall

调用机器上一个库里的函数；参数编组是纯语言的部分，调用属于桌面能力，盒子用跳板来回答。

- **参数:** Target, Type1, Param1, [Type2, Param2, ...], [ReturnType]
- **返回:** 函数返回的值
- **错误:** "Failed to load DLL.", "Call to nonexistent function.", "Invalid arg type." and "Invalid return type." are the reasons of v2; a library of Windows on a Linux box is called through the raven, and a box without one says "this box has no raven: the libraries of Windows cannot be called."

```ahk
pid := DllCall("libc.so.6\getpid", "Int")
```

### DllCall

调用机器上一个库里的函数；参数编组是纯语言的部分，调用属于桌面能力，盒子用跳板来回答。

- **参数:** Target, Type1, Param1, [Type2, Param2, ...], [ReturnType]
- **返回:** 函数返回的值
- **错误:** "Failed to load DLL.", "Call to nonexistent function.", "Invalid arg type." and "Invalid return type." are the reasons of v2; a library of Windows on a Linux box is called through the raven, and a box without one says "this box has no raven: the libraries of Windows cannot be called."

```ahk
pid := DllCall("libc.so.6\getpid", "Int")
```

### NumGet

从缓冲区的字节里读出一个数；数的来源是缓冲区，这里不读取某个地址的内存。

- **参数:** Buffer, [Offset], Type
- **返回:** 这些字节所持有的数
- **错误:** "this runtime does not read the memory of an address: the source of a number is a buffer." when the source is a number; a type that is not one of the eleven is refused with "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr."; bytes that do not fit are "the bytes do not fit in the buffer."

```ahk
value := NumGet(buf, 0, "Int")
```

### NumGet

从缓冲区的字节里读出一个数；数的来源是缓冲区，这里不读取某个地址的内存。

- **参数:** Buffer, [Offset], Type
- **返回:** 这些字节所持有的数
- **错误:** "this runtime does not read the memory of an address: the source of a number is a buffer." when the source is a number; a type that is not one of the eleven is refused with "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr."; bytes that do not fit are "the bytes do not fit in the buffer."

```ahk
value := NumGet(buf, 0, "Int")
```

### NumPut

把若干数写进缓冲区的字节；类型和数成对出现，目标在它们之后，和 v2 的写法一样。

- **参数:** Type1, Number1, [Type2, Number2, ...], Target, [Offset]
- **返回:** 写下去的最后一个数右边的地址
- **错误:** "this runtime does not write to the memory of an address: the target of a number is a buffer." when the target is a number; a number of types that is not even is "the types and the numbers of a write come in pairs."; anything after the target and the offset is "a write takes the target and the offset after the pairs."

```ahk
NumPut("Int", 42, buf, 0)
```

### NumPut

把若干数写进缓冲区的字节；类型和数成对出现，目标在它们之后，和 v2 的写法一样。

- **参数:** Type1, Number1, [Type2, Number2, ...], Target, [Offset]
- **返回:** 写下去的最后一个数右边的地址
- **错误:** "this runtime does not write to the memory of an address: the target of a number is a buffer." when the target is a number; a number of types that is not even is "the types and the numbers of a write come in pairs."; anything after the target and the offset is "a write takes the target and the offset after the pairs."

```ahk
NumPut("Int", 42, buf, 0)
```
