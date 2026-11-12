"""The behavior of the names of the third chunk: the files of the machine.

    from surface_behavior import behavior_of

The names here are the ones a script uses to read a file, to write one,
to move one, to ask a question about it and to keep the answer of a
question in an ini file. What each one does is the behavior of THIS
tree: the order of the arguments is the order of the binary of
v2.0.19, the answers are the ones of this tree, and a name that the
tree carries only in part says so.
"""

BEHAVIOR = {
    "FileAppend": {
        "en": {
            "summary": "Adds text at the end of a file. The name of the file may be `*`, and the text then goes to the output of the report; inside the body of a `Loop Read` the name may be left out, and the file of the loop is the one that is written.",
            "params": "Text, FileName, [Options]",
            "returns": "nothing",
            "errors": "ValueError when the name of the file is empty or when an option is not one of v2; OSError when the file cannot be opened for writing. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and a linefeed; a code page the tree does not carry is refused with the reason.",
            "example": 'FileAppend "hello`n", "out.txt"',
        },
        "zh": {
            "summary": "把文本追加到文件末尾。文件名可以是 `*`，此时文本进入报告的输出；在 `Loop Read` 的循环体内可以省略文件名，写入的是循环自己的文件。",
            "returns": "无返回值",
            "example": 'FileAppend "hello`n", "out.txt"',
        },
        "pt-BR": {
            "summary": "Acrescenta texto ao fim de um arquivo. O nome do arquivo pode ser `*`, e o texto vai então para a saída do relatório; dentro do corpo de um `Loop Read` o nome pode ser omitido, e o arquivo do laço é o que é escrito.",
            "returns": "nada",
            "example": 'FileAppend "hello`n", "out.txt"',
        },
    },
    "FileRead": {
        "en": {
            "summary": "Reads a whole file and answers its text. The mark of the encoding decides the encoding of the file, and the option of the script decides when the file has no mark. The value of the command is taken with the parenthesis, the way `FileRead(\"out.txt\")` is written.",
            "params": "FileName, [Options]",
            "returns": "the text of the file; a Buffer when the option `RAW` is there.",
            "errors": "OSError when the file cannot be read; ValueError when an option is not one of v2. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and `m<n>`, the number of bytes to read.",
            "example": 'text := FileRead("out.txt")',
        },
        "zh": {
            "summary": "读出整个文件，答案是它的文本。编码标记决定文件用什么编码读取；文件没有标记时由脚本的选项决定。",
            "returns": "文件的文本；带 `RAW` 选项时是一个 Buffer。",
            "example": 'text := FileRead("out.txt")',
        },
        "pt-BR": {
            "summary": "Lê um arquivo inteiro e devolve o seu texto. A marca da codificação decide a codificação do arquivo, e a opção do script decide quando o arquivo não tem marca.",
            "returns": "o texto do arquivo; um Buffer quando a opção `RAW` está presente.",
            "example": 'text := FileRead("out.txt")',
        },
    },
    "FileOpen": {
        "en": {
            "summary": "Opens a file and answers the object of it, of the class `File`. The modes are `r`, `w`, `a` and `rw`; the name `*` is the console of the machine.",
            "params": "FileName, Mode, [Encoding]",
            "returns": "a `File` object.",
            "errors": "ValueError when the mode is not one of the four; OSError when the file cannot be opened.",
            "example": 'f := FileOpen("out.txt", "w")',
        },
        "zh": {
            "summary": "打开文件并返回它的对象（`File` 类）。模式有 `r`、`w`、`a`、`rw`；文件名 `*` 是机器的控制台。",
            "returns": "一个 `File` 对象。",
            "example": 'f := FileOpen("out.txt", "w")',
        },
        "pt-BR": {
            "summary": "Abre um arquivo e devolve o objeto dele, da classe `File`. Os modos são `r`, `w`, `a` e `rw`; o nome `*` é o console da máquina.",
            "returns": "um objeto `File`.",
            "example": 'f := FileOpen("out.txt", "w")',
        },
    },
    "FileDelete": {
        "en": {
            "summary": "Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.",
            "params": "FileName",
            "returns": "nothing",
            "errors": "OSError when the file cannot be taken away.",
            "example": 'FileDelete "out.txt"',
        },
        "zh": {
            "summary": "删除一个文件，或删除某个模式匹配到的所有文件。模式没有匹配到文件不算失败。",
            "returns": "无返回值",
            "example": 'FileDelete "out.txt"',
        },
        "pt-BR": {
            "summary": "Apaga um arquivo, ou todos os arquivos que um padrão nomeia. Um padrão que não encontra nenhum arquivo não é uma falha.",
            "returns": "nada",
            "example": 'FileDelete "out.txt"',
        },
    },
    "FileExist": {
        "en": {
            "summary": "Answers the attributes of the first entry that the name or the pattern matches. The value of the command is taken with the parenthesis, the way `FileExist(\"a.txt\")` is written.",
            "params": "FileName",
            "returns": "the attributes as letters: `A` for a file, `D` for a folder, `RA` for a file that cannot be written, and the empty text when there is no such file.",
            "errors": "nothing fails: a name that matches nothing answers the empty text.",
            "example": 'x := FileExist("out.txt")',
        },
        "zh": {
            "summary": "回答名称或模式匹配到的第一个条目的属性。",
            "returns": "以字母表示的属性：文件是 `A`，目录是 `D`，只读文件是 `RA`，没有这样的文件时是空文本。",
            "example": 'x := FileExist("out.txt")',
        },
        "pt-BR": {
            "summary": "Devolve os atributos da primeira entrada que o nome ou o padrão encontra.",
            "returns": "os atributos em letras: `A` para um arquivo, `D` para uma pasta, `RA` para um arquivo que não pode ser escrito, e o texto vazio quando não há esse arquivo.",
            "example": 'x := FileExist("out.txt")',
        },
    },
    "FileGetSize": {
        "en": {
            "summary": "Answers how big a file is.",
            "params": "FileName, [Unit]",
            "returns": "the size, rounded down; a folder answers zero.",
            "errors": "ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.",
            "example": 'n := FileGetSize("out.txt", "K")',
        },
        "zh": {
            "summary": "回答一个文件有多大。",
            "returns": "大小，向下取整；目录是零。",
            "example": 'n := FileGetSize("out.txt", "K")',
        },
        "pt-BR": {
            "summary": "Devolve o tamanho de um arquivo.",
            "returns": "o tamanho, arredondado para baixo; uma pasta devolve zero.",
            "example": 'n := FileGetSize("out.txt", "K")',
        },
    },
    "FileGetTime": {
        "en": {
            "summary": "Answers when a file was made, when it was changed, or when it was read.",
            "params": "FileName, [Which]",
            "returns": "the stamp of v2, the fourteen digits of `YYYYMMDDHH24MISS`, in the local time of the machine.",
            "errors": "ValueError when `Which` is not `M`, `C`, `A` or `CUT`; OSError when the file cannot be read.",
            "example": 't := FileGetTime("out.txt", "M")',
        },
        "zh": {
            "summary": "回答文件是什么时候创建的、什么时候被修改的、什么时候被读取的。",
            "returns": "v2 的时间戳，`YYYYMMDDHH24MISS` 的十四位数字，用机器的本地时间。",
            "example": 't := FileGetTime("out.txt", "M")',
        },
        "pt-BR": {
            "summary": "Devolve quando um arquivo foi criado, quando foi alterado, ou quando foi lido.",
            "returns": "o carimbo do v2, os catorze dígitos de `YYYYMMDDHH24MISS`, no horário local da máquina.",
            "example": 't := FileGetTime("out.txt", "M")',
        },
    },
    "FileSetAttrib": {
        "en": {
            "summary": "Changes the attributes of a file. The attributes come first and the name of the file second, the way the binary of v2.0.19 takes them. The name of the file may be a pattern, and the attributes are changed in every file that the pattern names.",
            "params": "Attributes, FileName, [Recurse]",
            "returns": "nothing",
            "errors": "ValueError when the attributes are not one of v2; OSError when the attributes of the file cannot be changed. The third parameter is the recursion: the \"R\" walks into the subfolders, and any other letter is refused.",
            "example": 'FileSetAttrib "+H", "*.txt"',
        },
        "zh": {
            "summary": "修改文件的属性。参数顺序与 v2.0.19 的二进制一致：先是属性，后是文件名。文件名可以是模式，模式匹配的每个文件都会被修改。",
            "returns": "无返回值",
            "example": 'FileSetAttrib "+H", "*.txt"',
        },
        "pt-BR": {
            "summary": "Altera os atributos de um arquivo. Os atributos vêm primeiro e o nome do arquivo depois, como o binário do v2.0.19 os recebe. O nome do arquivo pode ser um padrão, e os atributos mudam em todo arquivo que o padrão nomeia.",
            "returns": "nada",
            "example": 'FileSetAttrib "+H", "*.txt"',
        },
    },
    "FileSetTime": {
        "en": {
            "summary": "Changes the time of a file. The time comes first and the name of the file second, the way the binary of v2.0.19 takes them; a time that is blank is the moment of the machine. The name of the file may be a pattern, and the time is changed in every file that the pattern names.",
            "params": "TimeStamp, FileName, [Which], [Recurse]",
            "returns": "nothing",
            "errors": "ValueError when the stamp is not one of v2, or when the time that is wanted is not the \"M\", the \"C\" or the \"A\"; OSError when the times of the file cannot be changed. The fourth parameter is the recursion: the \"R\" walks into the subfolders, and any other letter is refused.",
            "example": 'FileSetTime "20000101000000", "out.txt", "M"',
        },
        "zh": {
            "summary": "修改文件的时间。参数顺序与 v2.0.19 的二进制一致：先是时间，后是文件名；时间留空表示机器的当前时刻。文件名可以是模式，模式匹配的每个文件都会被改变。",
            "returns": "无返回值",
            "example": 'FileSetTime "20000101000000", "out.txt", "M"',
        },
        "pt-BR": {
            "summary": "Altera o horário de um arquivo. O tempo vem primeiro e o nome do arquivo depois, como o binário do v2.0.19 os recebe; um tempo em branco é o momento da máquina. O nome do arquivo pode ser um padrão, e o horário muda em todo arquivo que o padrão nomeia.",
            "returns": "nada",
            "example": 'FileSetTime "20000101000000", "out.txt", "M"',
        },
    },
    "FileCopy": {
        "en": {
            "summary": "Copies a file, or every file that a pattern names. A destination that ends with a separator, or that is a folder, is the folder the files are copied into.",
            "params": "Source, Dest, [Overwrite]",
            "returns": "nothing",
            "errors": "Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.",
            "example": 'FileCopy "a.txt", "b.txt"',
        },
        "zh": {
            "summary": "复制一个文件，或复制某个模式匹配到的所有文件。目标以分隔符结尾、或者目标本身是目录时，文件被复制进那个目录。",
            "returns": "无返回值",
            "example": 'FileCopy "a.txt", "b.txt"',
        },
        "pt-BR": {
            "summary": "Copia um arquivo, ou todos os arquivos que um padrão nomeia. Um destino que termina em separador, ou que é uma pasta, é a pasta para onde os arquivos são copiados.",
            "returns": "nada",
            "example": 'FileCopy "a.txt", "b.txt"',
        },
    },
    "FileMove": {
        "en": {
            "summary": "Moves a file, or every file that a pattern names. The source and the destination are the ones of `FileCopy`.",
            "params": "Source, Dest, [Overwrite]",
            "returns": "nothing",
            "errors": "Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.",
            "example": 'FileMove "a.txt", "b.txt"',
        },
        "zh": {
            "summary": "移动一个文件，或移动某个模式匹配到的所有文件。源和目标的规则与 `FileCopy` 相同。",
            "returns": "无返回值",
            "example": 'FileMove "a.txt", "b.txt"',
        },
        "pt-BR": {
            "summary": "Move um arquivo, ou todos os arquivos que um padrão nomeia. A origem e o destino são os de `FileCopy`.",
            "returns": "nada",
            "example": 'FileMove "a.txt", "b.txt"',
        },
    },
    "FileCreateShortcut": {
        "en": {
            "summary": "Makes a shortcut to a program. On this desktop a shortcut is a `.desktop` file of the freedesktop specification, and it is made even for a target that is not there.",
            "params": "Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]",
            "returns": "nothing",
            "errors": "Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.",
            "example": 'FileCreateShortcut "/bin/ls", "ls.desktop"',
        },
        "zh": {
            "summary": "为一个程序创建快捷方式。在这个桌面上，快捷方式是一个 freedesktop 规范的 `.desktop` 文件；即使目标不存在也会创建。",
            "returns": "无返回值",
            "example": 'FileCreateShortcut "/bin/ls", "ls.desktop"',
        },
        "pt-BR": {
            "summary": "Cria um atalho para um programa. Neste desktop um atalho é um arquivo `.desktop` da especificação freedesktop, e ele é criado mesmo quando o alvo não existe.",
            "returns": "nada",
            "example": 'FileCreateShortcut "/bin/ls", "ls.desktop"',
        },
    },
    "DirCreate": {
        "en": {
            "summary": "Makes a directory, with the folders that are missing in the way.",
            "params": "DirName",
            "returns": "nothing",
            "errors": "OSError when the directory cannot be made.",
            "example": 'DirCreate "out"',
        },
        "zh": {
            "summary": "创建目录，途中缺少的目录也一并创建。",
            "returns": "无返回值",
            "example": 'DirCreate "out"',
        },
        "pt-BR": {
            "summary": "Cria um diretório, com as pastas que faltam no caminho.",
            "returns": "nada",
            "example": 'DirCreate "out"',
        },
    },
    "DirExist": {
        "en": {
            "summary": "Answers the attributes of the first folder that the name or the pattern matches.",
            "params": "DirName",
            "returns": "`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.",
            "errors": "nothing fails.",
            "example": 'x := DirExist("out")',
        },
        "zh": {
            "summary": "回答名称或模式匹配到的第一个目录的属性。",
            "returns": "目录存在时是 `D`，不存在时是空文本；匹配到的是文件时不算目录。",
            "example": 'x := DirExist("out")',
        },
        "pt-BR": {
            "summary": "Devolve os atributos da primeira pasta que o nome ou o padrão encontra.",
            "returns": "`D` quando a pasta existe, e o texto vazio quando não existe; um arquivo que casa com o padrão não é uma pasta.",
            "example": 'x := DirExist("out")',
        },
    },
    "DirDelete": {
        "en": {
            "summary": "Takes a directory away. A directory that is not empty stays without it unless the recursion is asked for.",
            "params": "DirName, [Recurse]",
            "returns": "nothing; the name of a command answers nothing.",
            "errors": "Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.",
            "example": 'd := DirDelete("out", 1)',
        },
        "zh": {
            "summary": "删除一个目录。目录非空时，除非要求递归，否则目录会保留下来。",
            "returns": "无返回值；命令不回答任何东西。",
            "example": 'd := DirDelete("out", 1)',
        },
        "pt-BR": {
            "summary": "Apaga um diretório. Um diretório que não está vazio permanece, a não ser que a recursão seja pedida.",
            "returns": "nada; o nome de um comando não responde nada.",
            "example": 'd := DirDelete("out", 1)',
        },
    },
    "DirCopy": {
        "en": {
            "summary": "Copies a directory and everything in it. What is copied is the content of the directory into the destination.",
            "params": "Source, Dest, [Overwrite]",
            "returns": "nothing",
            "errors": "Error `Failed` when the source is not a directory, or when the destination is there and the overwrite is not asked for; a name that holds a wildcard is refused, and the reference fails with `Failed` on a wildcard too.",
            "example": 'DirCopy "src", "dst"',
        },
        "zh": {
            "summary": "复制一个目录及其中的全部内容。复制过去的是目录的内容。",
            "returns": "无返回值",
            "example": 'DirCopy "src", "dst"',
        },
        "pt-BR": {
            "summary": "Copia um diretório e tudo o que há nele. O que é copiado é o conteúdo do diretório para o destino.",
            "returns": "nada",
            "example": 'DirCopy "src", "dst"',
        },
    },
    "DirMove": {
        "en": {
            "summary": "Moves a directory, with everything in it. A destination that is there is merged with the one that comes: the files of the source win.",
            "params": "Source, Dest, [Overwrite]",
            "returns": "nothing",
            "errors": "Error `Failed` when the source is not a directory; a name that holds a wildcard is refused, and the reference fails with `Failed` on a wildcard too.",
            "example": 'DirMove "src", "dst"',
        },
        "zh": {
            "summary": "移动一个目录及其中的全部内容。目标已经存在时与来源合并：来源里的文件优先。",
            "returns": "无返回值",
            "example": 'DirMove "src", "dst"',
        },
        "pt-BR": {
            "summary": "Move um diretório, com tudo o que há nele. Um destino que já existe é mesclado com o que vem: os arquivos da origem vencem.",
            "returns": "nada",
            "example": 'DirMove "src", "dst"',
        },
    },
    "IniRead": {
        "en": {
            "summary": "Reads the value of a key of an ini file. With no section it answers the names of the sections; with no key it answers the section itself. The ini files are the ones of Windows: `[section]`, `key=value`, and `;` for a comment.",
            "params": "FileName, Section, Key, [Default]",
            "returns": "the value of the key; the default of the script when the key is not there and a default is given.",
            "errors": "Error `The requested key, section or file was not found.` when the key is not there and no default is given.",
            "example": 'v := IniRead("app.ini", "Main", "Name")',
        },
        "zh": {
            "summary": "读取 ini 文件中某个键的值。不给节名时回答所有节名；不给键名时回答整个节。ini 文件按 Windows 的写法：`[section]`、`key=value`、`;` 是注释。",
            "returns": "键的值；键不存在而脚本给了默认值时，回答默认值。",
            "example": 'v := IniRead("app.ini", "Main", "Name")',
        },
        "pt-BR": {
            "summary": "Lê o valor de uma chave de um arquivo ini. Sem seção, devolve os nomes das seções; sem chave, devolve a seção inteira. Os arquivos ini são os do Windows: `[section]`, `key=value` e `;` para comentário.",
            "returns": "o valor da chave; o padrão do script quando a chave não existe e um padrão foi dado.",
            "example": 'v := IniRead("app.ini", "Main", "Name")',
        },
    },
    "IniWrite": {
        "en": {
            "summary": "Writes a key of an ini file. The value comes first and the name of the file second, the way the binary of v2.0.19 takes them; a file that is not there is made.",
            "params": "Value, FileName, Section, Key",
            "returns": "nothing",
            "errors": "Error when the file cannot be written.",
            "example": 'IniWrite "hello", "app.ini", "Main", "Name"',
        },
        "zh": {
            "summary": "写入 ini 文件的一个键。参数顺序与 v2.0.19 的二进制一致：先是值，后是文件名；文件不存在时会被创建。",
            "returns": "无返回值",
            "example": 'IniWrite "hello", "app.ini", "Main", "Name"',
        },
        "pt-BR": {
            "summary": "Escreve uma chave de um arquivo ini. O valor vem primeiro e o nome do arquivo depois, como o binário do v2.0.19 os recebe; um arquivo que não existe é criado.",
            "returns": "nada",
            "example": 'IniWrite "hello", "app.ini", "Main", "Name"',
        },
    },
    "IniDelete": {
        "en": {
            "summary": "Takes a key of an ini file away. A key that is not given takes the whole section with it.",
            "params": "FileName, Section, [Key]",
            "returns": "nothing",
            "errors": "Error when the file cannot be written.",
            "example": 'IniDelete "app.ini", "Main", "Name"',
        },
        "zh": {
            "summary": "删除 ini 文件的一个键。不给键名时，整个节一起删除。",
            "returns": "无返回值",
            "example": 'IniDelete "app.ini", "Main", "Name"',
        },
        "pt-BR": {
            "summary": "Tira uma chave de um arquivo ini. Uma chave que não é dada leva a seção inteira com ela.",
            "returns": "nada",
            "example": 'IniDelete "app.ini", "Main", "Name"',
        },
    },
    "Download": {
        "en": {
            "summary": "Fetches a file of the network and writes it on the disk. The box does it with `curl`; the flag of the cache that v2 writes in front of the url is accepted and dropped.",
            "params": "URL, FileName",
            "returns": "nothing",
            "errors": "Error when the url cannot be reached at all, or when the machine has no `curl`; a file that is not there on the network is not a failure, and the page of the error of the server is what is written.",
            "example": 'Download "https://example.com/x", "x.html"',
        },
        "zh": {
            "summary": "从网络上取一个文件并写到磁盘上。这个盒子用 `curl` 完成；v2 写在 url 前面的缓存标志会被接受并丢弃。",
            "returns": "无返回值",
            "example": 'Download "https://example.com/x", "x.html"',
        },
        "pt-BR": {
            "summary": "Busca um arquivo da rede e o escreve no disco. A caixa faz isso com `curl`; a flag de cache que o v2 escreve na frente da url é aceita e descartada.",
            "returns": "nada",
            "example": 'Download "https://example.com/x", "x.html"',
        },
    },
    "FileRecycle": {
        "en": {
            "summary": "Sends a file, a folder, or the files that a pattern matches, to the trash of the machine. The box uses the trash of the XDG specification, and it is the home directory of the user that holds it.",
            "params": "FileName",
            "returns": "nothing",
            "errors": "Error `Failed` when the path is not there; a pattern that matches nothing is not a failure; ValueError when the name is empty; Error when the machine does not say where the home directory of the user is.",
            "example": 'FileRecycle "out.txt"',
        },
        "zh": {
            "summary": "把一个文件、一个目录，或某个模式匹配到的文件送进机器的回收站。这个盒子用的是 XDG 规范的回收站。",
            "returns": "无返回值",
            "example": 'FileRecycle "out.txt"',
        },
        "pt-BR": {
            "summary": "Manda um arquivo, uma pasta, ou os arquivos que um padrão casa, para a lixeira da máquina. A caixa usa a lixeira da especificação XDG.",
            "returns": "nada",
            "example": 'FileRecycle "out.txt"',
        },
    },
    "FileInstall": {
        "en": {
            "summary": "Copies a file the way the installer of a compiled script does. Nothing is compiled here: the file is copied. The third parameter is the flag of the overwrite, and a second copy onto a file that is there is a success when the flag is given.",
            "params": "Source, Dest, [Overwrite]",
            "returns": "nothing",
            "errors": "Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.",
            "example": 'FileInstall "data.txt", "data.out.txt"',
        },
        "zh": {
            "summary": "像编译后脚本的安装器那样复制一个文件。这里不编译任何东西：只是复制文件。",
            "returns": "无返回值",
            "example": 'FileInstall "data.txt", "data.out.txt"',
        },
        "pt-BR": {
            "summary": "Copia um arquivo como faz o instalador de um script compilado. Nada é compilado aqui: o arquivo é copiado.",
            "returns": "nada",
            "example": 'FileInstall "data.txt", "data.out.txt"',
        },
    },
    "SplitPath": {
        "en": {
            "summary": "Cuts a path into its pieces and writes them into the variables of the caller. A path of a file is cut at the backslash and a url at the slash, and the drive of a url is its scheme and its host.",
            "params": "Path, &OutFileName, &OutDir, &OutExt, &OutNameNoExt, &OutDrive",
            "returns": "nothing",
            "errors": "TypeError when a piece is not a variable to write into.",
            "example": 'SplitPath "C:\\dir\\file.txt", &name, &dir, &ext',
        },
        "zh": {
            "summary": "把路径切成几段，并写进调用者的变量里。文件路径按反斜杠切，网址按斜杠切；网址的盘符是它的方案和主机。",
            "returns": "无返回值",
            "example": 'SplitPath "C:\\dir\\file.txt", &name, &dir, &ext',
        },
        "pt-BR": {
            "summary": "Corta um caminho em suas partes e as escreve nas variáveis de quem chama. Um caminho de arquivo é cortado na barra invertida e uma url na barra, e a unidade de uma url é o seu esquema e o seu host.",
            "returns": "nada",
            "example": 'SplitPath "C:\\dir\\file.txt", &name, &dir, &ext',
        },
    },
}
