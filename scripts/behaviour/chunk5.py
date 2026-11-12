"""The behavior of the fifth chunk of names.

The names of this chunk are the ones that ask the machine about the
windows and the processes, that read and write the environment and the
registry, that call a library, that read the clipboard and the sound,
and that do the arithmetic of the dates.

The tree is a Linux port, and it says so where it matters: the window
family acts on the window the script made and on nothing else, the
process family reads `/proc`, the registry is a store of the runtime's
own, and the libraries of Windows are called through the raven.
"""

BEHAVIOR = {
    "WinMinimize": {
        "en": {
            "summary": "Minimizes the window the script made, and only that one: a window of another program cannot be minimized from a Wayland client.",
            "params": "WinTitle",
            "returns": "the empty text",
            "errors": "\"Target window not found.\" when the script has no window of its own, or when the title does not match the one it has; the reason of the desktop when the helper refuses the state.",
            "example": 'WinMinimize "MyWindow"',
        },
        "zh": {
            "summary": "最小化脚本自己创建的窗口，且只有这一个：Wayland 客户端无法最小化别的程序的窗口。",
            "returns": "空文本",
            "example": 'WinMinimize "MyWindow"',
        },
        "pt-BR": {
            "summary": "Minimiza a janela que o script criou, e apenas ela: um cliente Wayland não minimiza a janela de outro programa.",
            "returns": "o texto vazio",
            "example": 'WinMinimize "MyWindow"',
        },
    },
    "WinMaximize": {
        "en": {
            "summary": "Maximizes the window the script made, and only that one; the helper of the box is the one that knows how to do it.",
            "params": "WinTitle",
            "returns": "the empty text",
            "errors": "\"Target window not found.\" when the script has no window of its own, or when the title does not match the one it has.",
            "example": 'WinMaximize "MyWindow"',
        },
        "zh": {
            "summary": "最大化脚本自己创建的窗口，且只有这一个；由盒子里的辅助程序来完成。",
            "returns": "空文本",
            "example": 'WinMaximize "MyWindow"',
        },
        "pt-BR": {
            "summary": "Maximiza a janela que o script criou, e apenas ela; quem faz isso é o auxiliar da caixa.",
            "returns": "o texto vazio",
            "example": 'WinMaximize "MyWindow"',
        },
    },
    "WinRestore": {
        "en": {
            "summary": "Puts the window of the script back from the minimized state, and only that window.",
            "params": "WinTitle",
            "returns": "the empty text",
            "errors": "\"Target window not found.\" when the script has no window of its own, or when the title does not match the one it has.",
            "example": 'WinRestore "MyWindow"',
        },
        "zh": {
            "summary": "把脚本自己的窗口从最小化状态恢复，且只有这一个窗口。",
            "returns": "空文本",
            "example": 'WinRestore "MyWindow"',
        },
        "pt-BR": {
            "summary": "Devolve a janela do script do estado minimizado, e apenas essa janela.",
            "returns": "o texto vazio",
            "example": 'WinRestore "MyWindow"',
        },
    },
    "WinMove": {
        "en": {
            "summary": "Moves and resizes the window the script made; a Wayland compositor places the windows itself and need not honour the place a client asks for.",
            "params": "X, Y, Width, Height, WinTitle",
            "returns": "the empty text",
            "errors": "\"Target window not found.\" when the script has no window of its own, or when the title does not match the one it has.",
            "example": 'WinMove 100, 100, 800, 600, "MyWindow"',
        },
        "zh": {
            "summary": "移动并调整脚本自己创建的窗口的大小；Wayland 合成器自己摆放窗口，未必会照客户要求的坐标来。",
            "returns": "空文本",
            "example": 'WinMove 100, 100, 800, 600, "MyWindow"',
        },
        "pt-BR": {
            "summary": "Move e redimensiona a janela que o script criou; um compositor Wayland posiciona as janelas por conta própria e pode não aceitar o lugar pedido.",
            "returns": "o texto vazio",
            "example": 'WinMove 100, 100, 800, 600, "MyWindow"',
        },
    },
    "WinWait": {
        "en": {
            "summary": "Waits for a window whose title matches to exist, and answers the identifier of it; the identifier of a Wayland window is a token, so it is text.",
            "params": "WinTitle, WinText, [Timeout], [ExcludeTitle], [ExcludeText]",
            "returns": "the identifier of the window as text, or 0 when the timeout ran out",
            "errors": "a session that is not Wayland answers \"this desktop waits for the windows of a Wayland session, and this session is not Wayland\"; the two exclusions are refused with \"the two exclusions are not carried yet.\"; the wait is capped at sixty seconds when the timeout is left out.",
            "example": 'WinWait "Calculator", , 5',
        },
        "zh": {
            "summary": "等待标题匹配的窗口出现，并给出它的标识符；Wayland 窗口的标识符是一个记号，所以是文本。",
            "returns": "窗口标识符（文本），超时则为 0",
            "example": 'WinWait "Calculator", , 5',
        },
        "pt-BR": {
            "summary": "Espera que exista uma janela cujo título combine, e devolve o identificador dela; o identificador de uma janela Wayland é um token, e portanto texto.",
            "returns": "o identificador da janela como texto, ou 0 quando o tempo acabou",
            "example": 'WinWait "Calculator", , 5',
        },
    },
    "WinWaitActive": {
        "en": {
            "summary": "Waits for a window whose title matches to have the focus, and answers the identifier of it.",
            "params": "WinTitle, WinText, [Timeout], [ExcludeTitle], [ExcludeText]",
            "returns": "the identifier of the window as text, or 0 when the timeout ran out",
            "errors": "a session that is not Wayland answers \"this desktop waits for the windows of a Wayland session, and this session is not Wayland\"; the two exclusions are refused with \"the two exclusions are not carried yet.\"",
            "example": 'WinWaitActive "Calculator", , 5',
        },
        "zh": {
            "summary": "等待标题匹配的窗口获得焦点，并给出它的标识符。",
            "returns": "窗口标识符（文本），超时则为 0",
            "example": 'WinWaitActive "Calculator", , 5',
        },
        "pt-BR": {
            "summary": "Espera que uma janela cujo título combine tenha o foco, e devolve o identificador dela.",
            "returns": "o identificador da janela como texto, ou 0 quando o tempo acabou",
            "example": 'WinWaitActive "Calculator", , 5',
        },
    },
    "WinWaitClose": {
        "en": {
            "summary": "Waits for a window whose title matches to be gone, and answers whether it is.",
            "params": "WinTitle, WinText, [Timeout], [ExcludeTitle], [ExcludeText]",
            "returns": "1 when the window is gone, 0 when it is still there at the timeout",
            "errors": "a session that is not Wayland answers \"this desktop waits for the windows of a Wayland session, and this session is not Wayland\"; the two exclusions are refused with \"the two exclusions are not carried yet.\"",
            "example": 'WinWaitClose "Calculator", , 5',
        },
        "zh": {
            "summary": "等待标题匹配的窗口消失，并回答它是否已经消失。",
            "returns": "窗口已消失为 1，超时时它还在则为 0",
            "example": 'WinWaitClose "Calculator", , 5',
        },
        "pt-BR": {
            "summary": "Espera que uma janela cujo título combine desapareça, e diz se desapareceu.",
            "returns": "1 quando a janela sumiu, 0 quando ela ainda está lá no fim do tempo",
            "example": 'WinWaitClose "Calculator", , 5',
        },
    },
    "WinGetPID": {
        "en": {
            "summary": "Answers the process of a window, through the accessibility bus of the session: the registry of the bus names the applications, a window is a child of one, and the bus answers its PID.",
            "params": "WinTitle",
            "returns": "the pid of the process, as an integer",
            "errors": "\"Target window not found.\" when no window has that title; an empty title is refused with \"the window is found by its title, and an empty title is not carried yet.\"; the text of the window and the exclusions are refused with \"the text of the window and the exclusions are not carried yet.\"",
            "example": 'pid := WinGetPID("Calculator")',
        },
        "zh": {
            "summary": "通过会话的无障碍总线给出窗口所属进程：总线登记应用，窗口是应用的孩子，总线给出它的 PID。",
            "returns": "该进程的 pid（整数）",
            "example": 'pid := WinGetPID("Calculator")',
        },
        "pt-BR": {
            "summary": "Devolve o processo de uma janela, pelo barramento de acessibilidade da sessão: o registro do barramento nomeia os aplicativos e o barramento responde o PID.",
            "returns": "o pid do processo, como inteiro",
            "example": 'pid := WinGetPID("Calculator")',
        },
    },
    "WinGetProcessName": {
        "en": {
            "summary": "Answers the name of the program of a window; it rides the chain of WinGetPID -- the window names an application on the accessibility bus, the bus answers its process, and the process table answers the name.",
            "params": "WinTitle",
            "returns": "the file name of the executable, as text",
            "errors": "\"Target window not found.\" when no window has that title, \"Target process not found.\" when the process is gone by then; an empty title is refused with \"the window is found by its title, and an empty title is not carried yet.\"",
            "example": 'name := WinGetProcessName("Calculator")',
        },
        "zh": {
            "summary": "给出窗口所属程序的名称；它沿着 WinGetPID 的链条走：窗口在无障碍总线上找到应用，总线给出进程，进程表给出名称。",
            "returns": "可执行文件的文件名（文本）",
            "example": 'name := WinGetProcessName("Calculator")',
        },
        "pt-BR": {
            "summary": "Devolve o nome do programa de uma janela; ela segue a cadeia de WinGetPID: a janela nomeia um aplicativo no barramento, o barramento responde o processo e a tabela de processos responde o nome.",
            "returns": "o nome do arquivo do executável, como texto",
            "example": 'name := WinGetProcessName("Calculator")',
        },
    },
    "WinSetTitle": {
        "en": {
            "summary": "Renames the window the script made; renaming the window of another program is not something a Wayland client can do.",
            "params": "NewTitle, WinTitle",
            "returns": "the empty text",
            "errors": "\"Target window not found.\" when the script has no window of its own, or when the title does not match the one it has.",
            "example": 'WinSetTitle "New Title", "MyWindow"',
        },
        "zh": {
            "summary": "给脚本自己的窗口改名；Wayland 客户端无法给别的程序的窗口改名。",
            "returns": "空文本",
            "example": 'WinSetTitle "New Title", "MyWindow"',
        },
        "pt-BR": {
            "summary": "Renomeia a janela que o script criou; renomear a janela de outro programa não é algo que um cliente Wayland consiga fazer.",
            "returns": "o texto vazio",
            "example": 'WinSetTitle "New Title", "MyWindow"',
        },
    },
    "ProcessExist": {
        "en": {
            "summary": "Answers whether a process is there, and with which pid; the processes of the machine are read out of /proc.",
            "params": "[ProcessNameOrPID]",
            "returns": "the pid of the process, or 0 when there is none",
            "errors": "a machine without /proc answers \"the processes of this machine are read out of /proc, and this machine has none\"; a process that is not there is zero and not a failure.",
            "example": 'if ProcessExist("firefox")',
        },
        "zh": {
            "summary": "回答某个进程是否存在、它的 pid 是多少；机器上的进程是从 /proc 读出来的。",
            "returns": "该进程的 pid，不存在则为 0",
            "example": 'if ProcessExist("firefox")',
        },
        "pt-BR": {
            "summary": "Responde se existe um processo e com qual pid; os processos da máquina são lidos de /proc.",
            "returns": "o pid do processo, ou 0 quando não há nenhum",
            "example": 'if ProcessExist("firefox")',
        },
    },
    "ProcessGetName": {
        "en": {
            "summary": "Answers the name of the executable of a process; the name is the file name of /proc/<pid>/exe, and the name the kernel gives is the fallback.",
            "params": "[ProcessNameOrPID]",
            "returns": "the name of the executable, as text",
            "errors": "\"Target process not found.\" when the process is not there; a machine without /proc answers \"the processes of this machine are read out of /proc, and this machine has none\".",
            "example": 'name := ProcessGetName(1234)',
        },
        "zh": {
            "summary": "给出进程可执行文件的名称；名称取自 /proc/<pid>/exe 的文件名，内核给出的名字是退路。",
            "returns": "可执行文件的名称（文本）",
            "example": 'name := ProcessGetName(1234)',
        },
        "pt-BR": {
            "summary": "Devolve o nome do executável de um processo; o nome é o nome do arquivo de /proc/<pid>/exe, e o nome do kernel é o plano B.",
            "returns": "o nome do executável, como texto",
            "example": 'name := ProcessGetName(1234)',
        },
    },
    "ProcessGetPath": {
        "en": {
            "summary": "Answers the whole path of the executable of a process; the path is the target of the link /proc/<pid>/exe, and the \" (deleted)\" the kernel adds is cut.",
            "params": "[ProcessNameOrPID]",
            "returns": "the path of the executable, as text",
            "errors": "\"Target process not found.\" when the process is not there; a machine without /proc answers \"the processes of this machine are read out of /proc, and this machine has none\".",
            "example": 'path := ProcessGetPath(1234)',
        },
        "zh": {
            "summary": "给出进程可执行文件的完整路径；路径是 /proc/<pid>/exe 这个链接指向的地方，内核加上的 \" (deleted)\" 会被截掉。",
            "returns": "可执行文件的路径（文本）",
            "example": 'path := ProcessGetPath(1234)',
        },
        "pt-BR": {
            "summary": "Devolve o caminho inteiro do executável de um processo; o caminho é o alvo da ligação /proc/<pid>/exe, e o \" (deleted)\" que o kernel acrescenta é cortado.",
            "returns": "o caminho do executável, como texto",
            "example": 'path := ProcessGetPath(1234)',
        },
    },
    "ProcessWait": {
        "en": {
            "summary": "Waits for a process to be there; the box checks every hundred milliseconds, the way the binary of v2 does.",
            "params": "ProcessNameOrPID, [Timeout]",
            "returns": "the pid of the process when it appears, 0 at the timeout",
            "errors": "a call without a timeout waits for ever; a machine without /proc answers \"the processes of this machine are read out of /proc, and this machine has none\".",
            "example": 'ProcessWait "firefox", 10',
        },
        "zh": {
            "summary": "等待某个进程出现；盒子每百毫秒检查一次，和 v2 的二进制一样。",
            "returns": "出现时给出该进程的 pid，超时则为 0",
            "example": 'ProcessWait "firefox", 10',
        },
        "pt-BR": {
            "summary": "Espera que um processo exista; a caixa verifica a cada cem milissegundos, como faz o binário do v2.",
            "returns": "o pid do processo quando ele aparece, 0 no fim do tempo",
            "example": 'ProcessWait "firefox", 10',
        },
    },
    "ProcessWaitClose": {
        "en": {
            "summary": "Waits for a process that is there to close; a process that never existed is gone, and the wait is over at once.",
            "params": "ProcessNameOrPID, [Timeout]",
            "returns": "0 when the process is gone, the pid of the process that is still there at the timeout",
            "errors": "a call without a timeout waits for ever; a machine without /proc answers \"the processes of this machine are read out of /proc, and this machine has none\".",
            "example": 'ProcessWaitClose "firefox", 10',
        },
        "zh": {
            "summary": "等待一个已经在的进程结束；从未存在过的进程就是已经没了，等待立刻结束。",
            "returns": "进程已消失为 0，超时时它还在则给出它的 pid",
            "example": 'ProcessWaitClose "firefox", 10',
        },
        "pt-BR": {
            "summary": "Espera que um processo que existe termine; um processo que nunca existiu já sumiu, e a espera acaba na hora.",
            "returns": "0 quando o processo sumiu, o pid do processo que ainda está lá no fim do tempo",
            "example": 'ProcessWaitClose "firefox", 10',
        },
    },
    "ProcessClose": {
        "en": {
            "summary": "Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.",
            "params": "ProcessNameOrPID",
            "returns": "the pid of the process that was closed, 0 when it was not there or could not be closed",
            "errors": "a process of another user is a process the script cannot open, and the answer is zero and not a failure.",
            "example": 'ProcessClose "firefox"',
        },
        "zh": {
            "summary": "结束一个进程；盒子发送 SIGTERM，这是请进程自己退出的礼貌信号。",
            "returns": "被结束的进程的 pid，不存在或无法结束时为 0",
            "example": 'ProcessClose "firefox"',
        },
        "pt-BR": {
            "summary": "Encerra um processo; a caixa envia SIGTERM, o sinal educado que pede ao processo que termine.",
            "returns": "o pid do processo encerrado, 0 quando ele não estava lá ou não pôde ser encerrado",
            "example": 'ProcessClose "firefox"',
        },
    },
    "SysGet": {
        "en": {
            "summary": "Answers a metric of the system; the numbers are the ones of GetSystemMetrics, and only the ones about the screen are carried.",
            "params": "Metric",
            "returns": "the metric as an integer",
            "errors": "a metric of the window manager of Windows is refused with \"SysGet(n) is a metric of the window manager of Windows, and this desktop does not have one.\"; a compositor that published no monitor answers \"the compositor published no monitor, so the metric n of SysGet is not known.\"; a number that is not a metric at all is zero.",
            "example": 'w := SysGet(0)',
        },
        "zh": {
            "summary": "给出系统的一个度量；编号取自 GetSystemMetrics，只有与屏幕有关的那些被承载。",
            "returns": "该度量（整数）",
            "example": 'w := SysGet(0)',
        },
        "pt-BR": {
            "summary": "Devolve uma métrica do sistema; os números são os de GetSystemMetrics, e só as da tela são carregadas.",
            "returns": "a métrica, como inteiro",
            "example": 'w := SysGet(0)',
        },
    },
    "EnvGet": {
        "en": {
            "summary": "Answers the value of an environment variable of the runner; the environment is the one of the process the script runs in.",
            "params": "VariableName",
            "returns": "the value of the variable, or the empty text when the environment does not have it",
            "errors": "nothing; a name that is not there is the empty text and not a failure.",
            "example": 'home := EnvGet("HOME")',
        },
        "zh": {
            "summary": "给出运行器环境变量的值；环境就是脚本所在进程的环境。",
            "returns": "变量的值，环境里没有时是空文本",
            "example": 'home := EnvGet("HOME")',
        },
        "pt-BR": {
            "summary": "Devolve o valor de uma variável de ambiente do runner; o ambiente é o do processo em que o script roda.",
            "returns": "o valor da variável, ou o texto vazio quando o ambiente não a tem",
            "example": 'home := EnvGet("HOME")',
        },
    },
    "EnvSet": {
        "en": {
            "summary": "Sets an environment variable of the runner and answers the value; the script runs on one thread, so nothing else reads the environment while the change happens.",
            "params": "VariableName, Value",
            "returns": "the value that was set, as text",
            "errors": "an empty name and a name that carries an equal sign are refused with the reason of the machine.",
            "example": 'EnvSet "MY_VAR", "value"',
        },
        "zh": {
            "summary": "设置运行器的一个环境变量并返回该值；脚本只有一个线程，改动时没有别的代码在读环境。",
            "returns": "被设置的值（文本）",
            "example": 'EnvSet "MY_VAR", "value"',
        },
        "pt-BR": {
            "summary": "Define uma variável de ambiente do runner e devolve o valor; o script roda em uma thread só, e nada mais lê o ambiente durante a mudança.",
            "returns": "o valor que foi definido, como texto",
            "example": 'EnvSet "MY_VAR", "value"',
        },
    },
    "RegRead": {
        "en": {
            "summary": "Answers the value of the registry; this machine has no registry, and the tree carries the API over a store of its own -- the hives are folders and the keys are files, under the config folder of the user.",
            "params": "KeyName, ValueName, [Default]",
            "returns": "the value of the value, or the default when the value is not there",
            "errors": "\"Parameter #1 of RegRead must not be omitted in this case.\" when the key is left out; \"Parameter #1 of RegRead is invalid.\" when the name does not begin with a hive of the ten; \"(2) The system cannot find the file specified.\" when the key or the value is not there and no default is given; a value with no name is \"(161) The specified path is invalid.\"",
            "example": 'v := RegRead("HKCU\\Software\\MyApp", "Setting", "default")',
        },
        "zh": {
            "summary": "给出注册表中的值；这台机器没有注册表，树把 API 移植到自己的存储上——蜂巢是文件夹，键是文件，放在用户的配置目录下。",
            "returns": "该值的内容；值不存在时给出默认值",
            "example": 'v := RegRead("HKCU\\Software\\MyApp", "Setting", "default")',
        },
        "pt-BR": {
            "summary": "Devolve o valor do registro; esta máquina não tem registro, e a árvore carrega a API sobre um store próprio: as hives são pastas e as chaves são arquivos, na pasta de configuração do usuário.",
            "returns": "o valor, ou o padrão quando o valor não está lá",
            "example": 'v := RegRead("HKCU\\Software\\MyApp", "Setting", "default")',
        },
    },
    "RegWrite": {
        "en": {
            "summary": "Writes a value into the registry of the store of the runtime; the key is made when it is not there, with its ancestors.",
            "params": "Value, ValueType, KeyName, ValueName",
            "returns": "the empty text",
            "errors": "\"Parameter #3 of RegWrite must not be omitted in this case.\" when the key is left out; \"Parameter #2 of RegWrite is invalid.\" when the kind is not one of the five of the dialect; \"(161) The specified path is invalid.\" when the name of the value is left out; a key without a hive is an invalid parameter.",
            "example": 'RegWrite "value", "REG_SZ", "HKCU\\Software\\MyApp", "Setting"',
        },
        "zh": {
            "summary": "把一个值写进运行器自己的注册表存储；键不存在时会被连同它的祖先一起建立。",
            "returns": "空文本",
            "example": 'RegWrite "value", "REG_SZ", "HKCU\\Software\\MyApp", "Setting"',
        },
        "pt-BR": {
            "summary": "Escreve um valor no registro do store da runtime; a chave é criada quando não está lá, com os seus ancestrais.",
            "returns": "o texto vazio",
            "example": 'RegWrite "value", "REG_SZ", "HKCU\\Software\\MyApp", "Setting"',
        },
    },
    "RegDelete": {
        "en": {
            "summary": "Removes a value of the registry of the store of the runtime; the key itself stays.",
            "params": "KeyName, ValueName",
            "returns": "the empty text",
            "errors": "\"Parameter #1 of RegDelete must not be omitted in this case.\" when the key is left out; a key without a hive of the ten is an invalid parameter; \"(2) The system cannot find the file specified.\" when the key is not there.",
            "example": 'RegDelete "HKCU\\Software\\MyApp", "Setting"',
        },
        "zh": {
            "summary": "从运行器自己的注册表存储里删掉一个值；键本身保留。",
            "returns": "空文本",
            "example": 'RegDelete "HKCU\\Software\\MyApp", "Setting"',
        },
        "pt-BR": {
            "summary": "Remove um valor do registro do store da runtime; a chave em si fica.",
            "returns": "o texto vazio",
            "example": 'RegDelete "HKCU\\Software\\MyApp", "Setting"',
        },
    },
    "DllCall": {
        "en": {
            "summary": "Calls a function of a library of the machine; the marshalling is pure and in the language crate, the call is the capability of the desktop, and the box answers it with a trampoline.",
            "params": "Target, Type1, Param1, [Type2, Param2, ...], [ReturnType]",
            "returns": "the value the function answers",
            "errors": "\"Failed to load DLL.\", \"Call to nonexistent function.\", \"Invalid arg type.\" and \"Invalid return type.\" are the reasons of v2; a library of Windows on a Linux box is called through the raven, and a box without one says \"this box has no raven: the libraries of Windows cannot be called.\"",
            "example": 'pid := DllCall("libc.so.6\\getpid", "Int")',
        },
        "zh": {
            "summary": "调用机器上一个库里的函数；参数编组是纯语言的部分，调用属于桌面能力，盒子用跳板来回答。",
            "returns": "函数返回的值",
            "example": 'pid := DllCall("libc.so.6\\getpid", "Int")',
        },
        "pt-BR": {
            "summary": "Chama uma função de uma biblioteca da máquina; a conversão dos argumentos é pura e está na linguagem, a chamada é a capacidade do desktop, e a caixa responde com uma trampoline.",
            "returns": "o valor que a função devolve",
            "example": 'pid := DllCall("libc.so.6\\getpid", "Int")',
        },
    },
    "NumGet": {
        "en": {
            "summary": "Reads a number out of the bytes of a buffer; the source of a number is a buffer, and the memory of an address is not read here.",
            "params": "Buffer, [Offset], Type",
            "returns": "the number the bytes hold",
            "errors": "\"this runtime does not read the memory of an address: the source of a number is a buffer.\" when the source is a number; a type that is not one of the eleven is refused with \"the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr.\"; bytes that do not fit are \"the bytes do not fit in the buffer.\"",
            "example": 'value := NumGet(buf, 0, "Int")',
        },
        "zh": {
            "summary": "从缓冲区的字节里读出一个数；数的来源是缓冲区，这里不读取某个地址的内存。",
            "returns": "这些字节所持有的数",
            "example": 'value := NumGet(buf, 0, "Int")',
        },
        "pt-BR": {
            "summary": "Lê um número dos bytes de um buffer; a fonte de um número é um buffer, e a memória de um endereço não é lida aqui.",
            "returns": "o número que os bytes guardam",
            "example": 'value := NumGet(buf, 0, "Int")',
        },
    },
    "NumPut": {
        "en": {
            "summary": "Writes numbers into the bytes of a buffer; the types and the numbers come in pairs and the target comes after them, as v2 writes it.",
            "params": "Type1, Number1, [Type2, Number2, ...], Target, [Offset]",
            "returns": "the address to the right of the last number that was written",
            "errors": "\"this runtime does not write to the memory of an address: the target of a number is a buffer.\" when the target is a number; a number of types that is not even is \"the types and the numbers of a write come in pairs.\"; anything after the target and the offset is \"a write takes the target and the offset after the pairs.\"",
            "example": 'NumPut("Int", 42, buf, 0)',
        },
        "zh": {
            "summary": "把若干数写进缓冲区的字节；类型和数成对出现，目标在它们之后，和 v2 的写法一样。",
            "returns": "写下去的最后一个数右边的地址",
            "example": 'NumPut("Int", 42, buf, 0)',
        },
        "pt-BR": {
            "summary": "Escreve números nos bytes de um buffer; os tipos e os números vêm em pares e o alvo vem depois deles, como o v2 escreve.",
            "returns": "o endereço à direita do último número escrito",
            "example": 'NumPut("Int", 42, buf, 0)',
        },
    },
    "Clipboard": {
        "en": {
            "summary": "The variable of the clipboard; the v2 spells it A_Clipboard and the v1 spells it Clipboard, and both are the same text. It is read with wl-paste on a Wayland session and with xclip or xsel on an X11 one, and written with wl-copy or xclip.",
            "params": "--",
            "returns": "the text on the clipboard",
            "errors": "an empty clipboard is a value and not a failure; an X11 session with neither xclip nor xsel answers \"the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine\"; a write calls the function registered for OnClipboardChange with 1, because the change the runtime can see is its own.",
            "example": 'text := A_Clipboard',
        },
        "zh": {
            "summary": "剪贴板的变量；v2 写作 A_Clipboard，v1 写作 Clipboard，两者指的是同一份文本。Wayland 会话用 wl-paste 读，X11 会话用 xclip 或 xsel 读，用 wl-copy 或 xclip 写。",
            "returns": "剪贴板上的文本",
            "example": 'text := A_Clipboard',
        },
        "pt-BR": {
            "summary": "A variável da área de transferência; o v2 a escreve A_Clipboard e o v1 a escreve Clipboard, e as duas são o mesmo texto. Lê-se com wl-paste numa sessão Wayland e com xclip ou xsel numa sessão X11, e escreve-se com wl-copy ou xclip.",
            "returns": "o texto da área de transferência",
            "example": 'text := A_Clipboard',
        },
    },
    "SoundBeep": {
        "en": {
            "summary": "Plays a tone; the box has no speaker of its own, so the tone is written as a wave file under /tmp, played with paplay, and removed again.",
            "params": "[Frequency, Duration]",
            "returns": "1",
            "errors": "a frequency outside the range is refused with \"the frequency of a tone is between 37 and 32767.\"; a host without the mixer answers \"the mixer refused: ... pactl: not found\" and \"the mixer could not play ...\".",
            "example": 'SoundBeep 1000, 200',
        },
        "zh": {
            "summary": "播放一个音调；盒子自己没有扬声器，所以音调会写成 /tmp 下的波形文件，用 paplay 播放，然后删掉。",
            "returns": "1",
            "example": 'SoundBeep 1000, 200',
        },
        "pt-BR": {
            "summary": "Toca um tom; a caixa não tem alto-falante próprio, então o tom vira um arquivo wave em /tmp, é tocado com paplay e depois removido.",
            "returns": "1",
            "example": 'SoundBeep 1000, 200',
        },
    },
    "SoundPlay": {
        "en": {
            "summary": "Plays a sound file with the mixer of the session; the volume, the speed and the pitch of v2 are not carried.",
            "params": "File, [Options]",
            "returns": "1",
            "errors": "the volume, the speed and the pitch are refused with \"the volume, the speed and the pitch of a sound are not carried yet.\"; an empty path is \"the sound file is missing.\"; a host without the mixer answers the reason of the player.",
            "example": 'SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"',
        },
        "zh": {
            "summary": "用会话的混音器播放一个声音文件；v2 的音量、速度和音高没有被承载。",
            "returns": "1",
            "example": 'SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"',
        },
        "pt-BR": {
            "summary": "Toca um arquivo de som com o mixer da sessão; o volume, a velocidade e o tom do v2 não são carregados.",
            "returns": "1",
            "example": 'SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"',
        },
    },
    "DateAdd": {
        "en": {
            "summary": "Answers a moment plus an amount of time; the date comes first and the unit comes last, and the unit is read by its first letter.",
            "params": "DateTime, Number, Unit",
            "returns": "the new date, as the text YYYYMMDDHH24MISS",
            "errors": "\"Parameter #3 of DateAdd is invalid.\" when the unit is not one of d, h, m and s; \"Parameter #1 of DateAdd is invalid.\" when the date is not one that v2 can read; \"Parameter #2 of DateAdd requires a Number, but received a String.\"",
            "example": 'next := DateAdd(20240101, 1, "d")',
        },
        "zh": {
            "summary": "给出一个时刻加上一段时间；日期在前、单位在后，单位只读第一个字母。",
            "returns": "新的日期，文本形式 YYYYMMDDHH24MISS",
            "example": 'next := DateAdd(20240101, 1, "d")',
        },
        "pt-BR": {
            "summary": "Devolve um momento mais um tempo; a data vem primeiro e a unidade por último, e a unidade é lida pela primeira letra.",
            "returns": "a nova data, como o texto YYYYMMDDHH24MISS",
            "example": 'next := DateAdd(20240101, 1, "d")',
        },
    },
    "DateDiff": {
        "en": {
            "summary": "Answers the difference between two moments; the answer is the first date minus the second, truncated towards zero.",
            "params": "DateTime1, DateTime2, Unit",
            "returns": "the difference, as an integer, in the unit that was asked for",
            "errors": "\"Parameter #3 of DateDiff is invalid.\" when the unit is not one of d, h, m and s; a date that v2 cannot read is an invalid parameter.",
            "example": 'days := DateDiff(20240110, 20240101, "d")',
        },
        "zh": {
            "summary": "给出两个时刻之间的差；答案是第一个日期减第二个日期，向零截断。",
            "returns": "差值，整数，单位是所要求的那个",
            "example": 'days := DateDiff(20240110, 20240101, "d")',
        },
        "pt-BR": {
            "summary": "Devolve a diferença entre dois momentos; a resposta é a primeira data menos a segunda, truncada para zero.",
            "returns": "a diferença, como inteiro, na unidade pedida",
            "example": 'days := DateDiff(20240110, 20240101, "d")',
        },
    },
}
