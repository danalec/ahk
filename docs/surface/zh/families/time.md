# 日期与时间

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

语言: 中文（简体）

三个方言中属于 **日期与时间** 类的名称，以及这棵树对每个名称给出的回答。分类是脚本思考的领域，这些名称即是它的全部。

| 名称 | 方言 | 种类 | 类 | 参数 | 状态 | 调用探测 |
|---|---|---|---|---|---|---|
| `A_Hour` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Min` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Mon` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_MSec` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Now` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_NowUTC` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Sec` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_WDay` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_YDay` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Year` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_YYYY` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `DateAdd` | AutoHotkey v2.0.19 | command | -- | 3 | 值 | value |
| `DateDiff` | AutoHotkey v2.0.19 | command | -- | 3 | 值 | value |
| `DD` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `DDD` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `DDDD` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `FormatTime` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | 值 | -- |
| `FormatTime` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 值 | -- |
| `Hour` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `MDay` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Min` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | value |
| `Min` | AutoHotkey v1.1.37.02 | function | -- | up to 1 | 值 | -- |
| `MM` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `MMM` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `MMMM` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Mon` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `MSec` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Now` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `NowUTC` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Sec` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `TickCount` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `WDay` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `YDay` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Year` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `YWeek` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `YYYY` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |

## 这棵树对它做什么

### A_Hour

机器本地时间的小时，两位数字。

- **参数:** nothing; it is a variable, not a call
- **返回:** 小时，两位数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Hour, "*"
```

### A_Min

机器本地时间的分钟，两位数字。

- **参数:** nothing; it is a variable, not a call
- **返回:** 分钟，两位数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Min, "*"
```

### A_Mon

机器本地时间的月份。这个树给出的是与 `A_MM` 相同的两位数字，而不是月份的缩写。

- **参数:** nothing; it is a variable, not a call
- **返回:** 月份，两位数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Mon, "*"
```

### A_MSec

机器本地时间的毫秒，三位数字。v2 的时间数字不带毫秒，而这个名字带。

- **参数:** nothing; it is a variable, not a call
- **返回:** 毫秒，三位数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_MSec, "*"
```

### A_Now

机器的本地时间，写作 YYYYMMDDHH24MISS。

- **参数:** nothing; it is a variable, not a call
- **返回:** 时间，十四位数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused, because the name is a built-in variable that the dialect does not let a script write.

```ahk
FileAppend A_Now, "*"
```

### A_NowUTC

机器的 UTC 时间，写作 YYYYMMDDHH24MISS。

- **参数:** nothing; it is a variable, not a call
- **返回:** 时间，十四位数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_NowUTC, "*"
```

### A_Sec

机器本地时间的秒，两位数字。

- **参数:** nothing; it is a variable, not a call
- **返回:** 秒，两位数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Sec, "*"
```

### A_WDay

本地时间中的星期几，1 是星期日，7 是星期六。

- **参数:** nothing; it is a variable, not a call
- **返回:** 星期几，作为数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_WDay, "*"
```

### A_YDay

本地时间中的年内第几天，1 月 1 日是第一天。

- **参数:** nothing; it is a variable, not a call
- **返回:** 年内第几天，作为数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_YDay, "*"
```

### A_YYYY

机器本地时间的年份，四位数字。

- **参数:** nothing; it is a variable, not a call
- **返回:** 年份，四位数字的文本
- **错误:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_YYYY, "*"
```

### DateAdd

给出一个时刻加上一段时间；日期在前、单位在后，单位只读第一个字母。

- **参数:** DateTime, Number, Unit
- **返回:** 新的日期，文本形式 YYYYMMDDHH24MISS
- **错误:** "Parameter #3 of DateAdd is invalid." when the unit is not one of d, h, m and s; "Parameter #1 of DateAdd is invalid." when the date is not one that v2 can read; "Parameter #2 of DateAdd requires a Number, but received a String."

```ahk
next := DateAdd(20240101, 1, "d")
```

### DateDiff

给出两个时刻之间的差；答案是第一个日期减第二个日期，向零截断。

- **参数:** DateTime1, DateTime2, Unit
- **返回:** 差值，整数，单位是所要求的那个
- **错误:** "Parameter #3 of DateDiff is invalid." when the unit is not one of d, h, m and s; a date that v2 cannot read is an invalid parameter.

```ahk
days := DateDiff(20240110, 20240101, "d")
```

### FormatTime

按格式写出的时刻。日期在前，格式在后；完全不含日期的文本表示机器当前的时刻。

- **参数:** Date, [Format]
- **返回:** 按格式写出的日期
- **错误:** a date the dialect cannot read is an invalid parameter.

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

### Min

这些数字中最小的一个。

- **参数:** Number, [Number2], ...
- **返回:** 其中最小的数字
- **错误:** a value that is not a number is refused.

```ahk
Min(3, 7, 5)
```
