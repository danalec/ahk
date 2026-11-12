# 注册表 API

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

语言: 中文（简体）

三个方言中属于 **注册表 API** 类的名称，以及这棵树对每个名称给出的回答。分类是脚本思考的领域，这些名称即是它的全部。

| 名称 | 方言 | 种类 | 类 | 参数 | 状态 | 调用探测 |
|---|---|---|---|---|---|---|
| `A_LoopRegName` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_RegView` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `LoopRegKey` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `LoopRegName` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `LoopRegTimeModified` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `LoopRegType` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `RegCreateKey` | AutoHotkey v2.0.19 | function | -- | 0 to 1 | 值 | -- |
| `RegDelete` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | 值 | -- |
| `RegDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `RegDeleteKey` | AutoHotkey v2.0.19 | function | -- | 0 to 1 | 值 | -- |
| `RegRead` | AutoHotkey v2.0.19 | function | -- | 0 to 3 | 值 | -- |
| `RegRead` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `RegView` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `RegWrite` | AutoHotkey v2.0.19 | function | -- | 0 to 4 | 值 | -- |
| `RegWrite` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SetRegView` | AutoHotkey v2.0.19 | command | -- | 1 | 值 | -- |
| `SetRegView` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 值 | -- |

## 这棵树对它做什么

### A_LoopRegName

注册表循环所在的注册表项名称。注册表是 Windows 的东西，而这个运行时没有：注册表循环会以“the registry is a Windows thing and this box has none.”失败，这个名字始终是空文本。

- **参数:** nothing; it is a variable, not a call
- **返回:** 空文本；这个树从不写入它
- **错误:** nothing to read; the write is refused.

```ahk
FileAppend A_LoopRegName, "*"
```

### RegDelete

从运行器自己的注册表存储里删掉一个值；键本身保留。

- **参数:** KeyName, ValueName
- **返回:** 空文本
- **错误:** "Parameter #1 of RegDelete must not be omitted in this case." when the key is left out; a key without a hive of the ten is an invalid parameter; "(2) The system cannot find the file specified." when the key is not there.

```ahk
RegDelete "HKCU\Software\MyApp", "Setting"
```

### RegDelete

从运行器自己的注册表存储里删掉一个值；键本身保留。

- **参数:** KeyName, ValueName
- **返回:** 空文本
- **错误:** "Parameter #1 of RegDelete must not be omitted in this case." when the key is left out; a key without a hive of the ten is an invalid parameter; "(2) The system cannot find the file specified." when the key is not there.

```ahk
RegDelete "HKCU\Software\MyApp", "Setting"
```

### RegRead

给出注册表中的值；这台机器没有注册表，树把 API 移植到自己的存储上——蜂巢是文件夹，键是文件，放在用户的配置目录下。

- **参数:** KeyName, ValueName, [Default]
- **返回:** 该值的内容；值不存在时给出默认值
- **错误:** "Parameter #1 of RegRead must not be omitted in this case." when the key is left out; "Parameter #1 of RegRead is invalid." when the name does not begin with a hive of the ten; "(2) The system cannot find the file specified." when the key or the value is not there and no default is given; a value with no name is "(161) The specified path is invalid."

```ahk
v := RegRead("HKCU\Software\MyApp", "Setting", "default")
```

### RegRead

给出注册表中的值；这台机器没有注册表，树把 API 移植到自己的存储上——蜂巢是文件夹，键是文件，放在用户的配置目录下。

- **参数:** KeyName, ValueName, [Default]
- **返回:** 该值的内容；值不存在时给出默认值
- **错误:** "Parameter #1 of RegRead must not be omitted in this case." when the key is left out; "Parameter #1 of RegRead is invalid." when the name does not begin with a hive of the ten; "(2) The system cannot find the file specified." when the key or the value is not there and no default is given; a value with no name is "(161) The specified path is invalid."

```ahk
v := RegRead("HKCU\Software\MyApp", "Setting", "default")
```

### RegWrite

把一个值写进运行器自己的注册表存储；键不存在时会被连同它的祖先一起建立。

- **参数:** Value, ValueType, KeyName, ValueName
- **返回:** 空文本
- **错误:** "Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```

### RegWrite

把一个值写进运行器自己的注册表存储；键不存在时会被连同它的祖先一起建立。

- **参数:** Value, ValueType, KeyName, ValueName
- **返回:** 空文本
- **错误:** "Parameter #3 of RegWrite must not be omitted in this case." when the key is left out; "Parameter #2 of RegWrite is invalid." when the kind is not one of the five of the dialect; "(161) The specified path is invalid." when the name of the value is left out; a key without a hive is an invalid parameter.

```ahk
RegWrite "value", "REG_SZ", "HKCU\Software\MyApp", "Setting"
```
