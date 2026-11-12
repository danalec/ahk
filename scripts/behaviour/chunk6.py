"""The behavior of the built-in variables of the v2 dialect.

    from surface_behavior import behavior_of

Every name of this chunk is a variable that the runtime answers without
the script calling anything. The file says what each one holds, what it
answers and what it refuses, in the three languages of the subtree.
"""

BEHAVIOR = {
    "A_Now": {
        "en": {
            "summary": "The local time of the machine, written as YYYYMMDDHH24MISS.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the time, as a text of fourteen digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused, because the name is a built-in variable that the dialect does not let a script write.",
            "example": 'FileAppend A_Now, "*"',
        },
        "zh": {
            "summary": "机器的本地时间，写作 YYYYMMDDHH24MISS。",
            "returns": "时间，十四位数字的文本",
            "example": 'FileAppend A_Now, "*"',
        },
        "pt-BR": {
            "summary": "A hora local da máquina, escrita como YYYYMMDDHH24MISS.",
            "returns": "a hora, como um texto de quatorze dígitos",
            "example": 'FileAppend A_Now, "*"',
        },
    },
    "A_NowUTC": {
        "en": {
            "summary": "The time of the machine in UTC, written as YYYYMMDDHH24MISS.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the time, as a text of fourteen digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_NowUTC, "*"',
        },
        "zh": {
            "summary": "机器的 UTC 时间，写作 YYYYMMDDHH24MISS。",
            "returns": "时间，十四位数字的文本",
            "example": 'FileAppend A_NowUTC, "*"',
        },
        "pt-BR": {
            "summary": "A hora da máquina em UTC, escrita como YYYYMMDDHH24MISS.",
            "returns": "a hora, como um texto de quatorze dígitos",
            "example": 'FileAppend A_NowUTC, "*"',
        },
    },
    "A_TickCount": {
        "en": {
            "summary": "The number of milliseconds the runtime has counted since an arbitrary origin. The count never goes backwards.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the number of milliseconds, as a number",
            "errors": "the write is refused; the name is a built-in variable that the dialect does not let a script write.",
            "example": 'FileAppend A_TickCount, "*"',
        },
        "zh": {
            "summary": "运行时从一个任意起点开始计数的毫秒数。计数从不后退。",
            "returns": "毫秒数，作为数字",
            "example": 'FileAppend A_TickCount, "*"',
        },
        "pt-BR": {
            "summary": "O número de milissegundos que o runtime conta desde uma origem arbitrária. A contagem nunca volta atrás.",
            "returns": "o número de milissegundos, como um número",
            "example": 'FileAppend A_TickCount, "*"',
        },
    },
    "A_ScriptDir": {
        "en": {
            "summary": "The directory where the run started. The script of the runner arrives on the standard input and has no file of its own, so the directory of the script is the directory of the run.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the path of the directory, as a text",
            "errors": "the read fails when the runtime does not keep the directory where the run started; the write is refused.",
            "example": 'FileAppend A_ScriptDir, "*"',
        },
        "zh": {
            "summary": "运行开始的目录。运行器的脚本来自标准输入，本身没有文件，所以脚本的目录就是运行的目录。",
            "returns": "目录的路径，作为文本",
            "example": 'FileAppend A_ScriptDir, "*"',
        },
        "pt-BR": {
            "summary": "O diretório onde a execução começou. O script do runner chega pela entrada padrão e não tem arquivo próprio, então o diretório do script é o diretório da execução.",
            "returns": "o caminho do diretório, como um texto",
            "example": 'FileAppend A_ScriptDir, "*"',
        },
    },
    "A_ScriptFullPath": {
        "en": {
            "summary": "The path of the script. The script of the runner comes on the standard input and has no file, so the path is an asterisk.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the text \"*\"",
            "errors": "the write is refused.",
            "example": 'FileAppend A_ScriptFullPath, "*"',
        },
        "zh": {
            "summary": "脚本的路径。运行器的脚本来自标准输入，没有文件，所以路径是一个星号。",
            "returns": "文本 \"*\"",
            "example": 'FileAppend A_ScriptFullPath, "*"',
        },
        "pt-BR": {
            "summary": "O caminho do script. O script do runner vem pela entrada padrão e não tem arquivo, então o caminho é um asterisco.",
            "returns": "o texto \"*\"",
            "example": 'FileAppend A_ScriptFullPath, "*"',
        },
    },
    "A_ScriptName": {
        "en": {
            "summary": "The name of the script. The script of the runner comes on the standard input and has no file, so the name is an asterisk. A script may write the name, and the value written is the one it keeps.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the text \"*\", or the text that was written",
            "errors": "nothing to read; the write takes any value.",
            "example": 'FileAppend A_ScriptName, "*"',
        },
        "zh": {
            "summary": "脚本的名称。运行器的脚本来自标准输入，没有文件，所以名称是一个星号。脚本可以写这个名称，写入的值就是它保留的值。",
            "returns": "文本 \"*\"，或写入的文本",
            "example": 'FileAppend A_ScriptName, "*"',
        },
        "pt-BR": {
            "summary": "O nome do script. O script do runner vem pela entrada padrão e não tem arquivo, então o nome é um asterisco. Um script pode escrever o nome, e o valor escrito é o que ele guarda.",
            "returns": "o texto \"*\", ou o texto que foi escrito",
            "example": 'FileAppend A_ScriptName, "*"',
        },
    },
    "A_Index": {
        "en": {
            "summary": "The number of the turn of the loop that runs. It is zero before the first loop, and every loop saves the value it had and gives it back when it ends.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the number of the turn, as a number",
            "errors": "nothing to read; a script may write the name, and the loop goes on from the value that was written.",
            "example": 'for item in [1, 2, 3]\n    FileAppend A_Index, "*"',
        },
        "zh": {
            "summary": "正在运行的循环的轮次编号。第一个循环之前它是零；每个循环都保存它原有的值，并在结束时还回去。",
            "returns": "轮次编号，作为数字",
            "example": 'for item in [1, 2, 3]\n    FileAppend A_Index, "*"',
        },
        "pt-BR": {
            "summary": "O número da volta do laço que está rodando. É zero antes do primeiro laço, e cada laço guarda o valor que tinha e o devolve quando termina.",
            "returns": "o número da volta, como um número",
            "example": 'for item in [1, 2, 3]\n    FileAppend A_Index, "*"',
        },
    },
    "A_LoopField": {
        "en": {
            "summary": "The field the parse loop is on: the piece of the text between two delimiters.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the field, as a text; the empty text before the first parse loop",
            "errors": "nothing to read; the write is refused.",
            "example": 'Loop Parse "a,b,c", ","\n    FileAppend A_LoopField, "*"',
        },
        "zh": {
            "summary": "解析循环所在的字段：文本中两个分隔符之间的一段。",
            "returns": "该字段，作为文本；解析循环之前是空文本",
            "example": 'Loop Parse "a,b,c", ","\n    FileAppend A_LoopField, "*"',
        },
        "pt-BR": {
            "summary": "O campo em que o laço de análise está: o trecho do texto entre dois delimitadores.",
            "returns": "o campo, como um texto; o texto vazio antes do primeiro laço de análise",
            "example": 'Loop Parse "a,b,c", ","\n    FileAppend A_LoopField, "*"',
        },
    },
    "A_LoopFileName": {
        "en": {
            "summary": "The name of the file the file loop is on, without its folder.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the name of the file, as a text; the empty text before the first file loop",
            "errors": "nothing to read; the write is refused.",
            "example": 'Loop Files "*.txt"\n    FileAppend A_LoopFileName, "*"',
        },
        "zh": {
            "summary": "文件循环所在的文件的名称，不含文件夹。",
            "returns": "文件名，作为文本；文件循环之前是空文本",
            "example": 'Loop Files "*.txt"\n    FileAppend A_LoopFileName, "*"',
        },
        "pt-BR": {
            "summary": "O nome do arquivo em que o laço de arquivos está, sem a sua pasta.",
            "returns": "o nome do arquivo, como um texto; o texto vazio antes do primeiro laço de arquivos",
            "example": 'Loop Files "*.txt"\n    FileAppend A_LoopFileName, "*"',
        },
    },
    "A_LoopFileExt": {
        "en": {
            "summary": "The extension of the file the file loop is on, without the dot. A name that begins with a dot has no extension.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the extension, as a text; the empty text before the first file loop",
            "errors": "nothing to read; the write is refused.",
            "example": 'Loop Files "*.txt"\n    FileAppend A_LoopFileExt, "*"',
        },
        "zh": {
            "summary": "文件循环所在文件的扩展名，不含点号。以点号开头的文件名没有扩展名。",
            "returns": "扩展名，作为文本；文件循环之前是空文本",
            "example": 'Loop Files "*.txt"\n    FileAppend A_LoopFileExt, "*"',
        },
        "pt-BR": {
            "summary": "A extensão do arquivo em que o laço de arquivos está, sem o ponto. Um nome que começa com ponto não tem extensão.",
            "returns": "a extensão, como um texto; o texto vazio antes do primeiro laço de arquivos",
            "example": 'Loop Files "*.txt"\n    FileAppend A_LoopFileExt, "*"',
        },
    },
    "A_LoopFileDir": {
        "en": {
            "summary": "The folder that holds the file the file loop is on.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the path of the folder, as a text; the empty text before the first file loop",
            "errors": "nothing to read; the write is refused.",
            "example": 'Loop Files "*.txt"\n    FileAppend A_LoopFileDir, "*"',
        },
        "zh": {
            "summary": "文件循环所在文件所在的文件夹。",
            "returns": "文件夹的路径，作为文本；文件循环之前是空文本",
            "example": 'Loop Files "*.txt"\n    FileAppend A_LoopFileDir, "*"',
        },
        "pt-BR": {
            "summary": "A pasta que contém o arquivo em que o laço de arquivos está.",
            "returns": "o caminho da pasta, como um texto; o texto vazio antes do primeiro laço de arquivos",
            "example": 'Loop Files "*.txt"\n    FileAppend A_LoopFileDir, "*"',
        },
    },
    "A_LoopReadLine": {
        "en": {
            "summary": "The line of the file the read loop is on.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the line, as a text; the empty text before the first read loop",
            "errors": "nothing to read; the write is refused.",
            "example": 'Loop Read "notes.txt"\n    FileAppend A_LoopReadLine, "*"',
        },
        "zh": {
            "summary": "读取循环所在文件的那一行。",
            "returns": "该行，作为文本；读取循环之前是空文本",
            "example": 'Loop Read "notes.txt"\n    FileAppend A_LoopReadLine, "*"',
        },
        "pt-BR": {
            "summary": "A linha do arquivo em que o laço de leitura está.",
            "returns": "a linha, como um texto; o texto vazio antes do primeiro laço de leitura",
            "example": 'Loop Read "notes.txt"\n    FileAppend A_LoopReadLine, "*"',
        },
    },
    "A_LoopRegName": {
        "en": {
            "summary": "The name of the registry key the registry loop is on. The registry is a thing of Windows and this runtime has none: the registry loop fails with \"the registry is a Windows thing and this box has none.\", and the name keeps the empty text.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the empty text; the name is never written by the tree",
            "errors": "nothing to read; the write is refused.",
            "example": 'FileAppend A_LoopRegName, "*"',
        },
        "zh": {
            "summary": "注册表循环所在的注册表项名称。注册表是 Windows 的东西，而这个运行时没有：注册表循环会以“the registry is a Windows thing and this box has none.”失败，这个名字始终是空文本。",
            "returns": "空文本；这个树从不写入它",
            "example": 'FileAppend A_LoopRegName, "*"',
        },
        "pt-BR": {
            "summary": "O nome da chave de registro em que o laço de registro está. O registro é uma coisa do Windows e este runtime não tem um: o laço de registro falha com \"the registry is a Windows thing and this box has none.\", e o nome continua o texto vazio.",
            "returns": "o texto vazio; a árvore nunca escreve esse nome",
            "example": 'FileAppend A_LoopRegName, "*"',
        },
    },
    "A_ErrorLevel": {
        "en": {
            "summary": "Not a built-in variable of the v2 dialect: `ErrorLevel` is the variable the commands of the v1 write, and the v2 has no such name. Here the name is a variable of the script.",
            "params": "nothing; it is a variable, not a call",
            "returns": "nothing: the name has no value until a script writes it",
            "errors": "the read fails with the unset-variable failure until the script writes the name; the write is the write of a variable of the script.",
            "example": 'FileAppend A_ErrorLevel, "*"',
        },
        "zh": {
            "summary": "不是 v2 方言的内置变量：`ErrorLevel` 是 v1 命令写入的变量，v2 没有这个名字。这里这个名字是脚本的普通变量。",
            "returns": "没有：在脚本写入之前它没有值",
            "example": 'FileAppend A_ErrorLevel, "*"',
        },
        "pt-BR": {
            "summary": "Não é uma variável embutida do dialeto v2: `ErrorLevel` é a variável que os comandos do v1 escrevem, e o v2 não tem esse nome. Aqui o nome é uma variável do script.",
            "returns": "nada: o nome não tem valor até um script escrevê-lo",
            "example": 'FileAppend A_ErrorLevel, "*"',
        },
    },
    "A_ThisFunc": {
        "en": {
            "summary": "The name of the function that runs.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the name, as a text; the empty text when no function runs",
            "errors": "nothing to read; the write is refused.",
            "example": 'FileAppend A_ThisFunc, "*"',
        },
        "zh": {
            "summary": "正在运行的函数的名称。",
            "returns": "名称，作为文本；没有函数运行时是空文本",
            "example": 'FileAppend A_ThisFunc, "*"',
        },
        "pt-BR": {
            "summary": "O nome da função que está rodando.",
            "returns": "o nome, como um texto; o texto vazio quando nenhuma função roda",
            "example": 'FileAppend A_ThisFunc, "*"',
        },
    },
    "A_ThisHotkey": {
        "en": {
            "summary": "The name of the hotkey that runs. The hotkeys are the ones of the hotkey system of the runtime, and the name is the empty text when no hotkey runs.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the name of the hotkey, as a text; the empty text when no hotkey runs",
            "errors": "nothing to read; the write is refused.",
            "example": 'FileAppend A_ThisHotkey, "*"',
        },
        "zh": {
            "summary": "正在运行的热键的名称。热键属于运行时的热键系统；没有热键运行时这个名字是空文本。",
            "returns": "热键名称，作为文本；没有热键运行时是空文本",
            "example": 'FileAppend A_ThisHotkey, "*"',
        },
        "pt-BR": {
            "summary": "O nome da hotkey que está rodando. As hotkeys são as do sistema de hotkeys do runtime, e o nome é o texto vazio quando nenhuma hotkey roda.",
            "returns": "o nome da hotkey, como um texto; o texto vazio quando nenhuma hotkey roda",
            "example": 'FileAppend A_ThisHotkey, "*"',
        },
    },
    "A_Hour": {
        "en": {
            "summary": "The hour of the local time of the machine, in two digits.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the hour, as a text of two digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_Hour, "*"',
        },
        "zh": {
            "summary": "机器本地时间的小时，两位数字。",
            "returns": "小时，两位数字的文本",
            "example": 'FileAppend A_Hour, "*"',
        },
        "pt-BR": {
            "summary": "A hora da hora local da máquina, com dois dígitos.",
            "returns": "a hora, como um texto de dois dígitos",
            "example": 'FileAppend A_Hour, "*"',
        },
    },
    "A_Min": {
        "en": {
            "summary": "The minute of the local time of the machine, in two digits.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the minute, as a text of two digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_Min, "*"',
        },
        "zh": {
            "summary": "机器本地时间的分钟，两位数字。",
            "returns": "分钟，两位数字的文本",
            "example": 'FileAppend A_Min, "*"',
        },
        "pt-BR": {
            "summary": "O minuto da hora local da máquina, com dois dígitos.",
            "returns": "o minuto, como um texto de dois dígitos",
            "example": 'FileAppend A_Min, "*"',
        },
    },
    "A_Sec": {
        "en": {
            "summary": "The second of the local time of the machine, in two digits.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the second, as a text of two digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_Sec, "*"',
        },
        "zh": {
            "summary": "机器本地时间的秒，两位数字。",
            "returns": "秒，两位数字的文本",
            "example": 'FileAppend A_Sec, "*"',
        },
        "pt-BR": {
            "summary": "O segundo da hora local da máquina, com dois dígitos.",
            "returns": "o segundo, como um texto de dois dígitos",
            "example": 'FileAppend A_Sec, "*"',
        },
    },
    "A_MSec": {
        "en": {
            "summary": "The milliseconds of the local time of the machine, in three digits. The number of v2 carries no millisecond, and this name does.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the milliseconds, as a text of three digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_MSec, "*"',
        },
        "zh": {
            "summary": "机器本地时间的毫秒，三位数字。v2 的时间数字不带毫秒，而这个名字带。",
            "returns": "毫秒，三位数字的文本",
            "example": 'FileAppend A_MSec, "*"',
        },
        "pt-BR": {
            "summary": "Os milissegundos da hora local da máquina, com três dígitos. O número do v2 não carrega milissegundos, e este nome carrega.",
            "returns": "os milissegundos, como um texto de três dígitos",
            "example": 'FileAppend A_MSec, "*"',
        },
    },
    "A_YDay": {
        "en": {
            "summary": "The day of the year of the local time, 1 being the first of January.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the day of the year, as a text of a number",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_YDay, "*"',
        },
        "zh": {
            "summary": "本地时间中的年内第几天，1 月 1 日是第一天。",
            "returns": "年内第几天，作为数字的文本",
            "example": 'FileAppend A_YDay, "*"',
        },
        "pt-BR": {
            "summary": "O dia do ano da hora local, sendo 1 o primeiro de janeiro.",
            "returns": "o dia do ano, como um texto de um número",
            "example": 'FileAppend A_YDay, "*"',
        },
    },
    "A_YYYY": {
        "en": {
            "summary": "The year of the local time of the machine, in four digits.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the year, as a text of four digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_YYYY, "*"',
        },
        "zh": {
            "summary": "机器本地时间的年份，四位数字。",
            "returns": "年份，四位数字的文本",
            "example": 'FileAppend A_YYYY, "*"',
        },
        "pt-BR": {
            "summary": "O ano da hora local da máquina, com quatro dígitos.",
            "returns": "o ano, como um texto de quatro dígitos",
            "example": 'FileAppend A_YYYY, "*"',
        },
    },
    "A_MM": {
        "en": {
            "summary": "The month of the local time of the machine, in two digits.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the month, as a text of two digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_MM, "*"',
        },
        "zh": {
            "summary": "机器本地时间的月份，两位数字。",
            "returns": "月份，两位数字的文本",
            "example": 'FileAppend A_MM, "*"',
        },
        "pt-BR": {
            "summary": "O mês da hora local da máquina, com dois dígitos.",
            "returns": "o mês, como um texto de dois dígitos",
            "example": 'FileAppend A_MM, "*"',
        },
    },
    "A_Mon": {
        "en": {
            "summary": "The month of the local time of the machine. This tree answers the same two digits as `A_MM`, and not the short name of the month.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the month, as a text of two digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_Mon, "*"',
        },
        "zh": {
            "summary": "机器本地时间的月份。这个树给出的是与 `A_MM` 相同的两位数字，而不是月份的缩写。",
            "returns": "月份，两位数字的文本",
            "example": 'FileAppend A_Mon, "*"',
        },
        "pt-BR": {
            "summary": "O mês da hora local da máquina. Esta árvore responde os mesmos dois dígitos de `A_MM`, e não o nome abreviado do mês.",
            "returns": "o mês, como um texto de dois dígitos",
            "example": 'FileAppend A_Mon, "*"',
        },
    },
    "A_WDay": {
        "en": {
            "summary": "The day of the week of the local time, 1 being the Sunday and 7 the Saturday.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the day of the week, as a text of a number",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_WDay, "*"',
        },
        "zh": {
            "summary": "本地时间中的星期几，1 是星期日，7 是星期六。",
            "returns": "星期几，作为数字的文本",
            "example": 'FileAppend A_WDay, "*"',
        },
        "pt-BR": {
            "summary": "O dia da semana da hora local, sendo 1 o domingo e 7 o sábado.",
            "returns": "o dia da semana, como um texto de um número",
            "example": 'FileAppend A_WDay, "*"',
        },
    },
    "A_DD": {
        "en": {
            "summary": "The day of the month of the local time, in two digits.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the day, as a text of two digits",
            "errors": "the read fails when the clock of the runtime does not know the date of the machine; the write is refused.",
            "example": 'FileAppend A_DD, "*"',
        },
        "zh": {
            "summary": "本地时间中的月内第几天，两位数字。",
            "returns": "该日，两位数字的文本",
            "example": 'FileAppend A_DD, "*"',
        },
        "pt-BR": {
            "summary": "O dia do mês da hora local, com dois dígitos.",
            "returns": "o dia, como um texto de dois dígitos",
            "example": 'FileAppend A_DD, "*"',
        },
    },
    "A_AppData": {
        "en": {
            "summary": "The folder of the data of the user. On a Linux box it is `$XDG_DATA_HOME`, and `$HOME/.local/share` when the variable is not there.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the path of the folder, as a text",
            "errors": "the read fails with \"the machine has no such folder.\" when the machine has no such folder; the write is refused.",
            "example": 'FileAppend A_AppData, "*"',
        },
        "zh": {
            "summary": "用户数据的文件夹。在 Linux 机器上是 `$XDG_DATA_HOME`；该变量不存在时是 `$HOME/.local/share`。",
            "returns": "文件夹的路径，作为文本",
            "example": 'FileAppend A_AppData, "*"',
        },
        "pt-BR": {
            "summary": "A pasta de dados do usuário. Em uma máquina Linux é `$XDG_DATA_HOME`, e `$HOME/.local/share` quando a variável não existe.",
            "returns": "o caminho da pasta, como um texto",
            "example": 'FileAppend A_AppData, "*"',
        },
    },
    "A_Desktop": {
        "en": {
            "summary": "The folder of the desktop of the user: the folder the machine knows, and `$HOME/Desktop` when it knows none.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the path of the folder, as a text",
            "errors": "the read fails with \"the machine has no such folder.\" when the machine has no such folder; the write is refused.",
            "example": 'FileAppend A_Desktop, "*"',
        },
        "zh": {
            "summary": "用户桌面的文件夹：机器知道的文件夹；机器不知道时是 `$HOME/Desktop`。",
            "returns": "文件夹的路径，作为文本",
            "example": 'FileAppend A_Desktop, "*"',
        },
        "pt-BR": {
            "summary": "A pasta da área de trabalho do usuário: a pasta que a máquina conhece, e `$HOME/Desktop` quando ela não conhece nenhuma.",
            "returns": "o caminho da pasta, como um texto",
            "example": 'FileAppend A_Desktop, "*"',
        },
    },
    "A_Temp": {
        "en": {
            "summary": "The folder of the temporary files of the machine: `TMPDIR`, and `/tmp` when the variable is not there.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the path of the folder, as a text",
            "errors": "the read fails with \"the machine has no such folder.\" when the machine has no such folder; the write is refused.",
            "example": 'FileAppend A_Temp, "*"',
        },
        "zh": {
            "summary": "机器的临时文件文件夹：`TMPDIR`；该变量不存在时是 `/tmp`。",
            "returns": "文件夹的路径，作为文本",
            "example": 'FileAppend A_Temp, "*"',
        },
        "pt-BR": {
            "summary": "A pasta de arquivos temporários da máquina: `TMPDIR`, e `/tmp` quando a variável não existe.",
            "returns": "o caminho da pasta, como um texto",
            "example": 'FileAppend A_Temp, "*"',
        },
    },
    "A_UserName": {
        "en": {
            "summary": "The name of the user of the session: the `USER` of the environment.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the name, as a text",
            "errors": "the read fails when the session carries no such name: \"the session does not carry the name of the user: the environment has no USER.\"; the write is refused.",
            "example": 'FileAppend A_UserName, "*"',
        },
        "zh": {
            "summary": "会话用户的名称：环境中的 `USER`。",
            "returns": "名称，作为文本",
            "example": 'FileAppend A_UserName, "*"',
        },
        "pt-BR": {
            "summary": "O nome do usuário da sessão: o `USER` do ambiente.",
            "returns": "o nome, como um texto",
            "example": 'FileAppend A_UserName, "*"',
        },
    },
    "A_ComputerName": {
        "en": {
            "summary": "The name of the computer, as the network sees it: the kernel, and the `HOSTNAME` of the environment when the kernel has no name.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the name, as a text",
            "errors": "the read fails when neither the kernel nor the session carries the name; the write is refused.",
            "example": 'FileAppend A_ComputerName, "*"',
        },
        "zh": {
            "summary": "计算机的名称，按网络看到的：先是内核，内核没有名字时用环境中的 `HOSTNAME`。",
            "returns": "名称，作为文本",
            "example": 'FileAppend A_ComputerName, "*"',
        },
        "pt-BR": {
            "summary": "O nome do computador, como a rede o vê: o kernel, e o `HOSTNAME` do ambiente quando o kernel não tem nome.",
            "returns": "o nome, como um texto",
            "example": 'FileAppend A_ComputerName, "*"',
        },
    },
    "A_OSVersion": {
        "en": {
            "summary": "The version of the operating system of the machine: the release of the kernel, read from `/proc/sys/kernel/osrelease`.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the version, as a text",
            "errors": "the read fails when the release of the kernel cannot be read; the write is refused.",
            "example": 'FileAppend A_OSVersion, "*"',
        },
        "zh": {
            "summary": "机器操作系统的版本：内核的发行版，从 `/proc/sys/kernel/osrelease` 读出。",
            "returns": "版本，作为文本",
            "example": 'FileAppend A_OSVersion, "*"',
        },
        "pt-BR": {
            "summary": "A versão do sistema operacional da máquina: a release do kernel, lida de `/proc/sys/kernel/osrelease`.",
            "returns": "a versão, como um texto",
            "example": 'FileAppend A_OSVersion, "*"',
        },
    },
    "A_IsAdmin": {
        "en": {
            "summary": "Whether the script runs with the rights of an administrator: the effective user id, and the root is the one that is zero.",
            "params": "nothing; it is a variable, not a call",
            "returns": "1 when the script is elevated, 0 when it is not",
            "errors": "the read fails when the machine cannot say; the write is refused.",
            "example": 'FileAppend A_IsAdmin, "*"',
        },
        "zh": {
            "summary": "脚本是否以管理员权限运行：有效用户 ID，root 就是值为零的那个。",
            "returns": "提权时为 1，未提权时为 0",
            "example": 'FileAppend A_IsAdmin, "*"',
        },
        "pt-BR": {
            "summary": "Se o script roda com direitos de administrador: o id de usuário efetivo, e o root é o que é zero.",
            "returns": "1 quando o script está elevado, 0 quando não está",
            "example": 'FileAppend A_IsAdmin, "*"',
        },
    },
    "A_Is64bitOS": {
        "en": {
            "summary": "Whether the machine is a 64-bit one. The tree answers with the width of its own pointer: 1 on a 64-bit build of the runtime, 0 on a 32-bit one.",
            "params": "nothing; it is a variable, not a call",
            "returns": "1 or 0",
            "errors": "nothing to read; the write is refused.",
            "example": 'FileAppend A_Is64bitOS, "*"',
        },
        "zh": {
            "summary": "机器是否是 64 位。这个树用它自身指针的宽度回答：运行时的 64 位构建是 1，32 位构建是 0。",
            "returns": "1 或 0",
            "example": 'FileAppend A_Is64bitOS, "*"',
        },
        "pt-BR": {
            "summary": "Se a máquina é de 64 bits. A árvore responde com a largura do próprio ponteiro: 1 numa compilação de 64 bits do runtime, 0 numa de 32 bits.",
            "returns": "1 ou 0",
            "example": 'FileAppend A_Is64bitOS, "*"',
        },
    },
    "A_Clipboard": {
        "en": {
            "summary": "The text of the clipboard of the desktop. A script may write it, and the concatenation adds to what is there.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the text of the clipboard",
            "errors": "the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.",
            "example": 'A_Clipboard := "hello"\nFileAppend A_Clipboard, "*"',
        },
        "zh": {
            "summary": "桌面剪贴板中的文本。脚本可以写它，连接运算会在已有内容后追加。",
            "returns": "剪贴板中的文本",
            "example": 'A_Clipboard := "hello"\nFileAppend A_Clipboard, "*"',
        },
        "pt-BR": {
            "summary": "O texto da área de transferência da área de trabalho. Um script pode escrevê-la, e a concatenação soma ao que já está lá.",
            "returns": "o texto da área de transferência",
            "example": 'A_Clipboard := "hello"\nFileAppend A_Clipboard, "*"',
        },
    },
    "A_KeyDelay": {
        "en": {
            "summary": "The delay between the keystrokes of a `Send`, in milliseconds. It is a setting of the run, and it is ten by default.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the delay, as a number",
            "errors": "the write takes the number of the value, and a text that holds no number is zero.",
            "example": 'FileAppend A_KeyDelay, "*"',
        },
        "zh": {
            "summary": "`Send` 发送按键之间的延迟，单位是毫秒。它是运行的设置项，默认是十。",
            "returns": "延迟，作为数字",
            "example": 'FileAppend A_KeyDelay, "*"',
        },
        "pt-BR": {
            "summary": "O atraso entre as teclas de um `Send`, em milissegundos. É uma configuração da execução, e vale dez por padrão.",
            "returns": "o atraso, como um número",
            "example": 'FileAppend A_KeyDelay, "*"',
        },
    },
    "A_SendMode": {
        "en": {
            "summary": "The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the name of the mode, as a text",
            "errors": "the write takes the name of one of the four modes, and any other name is refused with \"Invalid value.\".",
            "example": 'FileAppend A_SendMode, "*"',
        },
        "zh": {
            "summary": "`Send` 使用的方法：`Input`、`Event`、`Play` 或 `InputThenPlay`。",
            "returns": "模式的名称，作为文本",
            "example": 'FileAppend A_SendMode, "*"',
        },
        "pt-BR": {
            "summary": "O método que um `Send` usa: `Input`, `Event`, `Play` ou `InputThenPlay`.",
            "returns": "o nome do modo, como um texto",
            "example": 'FileAppend A_SendMode, "*"',
        },
    },
    "A_Cursor": {
        "en": {
            "summary": "The shape of the cursor of the pointer. The shape belongs to the machine, and this runtime does not read it.",
            "params": "nothing; it is a variable, not a call",
            "returns": "nothing: the name refuses",
            "errors": "the read refuses with \"the shape of the cursor of the machine is not read by this runtime.\"",
            "example": 'FileAppend A_Cursor, "*"',
        },
        "zh": {
            "summary": "指针光标的形状。形状属于机器，而这个运行时不去读它。",
            "returns": "没有：这个名字拒绝回答",
            "example": 'FileAppend A_Cursor, "*"',
        },
        "pt-BR": {
            "summary": "A forma do cursor do ponteiro. A forma pertence à máquina, e este runtime não a lê.",
            "returns": "nada: o nome recusa",
            "example": 'FileAppend A_Cursor, "*"',
        },
    },
    "A_ScreenWidth": {
        "en": {
            "summary": "The width of the screen of the desktop, in pixels.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the width, as a number",
            "errors": "the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.",
            "example": 'FileAppend A_ScreenWidth, "*"',
        },
        "zh": {
            "summary": "桌面屏幕的宽度，单位是像素。",
            "returns": "宽度，作为数字",
            "example": 'FileAppend A_ScreenWidth, "*"',
        },
        "pt-BR": {
            "summary": "A largura da tela da área de trabalho, em pixels.",
            "returns": "a largura, como um número",
            "example": 'FileAppend A_ScreenWidth, "*"',
        },
    },
    "A_ScreenHeight": {
        "en": {
            "summary": "The height of the screen of the desktop, in pixels.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the height, as a number",
            "errors": "the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.",
            "example": 'FileAppend A_ScreenHeight, "*"',
        },
        "zh": {
            "summary": "桌面屏幕的高度，单位是像素。",
            "returns": "高度，作为数字",
            "example": 'FileAppend A_ScreenHeight, "*"',
        },
        "pt-BR": {
            "summary": "A altura da tela da área de trabalho, em pixels.",
            "returns": "a altura, como um número",
            "example": 'FileAppend A_ScreenHeight, "*"',
        },
    },
    "A_LoopFileSize": {
        "en": {
            "summary": "The size in bytes of the file the file loop is on. The file loop of the v2 does not set it: the loop of files of the v1 is the one that writes the size, and a script may write the name itself.",
            "params": "nothing; it is a variable, not a call",
            "returns": "the size, as a number; the name has no value until it is written",
            "errors": "the read fails with the unset-variable failure until the name has a value.",
            "example": 'FileAppend A_LoopFileSize, "*"',
        },
        "zh": {
            "summary": "文件循环所在文件的字节大小。v2 的文件循环不设置它：写这个大小的是 v1 的文件循环，脚本自己也可以写这个名字。",
            "returns": "大小，作为数字；这个名字在被写入之前没有值",
            "example": 'FileAppend A_LoopFileSize, "*"',
        },
        "pt-BR": {
            "summary": "O tamanho em bytes do arquivo em que o laço de arquivos está. O laço de arquivos do v2 não o define: é o laço de arquivos do v1 que escreve o tamanho, e um script pode escrever o nome.",
            "returns": "o tamanho, como um número; o nome não tem valor até ser escrito",
            "example": 'FileAppend A_LoopFileSize, "*"',
        },
    },
}
