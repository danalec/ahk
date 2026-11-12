"""The behavior of the names of the first chunk.

The names are the control flow of a script and the few commands that
drive the run: the box that shows a message, the pause, the end of the
script, the timer, and the dialogs.

What is written here is what THIS tree does, not what the reference
dialect does. Where the tree refuses a name, the refusal is the entry.
"""

BEHAVIOR = {
    "MsgBox": {
        "en": {
            "summary": "Shows a message. The tree has no window of its own, so the text goes into the output of the report instead of onto the screen, and the call answers the same text.",
            "params": "Message, [Title], [Options]",
            "returns": "the text of the message",
            "errors": "more than one parameter is refused with \"the options, the title and the variable that takes the answer are not carried yet.\"",
            "example": 'MsgBox "hello"',
        },
        "zh": {
            "summary": "显示一条消息。这棵树没有自己的窗口，因此文本进入报告的输出，而不是显示在屏幕上；调用返回同样的文本。",
            "returns": "消息的文本",
            "example": 'MsgBox "hello"',
        },
        "pt-BR": {
            "summary": "Mostra uma mensagem. A árvore não tem janela própria, então o texto vai para a saída do relatório em vez de aparecer na tela, e a chamada devolve o mesmo texto.",
            "returns": "o texto da mensagem",
            "example": 'MsgBox "hello"',
        },
    },
    "Sleep": {
        "en": {
            "summary": "Pauses the script for a number of milliseconds.",
            "params": "Milliseconds",
            "returns": "the number of milliseconds waited",
            "errors": "none; a wait longer than 60000 milliseconds is cut at that number.",
            "example": "Sleep 500",
        },
        "zh": {
            "summary": "让脚本暂停指定的毫秒数。",
            "returns": "等待的毫秒数",
            "example": "Sleep 500",
        },
        "pt-BR": {
            "summary": "Pausa o script por um número de milissegundos.",
            "returns": "o número de milissegundos esperado",
            "example": "Sleep 500",
        },
    },
    "Exit": {
        "en": {
            "summary": "Stops the script. The code is kept, and the script stops at the end of the statement it was asked in.",
            "params": "[Code]",
            "returns": "the code",
            "errors": "none.",
            "example": "Exit 0",
        },
        "zh": {
            "summary": "停止脚本。退出码被保留，脚本在调用它的那条语句结束时停止。",
            "returns": "退出码",
            "example": "Exit 0",
        },
        "pt-BR": {
            "summary": "Para o script. O código é guardado, e o script para no fim da instrução onde foi chamado.",
            "returns": "o código",
            "example": "Exit 0",
        },
    },
    "ExitApp": {
        "en": {
            "summary": "Ends the script with a code. The code goes into the exit_code of the report.",
            "params": "[Code]",
            "returns": "the code",
            "errors": "none.",
            "example": "ExitApp 3",
        },
        "zh": {
            "summary": "以一个退出码结束脚本。该退出码进入报告的 exit_code。",
            "returns": "退出码",
            "example": "ExitApp 3",
        },
        "pt-BR": {
            "summary": "Encerra o script com um código. O código vai para o exit_code do relatório.",
            "returns": "o código",
            "example": "ExitApp 3",
        },
    },
    "Run": {
        "en": {
            "summary": "Runs a command and waits for it. What the command wrote on its output goes into the output of the report.",
            "params": "Command, [WorkingDir]",
            "returns": "the exit code of the command",
            "errors": "more than two parameters are refused with \"the options of the window and the pid of the process are not carried yet.\"",
            "example": 'Run "ls"',
        },
        "zh": {
            "summary": "运行一条命令并等待它结束。命令写到输出上的内容进入报告的输出。",
            "returns": "命令的退出码",
            "example": 'Run "ls"',
        },
        "pt-BR": {
            "summary": "Executa um comando e espera por ele. O que o comando escreveu na saída vai para a saída do relatório.",
            "returns": "o código de saída do comando",
            "example": 'Run "ls"',
        },
    },
    "RunWait": {
        "en": {
            "summary": "Runs a command and waits for it, the same way Run does. The name is the function form of the same handler.",
            "params": "Command, [WorkingDir]",
            "returns": "the exit code of the command",
            "errors": "more than two parameters are refused with \"the options of the window and the pid of the process are not carried yet.\"",
            "example": 'RunWait "ls"',
        },
        "zh": {
            "summary": "运行一条命令并等待它结束，与 Run 相同。这个名字是同一个处理器的函数形式。",
            "returns": "命令的退出码",
            "example": 'RunWait "ls"',
        },
        "pt-BR": {
            "summary": "Executa um comando e espera por ele, do mesmo jeito que Run. O nome é a forma de função do mesmo tratador.",
            "returns": "o código de saída do comando",
            "example": 'RunWait "ls"',
        },
    },
    "Reload": {
        "en": {
            "summary": "The tree refuses this name: this runtime runs one script once, and it cannot run itself again.",
            "params": "none",
            "returns": "nothing; the call fails",
            "errors": "the refusal is \"this runtime runs one script once, and it cannot run itself again.\"",
            "example": "Reload",
        },
        "zh": {
            "summary": "这棵树拒绝这个名字：这个运行时只运行一次脚本，无法再次运行自己。",
            "returns": "无返回值；调用失败",
            "example": "Reload",
        },
        "pt-BR": {
            "summary": "A árvore recusa este nome: este runtime executa o script uma vez só, e não pode se executar de novo.",
            "returns": "nada; a chamada falha",
            "example": "Reload",
        },
    },
    "Suspend": {
        "en": {
            "summary": "Turns the hotkeys of the script off or on. With no parameter it turns them the other way. The state is visible in A_IsSuspended and in the enabled flag of the hotkey table.",
            "params": "[OnOff]",
            "returns": "the empty text",
            "errors": "a parameter that is not 0, 1 or -1 is an invalid parameter.",
            "example": "Suspend true",
        },
        "zh": {
            "summary": "关闭或打开脚本的热键。不带参数时切换状态。状态可在 A_IsSuspended 和热键表的启用标志中看到。",
            "returns": "空文本",
            "example": "Suspend true",
        },
        "pt-BR": {
            "summary": "Desliga ou liga as hotkeys do script. Sem parâmetro, inverte o estado. O estado aparece em A_IsSuspended e na flag habilitada da tabela de hotkeys.",
            "returns": "texto vazio",
            "example": "Suspend true",
        },
    },
    "Pause": {
        "en": {
            "summary": "Sets the paused state of the script. The tree keeps a state instead of waiting for a key, which would hang a script that has no keyboard. The state is visible in A_IsPaused.",
            "params": "[OnOff]",
            "returns": "the empty text",
            "errors": "a parameter that is not 0, 1 or -1 is an invalid parameter.",
            "example": "Pause true",
        },
        "zh": {
            "summary": "设置脚本的暂停状态。这棵树只记录状态，而不是等待按键——没有键盘的脚本会因此挂住。状态可在 A_IsPaused 中看到。",
            "returns": "空文本",
            "example": "Pause true",
        },
        "pt-BR": {
            "summary": "Define o estado de pausa do script. A árvore guarda um estado em vez de esperar uma tecla, o que travaria um script sem teclado. O estado aparece em A_IsPaused.",
            "returns": "texto vazio",
            "example": "Pause true",
        },
    },
    "Persistent": {
        "en": {
            "summary": "Answers 0 and changes nothing. A script of this runtime runs from its first line to its last and has no message loop to stay alive for, so the setting has no meaning here.",
            "params": "[OnOff]",
            "returns": "the integer 0",
            "errors": "none.",
            "example": "Persistent",
        },
        "zh": {
            "summary": "返回 0，并且不改变任何东西。这个运行时的脚本从第一行跑到最后一行，没有消息循环可以让它存活，因此这个设置在这里没有意义。",
            "returns": "整数 0",
            "example": "Persistent",
        },
        "pt-BR": {
            "summary": "Devolve 0 e não muda nada. Um script deste runtime roda da primeira à última linha e não tem laço de mensagens para se manter vivo, então a opção não tem significado aqui.",
            "returns": "o inteiro 0",
            "example": "Persistent",
        },
    },
    "OnExit": {
        "en": {
            "summary": "Registers the function that runs when the script ends, whatever ended it: the last statement, a code the script asked to stop with, or a failure that stopped it.",
            "params": "Callback, [Order]",
            "returns": "the integer 1",
            "errors": "a callback that is not a function object, or a prototype that does not take two parameters, is refused with \"Invalid callback function.\".",
            "example": "OnExit((reason, code) => 0)",
        },
        "zh": {
            "summary": "注册脚本结束时运行的函数，无论结束的原因是什么：最后一条语句、脚本请求停止的退出码，或让脚本停下的失败。",
            "returns": "整数 1",
            "example": "OnExit((reason, code) => 0)",
        },
        "pt-BR": {
            "summary": "Registra a função que roda quando o script termina, qualquer que seja o motivo: a última instrução, um código pedido pelo script, ou uma falha que o parou.",
            "returns": "o inteiro 1",
            "example": "OnExit((reason, code) => 0)",
        },
    },
    "OnError": {
        "en": {
            "summary": "Registers the function that handles a failure the script did not catch.",
            "params": "Callback, [Order]",
            "returns": "the integer 1",
            "errors": "a callback that is not a function object, or a prototype that does not take two parameters, is refused with \"Invalid callback function.\".",
            "example": "OnError((err, mode) => 0)",
        },
        "zh": {
            "summary": "注册处理未被捕获的失败的函数。",
            "returns": "整数 1",
            "example": "OnError((err, mode) => 0)",
        },
        "pt-BR": {
            "summary": "Registra a função que trata uma falha que o script não capturou.",
            "returns": "o inteiro 1",
            "example": "OnError((err, mode) => 0)",
        },
    },
    "Throw": {
        "en": {
            "summary": "A statement that raises a value. The value is usually an error object, and it may be any value.",
            "params": "Value",
            "returns": "nothing; the statement leaves the flow of the script",
            "errors": "none; the value is caught by the catch that names its class.",
            "example": 'throw ValueError("boom")',
        },
        "zh": {
            "summary": "抛出（抛出）一个值的语句。这个值通常是错误对象，也可以是任何值。",
            "returns": "无返回值；该语句改变脚本的流程",
            "example": 'throw ValueError("boom")',
        },
        "pt-BR": {
            "summary": "Uma instrução que levanta um valor. O valor costuma ser um objeto de erro, e pode ser qualquer valor.",
            "returns": "nada; a instrução muda o fluxo do script",
            "example": 'throw ValueError("boom")',
        },
    },
    "Try": {
        "en": {
            "summary": "A statement that runs a body and lets a catch handle a failure of it.",
            "params": "a statement or a block",
            "returns": "nothing",
            "errors": "none; a failure of the body goes to the catch that follows.",
            "example": "try {\n    x := 1 / 0\n}",
        },
        "zh": {
            "summary": "执行一段代码的语句，失败时交给后面的 catch 处理。",
            "returns": "无返回值",
            "example": "try {\n    x := 1 / 0\n}",
        },
        "pt-BR": {
            "summary": "Uma instrução que executa um corpo e deixa um catch tratar uma falha dele.",
            "returns": "nada",
            "example": "try {\n    x := 1 / 0\n}",
        },
    },
    "Catch": {
        "en": {
            "summary": "The handler of a failure of a try. It may name a class and a variable -- catch Error as e -- or only one of them, and it runs when the failure is of that class or of a class derived from it.",
            "params": "[Class] [as Name]",
            "returns": "nothing",
            "errors": "a bare `catch e`, with a name and no `as`, is refused with \"the class of a catch is not a class of this runtime: e\".",
            "example": "catch as e {\n    MsgBox e.Message\n}",
        },
        "zh": {
            "summary": "处理 try 中失败的分支。它可以写类名和变量名（catch Error as e），也可以只写其中一个；当失败属于该类或它的派生类时运行。",
            "returns": "无返回值",
            "example": "catch as e {\n    MsgBox e.Message\n}",
        },
        "pt-BR": {
            "summary": "O tratador de uma falha de um try. Pode nomear uma classe e uma variável -- catch Error as e -- ou só uma delas, e roda quando a falha é dessa classe ou de uma classe derivada dela.",
            "returns": "nada",
            "example": "catch as e {\n    MsgBox e.Message\n}",
        },
    },
    "Finally": {
        "en": {
            "summary": "The block that runs after a try, whether the body succeeded or failed.",
            "params": "a statement or a block",
            "returns": "nothing",
            "errors": "none.",
            "example": "try {\n    MsgBox \"body\"\n} finally {\n    MsgBox \"always\"\n}",
        },
        "zh": {
            "summary": "在 try 之后执行的代码块，无论其中的代码成功还是失败都会运行。",
            "returns": "无返回值",
            "example": "try {\n    MsgBox \"body\"\n} finally {\n    MsgBox \"always\"\n}",
        },
        "pt-BR": {
            "summary": "O bloco que roda depois de um try, tanto se o corpo teve sucesso quanto se falhou.",
            "returns": "nada",
            "example": "try {\n    MsgBox \"body\"\n} finally {\n    MsgBox \"always\"\n}",
        },
    },
    "If": {
        "en": {
            "summary": "A statement that runs a body when a condition is true. The condition is an expression; the parentheses may be left out.",
            "params": "Condition, [Body]",
            "returns": "nothing",
            "errors": "a failure of the condition is reported in v2; in v1 it is not, and the body does not run.",
            "example": "if (1 = 1)\n    MsgBox \"yes\"",
        },
        "zh": {
            "summary": "条件为真时执行一段代码的语句。条件是一个表达式，括号可以省略。",
            "returns": "无返回值",
            "example": "if (1 = 1)\n    MsgBox \"yes\"",
        },
        "pt-BR": {
            "summary": "Uma instrução que executa um corpo quando a condição é verdadeira. A condição é uma expressão; os parênteses podem ser omitidos.",
            "returns": "nada",
            "example": "if (1 = 1)\n    MsgBox \"yes\"",
        },
    },
    "Else": {
        "en": {
            "summary": "The other side of an if. Else if chains the conditions, and the first one that matches runs.",
            "params": "a statement or a block",
            "returns": "nothing",
            "errors": "none.",
            "example": "if (0)\n    MsgBox \"no\"\nelse\n    MsgBox \"yes\"",
        },
        "zh": {
            "summary": "if 的另一分支。else if 可以串联多个条件，第一个匹配的条件会执行。",
            "returns": "无返回值",
            "example": "if (0)\n    MsgBox \"no\"\nelse\n    MsgBox \"yes\"",
        },
        "pt-BR": {
            "summary": "O outro lado de um if. Else if encadeia condições, e a primeira que casar roda.",
            "returns": "nada",
            "example": "if (0)\n    MsgBox \"no\"\nelse\n    MsgBox \"yes\"",
        },
    },
    "While": {
        "en": {
            "summary": "A loop that runs its body while a condition is true.",
            "params": "Condition, [Body]",
            "returns": "nothing",
            "errors": "the loop trips a limit of the runtime and reports it as an error on the line of the statement.",
            "example": "i := 0\nwhile (i < 3) {\n    i += 1\n}",
        },
        "zh": {
            "summary": "条件为真时反复执行代码的循环。",
            "returns": "无返回值",
            "example": "i := 0\nwhile (i < 3) {\n    i += 1\n}",
        },
        "pt-BR": {
            "summary": "Um laço que executa seu corpo enquanto a condição for verdadeira.",
            "returns": "nada",
            "example": "i := 0\nwhile (i < 3) {\n    i += 1\n}",
        },
    },
    "Loop": {
        "en": {
            "summary": "A loop with a count, a loop without one that goes on until a break, and a loop over the files of a folder.",
            "params": "[Count], [Body]",
            "returns": "nothing; A_Index counts the turns from one",
            "errors": "a loop that never ends trips a limit of the runtime: 20000 statements, 1000 turns of a while body, 1000 nested calls.",
            "example": "Loop 3\n    MsgBox A_Index",
        },
        "zh": {
            "summary": "带次数的循环、不带次数直到 break 的循环，以及遍历文件夹中文件的循环。",
            "returns": "无返回值；A_Index 从 1 开始计数",
            "example": "Loop 3\n    MsgBox A_Index",
        },
        "pt-BR": {
            "summary": "Um laço com contagem, um laço sem contagem que roda até um break, e um laço sobre os arquivos de uma pasta.",
            "returns": "nada; A_Index conta as voltas a partir de um",
            "example": "Loop 3\n    MsgBox A_Index",
        },
    },
    "For": {
        "en": {
            "summary": "A loop that walks the values of an enumerable. With two variables it walks the key and the value together.",
            "params": "Value [, Key] in Source",
            "returns": "nothing; A_Index counts the turns too",
            "errors": "a source that is not enumerable is refused with \"Value not enumerable.\" on the line of the loop.",
            "example": "for v in [10, 20]\n    MsgBox v",
        },
        "zh": {
            "summary": "遍历可枚举对象的各个值的循环。写两个变量时会同时遍历键和值。",
            "returns": "无返回值；A_Index 也会计数",
            "example": "for v in [10, 20]\n    MsgBox v",
        },
        "pt-BR": {
            "summary": "Um laço que percorre os valores de um enumerável. Com duas variáveis, percorre a chave e o valor juntos.",
            "returns": "nada; A_Index também conta as voltas",
            "example": "for v in [10, 20]\n    MsgBox v",
        },
    },
    "Break": {
        "en": {
            "summary": "Leaves the loop it is in.",
            "params": "none",
            "returns": "nothing",
            "errors": "none.",
            "example": "Loop 5 {\n    break\n}",
        },
        "zh": {
            "summary": "跳出所在的循环。",
            "returns": "无返回值",
            "example": "Loop 5 {\n    break\n}",
        },
        "pt-BR": {
            "summary": "Sai do laço em que está.",
            "returns": "nada",
            "example": "Loop 5 {\n    break\n}",
        },
    },
    "Continue": {
        "en": {
            "summary": "Goes to the next turn of the loop it is in.",
            "params": "none",
            "returns": "nothing",
            "errors": "none.",
            "example": "Loop 3 {\n    continue\n}",
        },
        "zh": {
            "summary": "进入所在循环的下一轮。",
            "returns": "无返回值",
            "example": "Loop 3 {\n    continue\n}",
        },
        "pt-BR": {
            "summary": "Vai para a próxima volta do laço em que está.",
            "returns": "nada",
            "example": "Loop 3 {\n    continue\n}",
        },
    },
    "Return": {
        "en": {
            "summary": "Ends the call and answers a value. Outside a function it ends the script, and so does a return with no value at the top level.",
            "params": "[Value]",
            "returns": "the value, when the call has one",
            "errors": "none.",
            "example": "Twice(x) {\n    return x * 2\n}",
        },
        "zh": {
            "summary": "结束当前调用并返回一个值。在函数之外它会结束脚本，顶层不带值的 return 也是如此。",
            "returns": "值（当调用有返回值时）",
            "example": "Twice(x) {\n    return x * 2\n}",
        },
        "pt-BR": {
            "summary": "Encerra a chamada e devolve um valor. Fora de uma função, encerra o script, e o mesmo vale para um return sem valor no nível de topo.",
            "returns": "o valor, quando a chamada tem um",
            "example": "Twice(x) {\n    return x * 2\n}",
        },
    },
    "Switch": {
        "en": {
            "summary": "Compares the value of the subject with each case and runs the body of the first that matches. Default takes the rest. A case may list several values with commas, and a case with no body shares the body of the next one.",
            "params": "Subject, {Case ... Default ...}",
            "returns": "nothing",
            "errors": "none; the cases of v2 are compared without regard to the case.",
            "example": "switch 5 {\n    case 1:\n        MsgBox \"one\"\n    default:\n        MsgBox \"other\"\n}",
        },
        "zh": {
            "summary": "把主题的值与各个 case 比较，运行第一个匹配的分支。default 负责其余情况。case 可以用逗号列出多个值，没有代码的 case 与下一个共用代码。",
            "returns": "无返回值",
            "example": "switch 5 {\n    case 1:\n        MsgBox \"one\"\n    default:\n        MsgBox \"other\"\n}",
        },
        "pt-BR": {
            "summary": "Compara o valor do sujeito com cada case e executa o corpo do primeiro que casar. Default trata o resto. Um case pode listar vários valores separados por vírgulas, e um case sem corpo divide o corpo com o seguinte.",
            "returns": "nada",
            "example": "switch 5 {\n    case 1:\n        MsgBox \"one\"\n    default:\n        MsgBox \"other\"\n}",
        },
    },
    "Goto": {
        "en": {
            "summary": "Jumps to a label, which is a name followed by a colon at the start of a line.",
            "params": "Label",
            "returns": "nothing",
            "errors": "a goto that jumps into a block that does not enclose it is refused with \"A Goto must not jump into a block that does not enclose it.\".",
            "example": "goto Done\nDone:",
        },
        "zh": {
            "summary": "跳到标签，标签是行首带冒号的名字。",
            "returns": "无返回值",
            "example": "goto Done\nDone:",
        },
        "pt-BR": {
            "summary": "Salta para um rótulo, que é um nome seguido de dois-pontos no início de uma linha.",
            "returns": "nada",
            "example": "goto Done\nDone:",
        },
    },
    "SetTimer": {
        "en": {
            "summary": "Registers a function of the script that runs between the statements. The runtime is one thread, so the timer is not a thread of its own: it runs between the statements and shares the budget of the script.",
            "params": "Name, [Period]",
            "returns": "the integer 1",
            "errors": "more than two parameters are refused with \"the priority of the timer is not carried yet.\"",
            "example": 'SetTimer("Tick", 1000)',
        },
        "zh": {
            "summary": "注册一个在语句之间运行的脚本函数。这个运行时只有一个线程，因此定时器不是独立的线程：它在语句之间运行，与脚本共用预算。",
            "returns": "整数 1",
            "example": 'SetTimer("Tick", 1000)',
        },
        "pt-BR": {
            "summary": "Registra uma função do script que roda entre as instruções. O runtime tem uma só thread, então o timer não é uma thread própria: roda entre as instruções e divide o orçamento do script.",
            "returns": "o inteiro 1",
            "example": 'SetTimer("Tick", 1000)',
        },
    },
    "SetWorkingDir": {
        "en": {
            "summary": "Sets the working directory of the script. A name that is not absolute is a folder under the working directory, the way the dialect says it.",
            "params": "Directory",
            "returns": "the integer 1",
            "errors": "an empty name is refused with \"the name of the directory is missing.\", and a folder that cannot be entered fails with the reason of the desktop.",
            "example": 'SetWorkingDir "."',
        },
        "zh": {
            "summary": "设置脚本的工作目录。不是绝对路径的名字会被视为工作目录下的子目录。",
            "returns": "整数 1",
            "example": 'SetWorkingDir "."',
        },
        "pt-BR": {
            "summary": "Define o diretório de trabalho do script. Um nome que não é absoluto é uma pasta dentro do diretório de trabalho.",
            "returns": "o inteiro 1",
            "example": 'SetWorkingDir "."',
        },
    },
    "Critical": {
        "en": {
            "summary": "The tree refuses this name: this runtime runs one script in one thread, and it cannot make it critical.",
            "params": "[OnOff]",
            "returns": "nothing; the call fails",
            "errors": "the refusal is \"this runtime runs one script in one thread, and it cannot make it critical.\"",
            "example": "Critical",
        },
        "zh": {
            "summary": "这棵树拒绝这个名字：这个运行时只用一个线程运行一个脚本，无法把它设为关键状态。",
            "returns": "无返回值；调用失败",
            "example": "Critical",
        },
        "pt-BR": {
            "summary": "A árvore recusa este nome: este runtime executa um script numa thread só, e não pode torná-lo crítico.",
            "returns": "nada; a chamada falha",
            "example": "Critical",
        },
    },
    "ToolTip": {
        "en": {
            "summary": "Shows a small window of text at a place of the screen. Called with no text it takes the window away.",
            "params": "Text, [X], [Y], [Options]",
            "returns": "the number that identifies the window, or the empty text when the window is taken away",
            "errors": "the desktop reports the failure of the call.",
            "example": 'ToolTip "hello", 10, 10',
        },
        "zh": {
            "summary": "在屏幕的某个位置显示一个小文本窗口。不带文本调用时会把窗口移除。",
            "returns": "标识该窗口的编号；移除窗口时返回空文本",
            "example": 'ToolTip "hello", 10, 10',
        },
        "pt-BR": {
            "summary": "Mostra uma pequena janela de texto num ponto da tela. Chamada sem texto, ela remove a janela.",
            "returns": "o número que identifica a janela, ou texto vazio quando a janela é removida",
            "example": 'ToolTip "hello", 10, 10',
        },
    },
    "TrayTip": {
        "en": {
            "summary": "Sends a notification to the desktop with a text and a title. The name is a Windows one -- a balloon in the notification area -- but the tree sends a desktop notification, which reaches the same user with the same words.",
            "params": "Text, [Title], [Options]",
            "returns": "the empty text",
            "errors": "the desktop reports the failure of the notification.",
            "example": 'TrayTip "done", "the script"',
        },
        "zh": {
            "summary": "向桌面发送一条带文本和标题的通知。这个名字来自 Windows（通知区里的气泡），但这棵树发送的是桌面通知，同样的用户会看到同样的内容。",
            "returns": "空文本",
            "example": 'TrayTip "done", "the script"',
        },
        "pt-BR": {
            "summary": "Manda uma notificação à área de trabalho com um texto e um título. O nome é do Windows -- um balão na área de notificação -- mas a árvore manda uma notificação de mesa, que chega ao mesmo usuário com as mesmas palavras.",
            "returns": "texto vazio",
            "example": 'TrayTip "done", "the script"',
        },
    },
    "InputBox": {
        "en": {
            "summary": "Asks the user for a text in a dialog of the desktop and answers an object with the text in its Value and the way the dialog was closed in its Result.",
            "params": "Prompt, [Title], [Options], [Default]",
            "returns": "an object with the properties Value and Result, where Result is \"OK\", \"Cancel\" or \"Timeout\"",
            "errors": "an option the dialect does not know is refused with \"Invalid option.\".",
            "example": 'answer := InputBox("your name?", "hello")\nMsgBox answer.Value',
        },
        "zh": {
            "summary": "在桌面的对话框里向用户要一段文本，返回一个对象：Value 是输入的文本，Result 是对话框被关闭的方式。",
            "returns": "带 Value 和 Result 属性的对象，Result 是 \"OK\"、\"Cancel\" 或 \"Timeout\"",
            "example": 'answer := InputBox("your name?", "hello")\nMsgBox answer.Value',
        },
        "pt-BR": {
            "summary": "Pede um texto ao usuário numa caixa da área de trabalho e devolve um objeto com o texto em Value e o modo de fechamento em Result.",
            "returns": "um objeto com as propriedades Value e Result, onde Result é \"OK\", \"Cancel\" ou \"Timeout\"",
            "example": 'answer := InputBox("your name?", "hello")\nMsgBox answer.Value',
        },
    },
}
