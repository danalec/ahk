# Datas e horas

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Idioma: Português (Brasil)

Os nomes da família **Datas e horas** nos três dialetos, com o que esta árvore responde por cada um. A família é a área em que um script pensa; os nomes são a totalidade dela.

| Nome | Dialeto | Tipo | Classe | Parâmetros | Estado | Sonda |
|---|---|---|---|---|---|---|
| `A_Hour` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_Min` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_Mon` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_MSec` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_Now` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_NowUTC` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_Sec` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_WDay` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_YDay` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_Year` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `A_YYYY` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor | -- |
| `DateAdd` | AutoHotkey v2.0.19 | command | -- | 3 | valor | value |
| `DateDiff` | AutoHotkey v2.0.19 | command | -- | 3 | valor | value |
| `DD` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `DDD` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `DDDD` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `FormatTime` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | valor | -- |
| `FormatTime` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor | -- |
| `Hour` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `MDay` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `Min` | AutoHotkey v2.0.19 | variable | -- | -- | valor | value |
| `Min` | AutoHotkey v1.1.37.02 | function | -- | up to 1 | valor | -- |
| `MM` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `MMM` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `MMMM` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `Mon` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `MSec` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `Now` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `NowUTC` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `Sec` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `TickCount` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `WDay` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `YDay` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `Year` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `YWeek` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |
| `YYYY` | AutoHotkey v2.0.19 | variable | -- | -- | valor | -- |

## O que a arvore faz com ele

### A_Hour

A hora da hora local da máquina, com dois dígitos.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** a hora, como um texto de dois dígitos
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Hour, "*"
```

### A_Min

O minuto da hora local da máquina, com dois dígitos.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** o minuto, como um texto de dois dígitos
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Min, "*"
```

### A_Mon

O mês da hora local da máquina. Esta árvore responde os mesmos dois dígitos de `A_MM`, e não o nome abreviado do mês.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** o mês, como um texto de dois dígitos
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Mon, "*"
```

### A_MSec

Os milissegundos da hora local da máquina, com três dígitos. O número do v2 não carrega milissegundos, e este nome carrega.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** os milissegundos, como um texto de três dígitos
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_MSec, "*"
```

### A_Now

A hora local da máquina, escrita como YYYYMMDDHH24MISS.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** a hora, como um texto de quatorze dígitos
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused, because the name is a built-in variable that the dialect does not let a script write.

```ahk
FileAppend A_Now, "*"
```

### A_NowUTC

A hora da máquina em UTC, escrita como YYYYMMDDHH24MISS.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** a hora, como um texto de quatorze dígitos
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_NowUTC, "*"
```

### A_Sec

O segundo da hora local da máquina, com dois dígitos.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** o segundo, como um texto de dois dígitos
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_Sec, "*"
```

### A_WDay

O dia da semana da hora local, sendo 1 o domingo e 7 o sábado.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** o dia da semana, como um texto de um número
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_WDay, "*"
```

### A_YDay

O dia do ano da hora local, sendo 1 o primeiro de janeiro.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** o dia do ano, como um texto de um número
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_YDay, "*"
```

### A_YYYY

O ano da hora local da máquina, com quatro dígitos.

- **Parâmetros:** nothing; it is a variable, not a call
- **Responde:** o ano, como um texto de quatro dígitos
- **Erros:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_YYYY, "*"
```

### DateAdd

Devolve um momento mais um tempo; a data vem primeiro e a unidade por último, e a unidade é lida pela primeira letra.

- **Parâmetros:** DateTime, Number, Unit
- **Responde:** a nova data, como o texto YYYYMMDDHH24MISS
- **Erros:** "Parameter #3 of DateAdd is invalid." when the unit is not one of d, h, m and s; "Parameter #1 of DateAdd is invalid." when the date is not one that v2 can read; "Parameter #2 of DateAdd requires a Number, but received a String."

```ahk
next := DateAdd(20240101, 1, "d")
```

### DateDiff

Devolve a diferença entre dois momentos; a resposta é a primeira data menos a segunda, truncada para zero.

- **Parâmetros:** DateTime1, DateTime2, Unit
- **Responde:** a diferença, como inteiro, na unidade pedida
- **Erros:** "Parameter #3 of DateDiff is invalid." when the unit is not one of d, h, m and s; a date that v2 cannot read is an invalid parameter.

```ahk
days := DateDiff(20240110, 20240101, "d")
```

### FormatTime

Um momento escrito do jeito que o formato diz. A data vem primeiro e o formato por último; um texto sem data nenhuma é a data da máquina.

- **Parâmetros:** Date, [Format]
- **Responde:** a data escrita do jeito que o formato diz
- **Erros:** a date the dialect cannot read is an invalid parameter.

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

### FormatTime

Um momento escrito do jeito que o formato diz. A data vem primeiro e o formato por último; um texto sem data nenhuma é a data da máquina.

- **Parâmetros:** Date, [Format]
- **Responde:** a data escrita do jeito que o formato diz
- **Erros:** a date the dialect cannot read is an invalid parameter.

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

### Min

O menor dos números.

- **Parâmetros:** Number, [Number2], ...
- **Responde:** o menor deles
- **Erros:** a value that is not a number is refused.

```ahk
Min(3, 7, 5)
```

### Min

O menor dos números.

- **Parâmetros:** Number, [Number2], ...
- **Responde:** o menor deles
- **Erros:** a value that is not a number is refused.

```ahk
Min(3, 7, 5)
```
