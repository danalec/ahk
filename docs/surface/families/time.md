# Dates and times

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [Français](fr/index.md) | [Italiano](it/index.md) | [Polski](pl/index.md) | [Türkçe](tr/index.md) | [Deutsch](de/index.md) | [Русский](ru/index.md) | [日本語](ja/index.md) | [中文（简体）](zh/index.md) | [繁體中文](zh-Hant/index.md) | [한국어](ko/index.md) |

Language: English

The names of the family **Dates and times** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State | Probe |
|---|---|---|---|---|---|---|
| `A_Hour` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_Min` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_Mon` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_MSec` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_Now` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_NowUTC` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_Sec` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_WDay` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_YDay` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_Year` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_YYYY` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `DateAdd` | AutoHotkey v2.0.19 | command | -- | 3 | value | value |
| `DateDiff` | AutoHotkey v2.0.19 | command | -- | 3 | value | value |
| `DD` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `DDD` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `DDDD` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `FormatTime` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | value | -- |
| `FormatTime` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `Hour` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `MDay` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `Min` | AutoHotkey v2.0.19 | variable | -- | -- | value | value |
| `Min` | AutoHotkey v1.1.37.02 | function | -- | up to 1 | value | -- |
| `MM` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `MMM` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `MMMM` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `Mon` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `MSec` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `Now` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `NowUTC` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `Sec` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `TickCount` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `WDay` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `YDay` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `Year` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `YWeek` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `YYYY` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |

## What the tree does with it

### A_Hour

The hour of the local time of the machine, in two digits.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the hour, as a text of two digits
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Hour, "*"
```

### A_Min

The minute of the local time of the machine, in two digits.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the minute, as a text of two digits
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Min, "*"
```

### A_Mon

The month of the local time of the machine. This tree answers the same two digits as `A_MM`, and not the short name of the month.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the month, as a text of two digits
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Mon, "*"
```

### A_MSec

The milliseconds of the local time of the machine, in three digits. The number of v2 carries no millisecond, and this name does.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the milliseconds, as a text of three digits
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_MSec, "*"
```

### A_Now

The local time of the machine, written as YYYYMMDDHH24MISS.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the time, as a text of fourteen digits
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused, because the name is a built-in variable that the dialect does not let a script write.

```ahk
FileAppend A_Now, "*"
```

### A_NowUTC

The time of the machine in UTC, written as YYYYMMDDHH24MISS.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the time, as a text of fourteen digits
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_NowUTC, "*"
```

### A_Sec

The second of the local time of the machine, in two digits.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the second, as a text of two digits
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Sec, "*"
```

### A_WDay

The day of the week of the local time, 1 being the Sunday and 7 the Saturday.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the day of the week, as a text of a number
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_WDay, "*"
```

### A_YDay

The day of the year of the local time, 1 being the first of January.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the day of the year, as a text of a number
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_YDay, "*"
```

### A_YYYY

The year of the local time of the machine, in four digits.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the year, as a text of four digits
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_YYYY, "*"
```

### DateAdd

Answers a moment plus an amount of time; the date comes first and the unit comes last, and the unit is read by its first letter.

- **Parameters:** DateTime, Number, Unit
- **Answers:** the new date, as the text YYYYMMDDHH24MISS
- **Errors:** "Parameter #3 of DateAdd is invalid." when the unit is not one of d, h, m and s; "Parameter #1 of DateAdd is invalid." when the date is not one that v2 can read; "Parameter #2 of DateAdd requires a Number, but received a String."

```ahk
next := DateAdd(20240101, 1, "d")
```

### DateDiff

Answers the difference between two moments; the answer is the first date minus the second, truncated towards zero.

- **Parameters:** DateTime1, DateTime2, Unit
- **Answers:** the difference, as an integer, in the unit that was asked for
- **Errors:** "Parameter #3 of DateDiff is invalid." when the unit is not one of d, h, m and s; a date that v2 cannot read is an invalid parameter.

```ahk
days := DateDiff(20240110, 20240101, "d")
```

### FormatTime

A moment written the way a format says. The date comes first and the format comes last; a text that carries no date at all is the date of the machine.

- **Parameters:** Date, [Format]
- **Answers:** the date written the way the format says
- **Errors:** a date the dialect cannot read is an invalid parameter.

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

### Min

The smallest of the numbers.

- **Parameters:** Number, [Number2], ...
- **Answers:** the smallest of them
- **Errors:** a value that is not a number is refused.

```ahk
Min(3, 7, 5)
```
