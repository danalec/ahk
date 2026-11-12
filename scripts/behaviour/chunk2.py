"""The behavior of the names of the second chunk.

The names are the text and the numbers: the functions that cut a string,
search it, measure it, and the ones that do arithmetic on it.

What is written here is what THIS tree does, not what the reference
dialect does. Where the tree measures a thing in UTF-16 code units, or
answers a text where the reference answers a number, that is written
here, because it is what a script of this tree sees.
"""

BEHAVIOR = {
    "StrLen": {
        "en": {
            "summary": "Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.",
            "params": "Text",
            "returns": "the number of code units",
            "errors": "none; a value that is not text is read as its text.",
            "example": 'StrLen("hello")',
        },
        "zh": {
            "summary": "按 UTF-16 码元统计文本的字符数，因此基本平面之外的字符算作两个。",
            "returns": "码元的数量",
            "example": 'StrLen("hello")',
        },
        "pt-BR": {
            "summary": "Conta os caracteres de um texto em unidades de código UTF-16, então um caractere fora do plano básico conta como dois.",
            "returns": "o número de unidades de código",
            "example": 'StrLen("hello")',
        },
    },
    "SubStr": {
        "en": {
            "summary": "The part of a text between two positions. The positions count from one, and a negative position counts from the end: -1 is the last character.",
            "params": "Text, Start, [Length]",
            "returns": "the text of that part; a negative length leaves that many characters out at the end",
            "errors": "none; the text is cut in the code units of UTF-16, so the second unit of a surrogate pair is a text of its own.",
            "example": 'SubStr("hello", 2, 3)',
        },
        "zh": {
            "summary": "文本中两个位置之间的部分。位置从 1 开始，负位置从末尾倒数，-1 是最后一个字符。",
            "returns": "该部分的文本；负的长度会从末尾去掉相应数量的字符",
            "example": 'SubStr("hello", 2, 3)',
        },
        "pt-BR": {
            "summary": "A parte de um texto entre duas posições. As posições contam a partir de um, e uma posição negativa conta a partir do fim: -1 é o último caractere.",
            "returns": "o texto dessa parte; um comprimento negativo deixa tantos caracteres de fora no fim",
            "example": 'SubStr("hello", 2, 3)',
        },
    },
    "StrReplace": {
        "en": {
            "summary": "The text with one part of it swapped for another. The case sense says whether the letters are taken as they are, and it is off when the caller does not say.",
            "params": "Haystack, Needle, Replacement, [CaseSense], [&Count], [Limit]",
            "returns": "the text with the replacements",
            "errors": "the count must be a variable reference; a needle that is empty changes nothing and the text comes back as it is; the logical case sense is an invalid parameter.",
            "example": 'StrReplace("a-b-c", "-", "+")',
        },
        "zh": {
            "summary": "把文本中的一段换成另一段得到的文本。大小写敏感方式说明是否区分大小写，调用方不说时为不区分。",
            "returns": "替换之后的文本",
            "example": 'StrReplace("a-b-c", "-", "+")',
        },
        "pt-BR": {
            "summary": "O texto com uma parte dele trocada por outra. O sentido de caixa decide se as letras são levadas em conta como são, e fica desligado quando quem chama não diz.",
            "returns": "o texto com as substituições",
            "example": 'StrReplace("a-b-c", "-", "+")',
        },
    },
    "StrSplit": {
        "en": {
            "summary": "The pieces of a text, cut at the delimiters. A delimiter is a literal: a text is one delimiter, an array of texts is a set of them, and the first one of them that is there wins.",
            "params": "Text, [Delimiters], [OmitChars], [MaxParts]",
            "returns": "an array of the pieces; the characters to omit are cut from the beginning and the end of every piece",
            "errors": "the delimiters must be a text or an array of texts; an empty array is refused with \"Parameter #2 of StrSplit is invalid.\".",
            "example": 'StrSplit("a,b,c", ",")',
        },
        "zh": {
            "summary": "文本按分隔符切开的片段。分隔符是字面量：文本是一个分隔符，文本数组是一组分隔符，其中最先出现的那一个获胜。",
            "returns": "片段的数组；要省略的字符会从每个片段的首尾切掉",
            "example": 'StrSplit("a,b,c", ",")',
        },
        "pt-BR": {
            "summary": "As partes de um texto, cortadas nos delimitadores. Um delimitador é literal: um texto é um delimitador, um array de textos é um conjunto deles, e o primeiro que estiver presente vence.",
            "returns": "um array das partes; os caracteres a omitir são cortados do começo e do fim de cada parte",
            "example": 'StrSplit("a,b,c", ",")',
        },
    },
    "StrLower": {
        "en": {
            "summary": "The text in the lower case.",
            "params": "Text",
            "returns": "the text, with the capital letters turned into small ones",
            "errors": "none.",
            "example": 'StrLower("Hello")',
        },
        "zh": {
            "summary": "转为小写的文本。",
            "returns": "大写字母变成小写之后的文本",
            "example": 'StrLower("Hello")',
        },
        "pt-BR": {
            "summary": "O texto em minúsculas.",
            "returns": "o texto, com as maiúsculas viradas em minúsculas",
            "example": 'StrLower("Hello")',
        },
    },
    "StrUpper": {
        "en": {
            "summary": "The text in the upper case.",
            "params": "Text",
            "returns": "the text, with the small letters turned into capital ones",
            "errors": "none.",
            "example": 'StrUpper("hello")',
        },
        "zh": {
            "summary": "转为大写的文本。",
            "returns": "小写字母变成大写之后的文本",
            "example": 'StrUpper("hello")',
        },
        "pt-BR": {
            "summary": "O texto em maiúsculas.",
            "returns": "o texto, com as minúsculas viradas em maiúsculas",
            "example": 'StrUpper("hello")',
        },
    },
    "Trim": {
        "en": {
            "summary": "The text without the characters at its ends.",
            "params": "Text, [Chars]",
            "returns": "the text without the leading and the trailing characters of the set; the set is the space and the tab when the caller does not name one",
            "errors": "none.",
            "example": 'Trim("  hello  ")',
        },
        "zh": {
            "summary": "去掉两端字符后的文本。",
            "returns": "去掉首尾那组字符之后的文本；调用方不指定时这组字符是空格和制表符",
            "example": 'Trim("  hello  ")',
        },
        "pt-BR": {
            "summary": "O texto sem os caracteres das pontas.",
            "returns": "o texto sem os caracteres iniciais e finais do conjunto; o conjunto é o espaço e o tab quando quem chama não nomeia um",
            "example": 'Trim("  hello  ")',
        },
    },
    "LTrim": {
        "en": {
            "summary": "The text without the characters at its left end.",
            "params": "Text, [Chars]",
            "returns": "the text without the leading characters of the set; the set is the space and the tab when the caller does not name one",
            "errors": "none.",
            "example": 'LTrim("  hello  ")',
        },
        "zh": {
            "summary": "去掉左端字符后的文本。",
            "returns": "去掉开头那组字符之后的文本；调用方不指定时这组字符是空格和制表符",
            "example": 'LTrim("  hello  ")',
        },
        "pt-BR": {
            "summary": "O texto sem os caracteres da ponta esquerda.",
            "returns": "o texto sem os caracteres iniciais do conjunto; o conjunto é o espaço e o tab quando quem chama não nomeia um",
            "example": 'LTrim("  hello  ")',
        },
    },
    "RTrim": {
        "en": {
            "summary": "The text without the characters at its right end.",
            "params": "Text, [Chars]",
            "returns": "the text without the trailing characters of the set; the set is the space and the tab when the caller does not name one",
            "errors": "none.",
            "example": 'RTrim("  hello  ")',
        },
        "zh": {
            "summary": "去掉右端字符后的文本。",
            "returns": "去掉末尾那组字符之后的文本；调用方不指定时这组字符是空格和制表符",
            "example": 'RTrim("  hello  ")',
        },
        "pt-BR": {
            "summary": "O texto sem os caracteres da ponta direita.",
            "returns": "o texto sem os caracteres finais do conjunto; o conjunto é o espaço e o tab quando quem chama não nomeia um",
            "example": 'RTrim("  hello  ")',
        },
    },
    "InStr": {
        "en": {
            "summary": "Where a text is in another text. The position counts from one and is zero when the needle is not there at all.",
            "params": "Haystack, Needle, [CaseSense], [Start], [Occurrence]",
            "returns": "the position of the match, or 0 when there is none",
            "errors": "a needle that is empty is an invalid parameter; the logical case sense is an invalid parameter; a start or an occurrence of zero is an invalid parameter.",
            "example": 'InStr("hello", "ll")',
        },
        "zh": {
            "summary": "一段文本在另一段文本中的位置。位置从 1 开始，找不到时为 0。",
            "returns": "匹配的位置；没有匹配时为 0",
            "example": 'InStr("hello", "ll")',
        },
        "pt-BR": {
            "summary": "Onde um texto está em outro texto. A posição conta a partir de um e é zero quando o trecho procurado não está lá.",
            "returns": "a posição da ocorrência, ou 0 quando não há nenhuma",
            "example": 'InStr("hello", "ll")',
        },
    },
    "Format": {
        "en": {
            "summary": "A pattern with the values in it. The pattern is the one of the reference dialect: {1} is the first argument, a placeholder with no number takes the next one, and {{ and }} are one brace of their own.",
            "params": "Pattern, [Value1], [Value2], ...",
            "returns": "the text of the pattern with the values in it",
            "errors": "a placeholder that names an argument that is not there, or whose spec is not one the dialect reads, is left exactly as it was written; a value of a kind the spec does not want is refused.",
            "example": 'Format("{1} + {2} = {3}", 1, 2, 3)',
        },
        "zh": {
            "summary": "把数值填入模式的文本。模式是参考方言的那一套：{1} 是第一个参数，没有编号的占位符取下一个参数，{{ 和 }} 各代表一个花括号。",
            "returns": "填入数值之后的模式文本",
            "example": 'Format("{1} + {2} = {3}", 1, 2, 3)',
        },
        "pt-BR": {
            "summary": "Um padrão com os valores dentro. O padrão é o do dialeto de referência: {1} é o primeiro argumento, um placeholder sem número pega o próximo, e {{ e }} são uma chave cada.",
            "returns": "o texto do padrão com os valores dentro",
            "example": 'Format("{1} + {2} = {3}", 1, 2, 3)',
        },
    },
    "Chr": {
        "en": {
            "summary": "The character with a code point.",
            "params": "Code",
            "returns": "the text of that character",
            "errors": "a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.",
            "example": 'Chr(65)',
        },
        "zh": {
            "summary": "具有某个码位的字符。",
            "returns": "该字符的文本",
            "example": 'Chr(65)',
        },
        "pt-BR": {
            "summary": "O caractere com um ponto de código.",
            "returns": "o texto desse caractere",
            "example": 'Chr(65)',
        },
    },
    "Ord": {
        "en": {
            "summary": "The code point of the first character of a text.",
            "params": "Text",
            "returns": "the code point, or zero when the text has no character",
            "errors": "none; a lone surrogate answers with the code unit of it.",
            "example": 'Ord("A")',
        },
        "zh": {
            "summary": "文本第一个字符的码位。",
            "returns": "码位；文本没有字符时为 0",
            "example": 'Ord("A")',
        },
        "pt-BR": {
            "summary": "O ponto de código do primeiro caractere de um texto.",
            "returns": "o ponto de código, ou zero quando o texto não tem caractere",
            "example": 'Ord("A")',
        },
    },
    "StrGet": {
        "en": {
            "summary": "The text of a buffer, or the text at an address. The source is a buffer or a number; the length and the encoding come after it.",
            "params": "Source, [Length], [Encoding]",
            "returns": "the text; an address that names no buffer and no text answers the empty text",
            "errors": "a text as the source is refused with \"Parameter #1 invalid.\"; an encoding the dialect does not know is refused with \"Invalid Encoding.\".",
            "example": 'StrGet(Buffer(16), 4, "UTF-8")',
        },
        "zh": {
            "summary": "缓冲区的文本，或某个地址处的文本。来源是缓冲区或数字，长度和编码在它之后。",
            "returns": "文本；既不对应缓冲区也不对应文本的地址返回空文本",
            "example": 'StrGet(Buffer(16), 4, "UTF-8")',
        },
        "pt-BR": {
            "summary": "O texto de um buffer, ou o texto num endereço. A origem é um buffer ou um número; o comprimento e a codificação vêm depois.",
            "returns": "o texto; um endereço que não corresponde a buffer nem a texto devolve o texto vazio",
            "example": 'StrGet(Buffer(16), 4, "UTF-8")',
        },
    },
    "StrPut": {
        "en": {
            "summary": "Writes a text into a buffer. Without a target the call answers the number of bytes the text needs, terminator included.",
            "params": "Text, [Target], [Encoding], [Length]",
            "returns": "the number of bytes written, or the number of bytes the text needs when there is no target",
            "errors": "a target that is not a buffer is refused with \"Parameter #2 of StrPut is invalid.\"; a text that does not fit in the size the caller names is refused with \"Invalid Length.\".",
            "example": 'StrPut("abc", Buffer(16), "UTF-8")',
        },
        "zh": {
            "summary": "把文本写进缓冲区。没有目标时，调用返回该文本需要的字节数（含结束符）。",
            "returns": "写入的字节数；没有目标时返回文本需要的字节数",
            "example": 'StrPut("abc", Buffer(16), "UTF-8")',
        },
        "pt-BR": {
            "summary": "Escreve um texto num buffer. Sem alvo, a chamada devolve quantos bytes o texto precisa, com o terminador incluído.",
            "returns": "os bytes escritos, ou os bytes que o texto precisa quando não há alvo",
            "example": 'StrPut("abc", Buffer(16), "UTF-8")',
        },
    },
    "RegExMatch": {
        "en": {
            "summary": "Where a pattern is in a text. The position counts from one and is zero when the pattern is not there.",
            "params": "Text, Pattern, [&Output], [Start]",
            "returns": "the position of the match, or 0 when there is none",
            "errors": "the output must be a variable reference; the output holds an object of the class RegExMatchInfo, and a text that does not match leaves it holding the empty text.",
            "example": 'RegExMatch("abc123", "\\d+")',
        },
        "zh": {
            "summary": "模式在文本中的位置。位置从 1 开始，找不到时为 0。",
            "returns": "匹配的位置；没有匹配时为 0",
            "example": 'RegExMatch("abc123", "\\d+")',
        },
        "pt-BR": {
            "summary": "Onde um padrão está num texto. A posição conta a partir de um e é zero quando o padrão não está lá.",
            "returns": "a posição da ocorrência, ou 0 quando não há nenhuma",
            "example": 'RegExMatch("abc123", "\\d+")',
        },
    },
    "RegExReplace": {
        "en": {
            "summary": "The text with the matches of a pattern replaced. The replacement may name the groups of the pattern with $1 or \\1, and $$ is a dollar of its own.",
            "params": "Text, Pattern, Replacement, [&Count], [Limit], [Start]",
            "returns": "the text with the replacements",
            "errors": "the count must be a variable reference; a pattern that does not compile fails with the reason of the compiler.",
            "example": 'RegExReplace("a1b2", "\\d", "#")',
        },
        "zh": {
            "summary": "把模式的匹配替换掉之后的文本。替换文本可以用 $1 或 \\1 引用模式的分组，$$ 表示一个美元符号本身。",
            "returns": "替换之后的文本",
            "example": 'RegExReplace("a1b2", "\\d", "#")',
        },
        "pt-BR": {
            "summary": "O texto com as ocorrências de um padrão substituídas. A substituição pode nomear os grupos do padrão com $1 ou \\1, e $$ é um cifrão de verdade.",
            "returns": "o texto com as substituições",
            "example": 'RegExReplace("a1b2", "\\d", "#")',
        },
    },
    "Sort": {
        "en": {
            "summary": "The items of a text in order. The second parameter is the options, read letter by letter, and the third is a callback that compares two items.",
            "params": "Text, [Options], [Callback]",
            "returns": "the text with the items in order",
            "errors": 'the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".',
            "example": 'Sort("b`na`n", "N")',
        },
        "zh": {
            "summary": "文本中各条目排好序之后的文本。第二个参数是选项，逐字母读取；第三个参数是比较两个条目的回调。",
            "returns": "条目排序之后的文本",
            "example": 'Sort("b`na`n", "N")',
        },
        "pt-BR": {
            "summary": "Os itens de um texto em ordem. O segundo parâmetro são as opções, lidas letra a letra, e o terceiro é um callback que compara dois itens.",
            "returns": "o texto com os itens em ordem",
            "example": 'Sort("b`na`n", "N")',
        },
    },
    "FormatTime": {
        "en": {
            "summary": "A moment written the way a format says. The date comes first and the format comes last; a text that carries no date at all is the date of the machine.",
            "params": "Date, [Format]",
            "returns": "the date written the way the format says",
            "errors": "a date the dialect cannot read is an invalid parameter.",
            "example": 'FormatTime(A_Now, "yyyy-MM-dd")',
        },
        "zh": {
            "summary": "按格式写出的时刻。日期在前，格式在后；完全不含日期的文本表示机器当前的时刻。",
            "returns": "按格式写出的日期",
            "example": 'FormatTime(A_Now, "yyyy-MM-dd")',
        },
        "pt-BR": {
            "summary": "Um momento escrito do jeito que o formato diz. A data vem primeiro e o formato por último; um texto sem data nenhuma é a data da máquina.",
            "returns": "a data escrita do jeito que o formato diz",
            "example": 'FormatTime(A_Now, "yyyy-MM-dd")',
        },
    },
    "VerCompare": {
        "en": {
            "summary": "Two version strings, compared. The numbers of the two versions are compared one by one, and the component that is missing is a zero. A version with a pre-release suffix is older than the same version without it.",
            "params": "Version1, Version2",
            "returns": "the order of the two versions as -1, 0 or 1; when the second version carries an operator -- <, <=, >, >= or = -- the answer is whether the relation holds",
            "errors": "none.",
            "example": 'VerCompare("1.2.0", "1.10.0")',
        },
        "zh": {
            "summary": "比较两个版本字符串。两个版本的数字逐个比较，缺少的组件按 0 处理。带预发布后缀的版本比不带它的同版本更旧。",
            "returns": "两个版本的顺序，-1、0 或 1；第二个版本带运算符（<、<=、>、>= 或 =）时返回该关系是否成立",
            "example": 'VerCompare("1.2.0", "1.10.0")',
        },
        "pt-BR": {
            "summary": "Duas strings de versão, comparadas. Os números das duas versões são comparados um a um, e o componente que falta vale zero. Uma versão com sufixo de pré-lançamento é mais antiga que a mesma versão sem ele.",
            "returns": "a ordem das duas versões como -1, 0 ou 1; quando a segunda versão traz um operador -- <, <=, >, >= ou = -- a resposta é se a relação vale",
            "example": 'VerCompare("1.2.0", "1.10.0")',
        },
    },
    "Type": {
        "en": {
            "summary": "The name of the type of a value.",
            "params": "Value",
            "returns": "the name: Integer, Float, String, Number, Object, VarRef, Class, or the name of the class of the object",
            "errors": "none.",
            "example": 'Type(1)',
        },
        "zh": {
            "summary": "一个值的类型名。",
            "returns": "类型名：Integer、Float、String、Number、Object、VarRef、Class，或者对象所属的类名",
            "example": 'Type(1)',
        },
        "pt-BR": {
            "summary": "O nome do tipo de um valor.",
            "returns": "o nome: Integer, Float, String, Number, Object, VarRef, Class, ou o nome da classe do objeto",
            "example": 'Type(1)',
        },
    },
    "IsNumber": {
        "en": {
            "summary": "Whether a value is a number: an integer, a float, or a text that holds one. A hexadecimal text is a number too.",
            "params": "Value",
            "returns": "1 when it is a number, 0 when it is not",
            "errors": "a second parameter is refused with \"the code page of the text is not carried yet.\".",
            "example": 'IsNumber("42")',
        },
        "zh": {
            "summary": "一个值是不是数字：整数、浮点数，或是写着数字的文本。十六进制文本也算数字。",
            "returns": "是数字时为 1，否则为 0",
            "example": 'IsNumber("42")',
        },
        "pt-BR": {
            "summary": "Se um valor é um número: um inteiro, um float, ou um texto que contém um. Um texto hexadecimal também é número.",
            "returns": "1 quando é número, 0 quando não é",
            "example": 'IsNumber("42")',
        },
    },
    "IsObject": {
        "en": {
            "summary": "Whether a value is an object. Everything that is not a number and not a text is an object: an array, a map, a class instance and a reference.",
            "params": "Value",
            "returns": "1 when it is an object, 0 when it is not",
            "errors": "none.",
            "example": "IsObject([])",
        },
        "zh": {
            "summary": "一个值是不是对象。凡不是数字也不是文本的都是对象：数组、映射、类实例和引用。",
            "returns": "是对象时为 1，否则为 0",
            "example": "IsObject([])",
        },
        "pt-BR": {
            "summary": "Se um valor é um objeto. Tudo que não é número nem texto é objeto: um array, um map, uma instância de classe e uma referência.",
            "returns": "1 quando é objeto, 0 quando não é",
            "example": "IsObject([])",
        },
    },
    "Abs": {
        "en": {
            "summary": "The distance from zero. An integer stays an integer.",
            "params": "Number",
            "returns": "the number without its sign",
            "errors": "a value that is not a number is refused.",
            "example": "Abs(-3)",
        },
        "zh": {
            "summary": "与零的距离。整数仍然是整数。",
            "returns": "去掉符号后的数字",
            "example": "Abs(-3)",
        },
        "pt-BR": {
            "summary": "A distância até zero. Um inteiro continua inteiro.",
            "returns": "o número sem o seu sinal",
            "example": "Abs(-3)",
        },
    },
    "Ceil": {
        "en": {
            "summary": "The whole number above a number.",
            "params": "Number",
            "returns": "the whole number, a float when the number was not whole",
            "errors": "a value that is not a number is refused.",
            "example": "Ceil(1.2)",
        },
        "zh": {
            "summary": "不小于该数的整数。",
            "returns": "该整数；原来的数不是整数时返回浮点数",
            "example": "Ceil(1.2)",
        },
        "pt-BR": {
            "summary": "O número inteiro acima de um número.",
            "returns": "o inteiro, um float quando o número não era inteiro",
            "example": "Ceil(1.2)",
        },
    },
    "Floor": {
        "en": {
            "summary": "The whole number below a number.",
            "params": "Number",
            "returns": "the whole number, a float when the number was not whole",
            "errors": "a value that is not a number is refused.",
            "example": "Floor(1.8)",
        },
        "zh": {
            "summary": "不大于该数的整数。",
            "returns": "该整数；原来的数不是整数时返回浮点数",
            "example": "Floor(1.8)",
        },
        "pt-BR": {
            "summary": "O número inteiro abaixo de um número.",
            "returns": "o inteiro, um float quando o número não era inteiro",
            "example": "Floor(1.8)",
        },
    },
    "Round": {
        "en": {
            "summary": "The number to the number of decimals the caller asks for. With a positive number of decimals the answer is a text of exactly that many digits after the point, zeros kept; with the number omitted, zero or negative it is a whole number.",
            "params": "Number, [Decimals]",
            "returns": "a text with the decimals asked for, or a whole number",
            "errors": "a text that does not hold a number is refused; more than 100000 decimals is an invalid parameter.",
            "example": "Round(2.567, 2)",
        },
        "zh": {
            "summary": "把数字舍入到调用方指定的小数位数。小数位数为正时返回的是文本，小数点后恰好是该位数的数字并保留零；不给、为 0 或为负时返回整数。",
            "returns": "带有所需小数位的文本，或一个整数",
            "example": "Round(2.567, 2)",
        },
        "pt-BR": {
            "summary": "O número arredondado ao número de decimais pedido. Com um número positivo de decimais a resposta é um texto com exatamente esse tanto de dígitos depois do ponto, zeros incluidos; sem o número, com zero ou negativo, é um inteiro.",
            "returns": "um texto com os decimais pedidos, ou um inteiro",
            "example": "Round(2.567, 2)",
        },
    },
    "Max": {
        "en": {
            "summary": "The largest of the numbers.",
            "params": "Number, [Number2], ...",
            "returns": "the largest of them",
            "errors": "a value that is not a number is refused.",
            "example": "Max(3, 7, 5)",
        },
        "zh": {
            "summary": "这些数字中最大的一个。",
            "returns": "其中最大的数字",
            "example": "Max(3, 7, 5)",
        },
        "pt-BR": {
            "summary": "O maior dos números.",
            "returns": "o maior deles",
            "example": "Max(3, 7, 5)",
        },
    },
    "Min": {
        "en": {
            "summary": "The smallest of the numbers.",
            "params": "Number, [Number2], ...",
            "returns": "the smallest of them",
            "errors": "a value that is not a number is refused.",
            "example": "Min(3, 7, 5)",
        },
        "zh": {
            "summary": "这些数字中最小的一个。",
            "returns": "其中最小的数字",
            "example": "Min(3, 7, 5)",
        },
        "pt-BR": {
            "summary": "O menor dos números.",
            "returns": "o menor deles",
            "example": "Min(3, 7, 5)",
        },
    },
    "Mod": {
        "en": {
            "summary": "The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.",
            "params": "Dividend, Divisor",
            "returns": "the remainder",
            "errors": "a divisor of zero fails with \"Divide by zero.\"; a value that is not a number is refused with \"Invalid parameter(s).\".",
            "example": "Mod(7, 3)",
        },
        "zh": {
            "summary": "除法的余数。余数的符号与被除数相同，两个参数都是整数时结果也是整数。",
            "returns": "余数",
            "example": "Mod(7, 3)",
        },
        "pt-BR": {
            "summary": "O resto de uma divisão. O sinal do resto é o do dividendo, e a resposta é um número inteiro quando os dois argumentos são.",
            "returns": "o resto",
            "example": "Mod(7, 3)",
        },
    },
    "Random": {
        "en": {
            "summary": "A number between the two the caller names. Without arguments it is a number between zero and one; with one it is a whole number below it; with two it is a whole number between them, both ends counted.",
            "params": "[Bound] or [Low, High]",
            "returns": "the number",
            "errors": "none.",
            "example": "Random(1, 6)",
        },
        "zh": {
            "summary": "调用方指定两个数之间的数字。不带参数时是 0 与 1 之间的数；带一个参数是小于它的整数；带两个参数是两者之间的整数，两端都算在内。",
            "returns": "该数字",
            "example": "Random(1, 6)",
        },
        "pt-BR": {
            "summary": "Um número entre os dois que quem chama nomeia. Sem argumentos é um número entre zero e um; com um é um inteiro abaixo dele; com dois é um inteiro entre eles, as duas pontas contadas.",
            "returns": "o número",
            "example": "Random(1, 6)",
        },
    },
    "Sin": {
        "en": {
            "summary": "The sine of an angle, in radians.",
            "params": "Number",
            "returns": "a float, whatever went in",
            "errors": "a value that is not a number is refused.",
            "example": "Sin(0)",
        },
        "zh": {
            "summary": "一个角度（弧度）的正弦。",
            "returns": "浮点数，无论输入是什么",
            "example": "Sin(0)",
        },
        "pt-BR": {
            "summary": "O seno de um ângulo, em radianos.",
            "returns": "um float, qualquer que tenha sido a entrada",
            "example": "Sin(0)",
        },
    },
    "Cos": {
        "en": {
            "summary": "The cosine of an angle, in radians.",
            "params": "Number",
            "returns": "a float, whatever went in",
            "errors": "a value that is not a number is refused.",
            "example": "Cos(0)",
        },
        "zh": {
            "summary": "一个角度（弧度）的余弦。",
            "returns": "浮点数，无论输入是什么",
            "example": "Cos(0)",
        },
        "pt-BR": {
            "summary": "O cosseno de um ângulo, em radianos.",
            "returns": "um float, qualquer que tenha sido a entrada",
            "example": "Cos(0)",
        },
    },
    "Tan": {
        "en": {
            "summary": "The tangent of an angle, in radians.",
            "params": "Number",
            "returns": "a float, whatever went in",
            "errors": "a value that is not a number is refused.",
            "example": "Tan(0)",
        },
        "zh": {
            "summary": "一个角度（弧度）的正切。",
            "returns": "浮点数，无论输入是什么",
            "example": "Tan(0)",
        },
        "pt-BR": {
            "summary": "A tangente de um ângulo, em radianos.",
            "returns": "um float, qualquer que tenha sido a entrada",
            "example": "Tan(0)",
        },
    },
    "Sqrt": {
        "en": {
            "summary": "The square root of a number.",
            "params": "Number",
            "returns": "a float, even for a square",
            "errors": "a negative number is an invalid parameter.",
            "example": "Sqrt(9)",
        },
        "zh": {
            "summary": "一个数的平方根。",
            "returns": "浮点数，即使结果是整数也是浮点数",
            "example": "Sqrt(9)",
        },
        "pt-BR": {
            "summary": "A raiz quadrada de um número.",
            "returns": "um float, mesmo quando o resultado é inteiro",
            "example": "Sqrt(9)",
        },
    },
    "Exp": {
        "en": {
            "summary": "e raised to the number.",
            "params": "Number",
            "returns": "a float",
            "errors": "a value that is not a number is refused.",
            "example": "Exp(1)",
        },
        "zh": {
            "summary": "以该数为指数的 e 的幂。",
            "returns": "浮点数",
            "example": "Exp(1)",
        },
        "pt-BR": {
            "summary": "e elevado ao número.",
            "returns": "um float",
            "example": "Exp(1)",
        },
    },
    "Ln": {
        "en": {
            "summary": "The natural logarithm of a number.",
            "params": "Number",
            "returns": "a float",
            "errors": "a negative number is an invalid parameter; a value that is not a number is refused.",
            "example": "Ln(1)",
        },
        "zh": {
            "summary": "一个数的自然对数。",
            "returns": "浮点数",
            "example": "Ln(1)",
        },
        "pt-BR": {
            "summary": "O logaritmo natural de um número.",
            "returns": "um float",
            "example": "Ln(1)",
        },
    },
}
