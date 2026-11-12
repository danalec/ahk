# -*- coding: utf-8 -*-
"""The behavior of the input, the mouse, the screen and the window names.

The chunk is the fourth one of `scripts/surface_behavior.py`: the names
that act on the machine. Every one of them goes through the desktop
backend, and the backend of this tree is a Linux desktop: a uinput
keyboard of its own, an absolute uinput pointer, `grim` for the screen,
and the compositor for the windows.

A name of this chunk is carried -- the tree answers it -- but the answer
is often a reason rather than a value, and the reason says what the
machine cannot do. The names that are shaped for Windows say so
plainly: the play method of `Send`, the blocking of the input of the
whole machine, the hooks of Windows, and the windows of another
program.
"""

BEHAVIOR = {
    "Send": {
        "en": {
            "summary": "Types text and presses keys on the machine, through the uinput keyboard of the backend.",
            "params": "Keys",
            "returns": "nothing",
            "errors": "the keyboard is a Linux uinput device, so on a host that is not Linux every key action fails; a character the US layout has no key for fails with 'the keyboard of this box has no key for'.",
            "example": 'Send "hello{Enter}"',
        },
        "zh": {
            "summary": "通过后端自建的 uinput 键盘在机器上输入文本并按下按键。",
            "returns": "无返回值",
            "example": 'Send "hello{Enter}"',
        },
        "pt-BR": {
            "summary": "Digita texto e pressiona teclas na máquina, pelo teclado uinput do backend.",
            "returns": "nada",
            "example": 'Send "hello{Enter}"',
        },
    },
    "SendText": {
        "en": {
            "summary": "Types the text as it stands: the braces of v2 are not read here, and the text goes to the keyboard one character at a time.",
            "params": "Text",
            "returns": "nothing",
            "errors": "the keyboard of the backend is a Linux uinput device, so on a host that is not Linux the text is not typed.",
            "example": 'SendText "a literal {braced} text"',
        },
        "zh": {
            "summary": "按原样输入文本：这里不解析 v2 的大括号，文本逐字符送入键盘。",
            "returns": "无返回值",
            "example": 'SendText "a literal {braced} text"',
        },
        "pt-BR": {
            "summary": "Digita o texto como ele está: as chaves de v2 não são lidas aqui, e o texto vai ao teclado um caractere por vez.",
            "returns": "nada",
            "example": 'SendText "a literal {braced} text"',
        },
    },
    "SendEvent": {
        "en": {
            "summary": "Sends the keys the way the box sends them. The event method of v2 is the method of Windows, and the tree has one keyboard, so the keys go through it the way `Send` sends them.",
            "params": "Keys",
            "returns": "nothing",
            "errors": "the keyboard of the backend is a Linux uinput device, and it fails on a host that is not Linux.",
            "example": 'SendEvent "^a"',
        },
        "zh": {
            "summary": "按后端的方式发送按键。v2 的事件方式是 Windows 的方式，而本运行时只有一个键盘，因此按键走的就是 `Send` 走的键盘。",
            "returns": "无返回值",
            "example": 'SendEvent "^a"',
        },
        "pt-BR": {
            "summary": "Envia as teclas do jeito do backend. O método de evento de v2 é o método do Windows, e a árvore tem só um teclado, então as teclas passam por ele como `Send` faz.",
            "returns": "nada",
            "example": 'SendEvent "^a"',
        },
    },
    "SendInput": {
        "en": {
            "summary": "Sends the keys the way the box sends them: the input method of v2 is the method of the keyboard of the box, so the keys go through it, the way `Send` sends them.",
            "params": "Keys",
            "returns": "nothing",
            "errors": "the keyboard of the backend is a Linux uinput device, and it fails on a host that is not Linux.",
            "example": 'SendInput "{Enter}"',
        },
        "zh": {
            "summary": "按后端的方式发送按键：v2 的输入方式就是后端键盘的方式，因此按键走的路径与 `Send` 相同。",
            "returns": "无返回值",
            "example": 'SendInput "{Enter}"',
        },
        "pt-BR": {
            "summary": "Envia as teclas do jeito do backend: o método de entrada de v2 é o método do teclado do backend, então as teclas passam por ele, como `Send` faz.",
            "returns": "nada",
            "example": 'SendInput "{Enter}"',
        },
    },
    "SendPlay": {
        "en": {
            "summary": "Refused. The play method of v2 does not send keystrokes: it posts the messages to the active window, and there is no window here to post them to.",
            "params": "Keys",
            "returns": "nothing; the statement fails",
            "errors": "the play method of v2 posts the messages to the active window, and there is no window here to post them to.",
            "example": 'SendPlay "{Enter}"',
        },
        "zh": {
            "summary": "被拒绝。v2 的 play 方式不发送按键，而是向活动窗口投递消息，而这里没有可以投递的窗口。",
            "returns": "无返回值；语句失败",
            "example": 'SendPlay "{Enter}"',
        },
        "pt-BR": {
            "summary": "Recusado. O método play de v2 não envia teclas: ele posta as mensagens na janela ativa, e aqui não há janela para receber.",
            "returns": "nada; a instrução falha",
            "example": 'SendPlay "{Enter}"',
        },
    },
    "SendMode": {
        "en": {
            "summary": "Says how the keys are sent. The keys of this runtime go through the keyboard of the box, one at a time, and that is the mode of the box; the other modes of v2 are the ones of Windows.",
            "params": "Method",
            "returns": "the number 1 for the mode of the box",
            "errors": "a mode that is not 'Input' fails with 'the keys of this runtime go through the keyboard of the box, and that is the only mode there is.'",
            "example": 'SendMode "Input"',
        },
        "zh": {
            "summary": "说明按键如何发送。本运行时的按键逐个走后端的键盘，这就是后端的方式；v2 的其他方式属于 Windows。",
            "returns": "后端的方式返回数字 1",
            "example": 'SendMode "Input"',
        },
        "pt-BR": {
            "summary": "Diz como as teclas são enviadas. As teclas desta árvore passam uma a uma pelo teclado do backend, e esse é o modo do backend; os outros modos de v2 são os do Windows.",
            "returns": "o número 1 para o modo do backend",
            "example": 'SendMode "Input"',
        },
    },
    "SendLevel": {
        "en": {
            "summary": "Carries the level of the events the script sends, and answers the level that was there before. The level tells the hotkeys which of the artificial events they are to ignore, and nothing reads it yet.",
            "params": "Level",
            "returns": "the level that was there before, a number",
            "errors": "a level outside 0 to 100 is refused as an invalid parameter.",
            "example": 'SendLevel 1',
        },
        "zh": {
            "summary": "保存脚本所发送事件的级别，并返回之前的级别。该级别用于告诉热键忽略哪些模拟事件，目前还没有东西读取它。",
            "returns": "之前的级别，一个数字",
            "example": 'SendLevel 1',
        },
        "pt-BR": {
            "summary": "Guarda o nível dos eventos que o script envia e devolve o nível anterior. O nível diz aos hotkeys quais eventos artificiais ignorar, e nada o lê ainda.",
            "returns": "o nível anterior, um número",
            "example": 'SendLevel 1',
        },
    },
    "Click": {
        "en": {
            "summary": "Clicks with the pointer of the backend, which is an absolute uinput device: it moves to the place, settles, presses and releases.",
            "params": "[X, Y] [Button] [Count]",
            "returns": "nothing",
            "errors": "the wheel, the fourth and the fifth buttons, the pressing of a button and the place relative to the pointer are refused as not carried yet; on a host without `grim` the click fails with the reason of the screen, because the pointer needs the size of the screen.",
            "example": 'Click 100, 200',
        },
        "zh": {
            "summary": "用后端的指针点击，该指针是绝对的 uinput 设备：先移动到位、停顿，再按下并松开。",
            "returns": "无返回值",
            "example": 'Click 100, 200',
        },
        "pt-BR": {
            "summary": "Clica com o ponteiro do backend, que é um dispositivo uinput absoluto: move até o lugar, assenta, pressiona e solta.",
            "returns": "nada",
            "example": 'Click 100, 200',
        },
    },
    "MouseClick": {
        "en": {
            "summary": "Clicks with the pointer of the backend. The button is the first parameter, the way the function of v2 takes it, and a button that is not there is the left one.",
            "params": "Button, X, Y [Count] [Options]",
            "returns": "nothing",
            "errors": "the wheel, the fourth and the fifth buttons are refused as not carried yet.",
            "example": 'MouseClick "Right", 100, 200',
        },
        "zh": {
            "summary": "用后端的指针点击。按钮是第一个参数，与 v2 函数的写法一致；不写按钮时是左键。",
            "returns": "无返回值",
            "example": 'MouseClick "Right", 100, 200',
        },
        "pt-BR": {
            "summary": "Clica com o ponteiro do backend. O botão é o primeiro parâmetro, como na função de v2, e um botão ausente é o esquerdo.",
            "returns": "nada",
            "example": 'MouseClick "Right", 100, 200',
        },
    },
    "MouseMove": {
        "en": {
            "summary": "Moves the pointer of the backend to a place on the screen. The box moves the pointer to the place, and not from where it is.",
            "params": "X, Y [Relative] [Speed]",
            "returns": "nothing",
            "errors": "the relative place of the move and the speed of the move are refused as not carried yet.",
            "example": 'MouseMove 400, 300',
        },
        "zh": {
            "summary": "把后端的指针移动到屏幕上的某个位置。后端是把指针移到该位置，而不是从当前位置相对移动。",
            "returns": "无返回值",
            "example": 'MouseMove 400, 300',
        },
        "pt-BR": {
            "summary": "Move o ponteiro do backend até um lugar da tela. O backend move o ponteiro até o lugar, e não a partir de onde ele está.",
            "returns": "nada",
            "example": 'MouseMove 400, 300',
        },
    },
    "MouseGetPos": {
        "en": {
            "summary": "Reads the place of the pointer this runtime drives, out of the kernel. It is the only pointer on the box whose place can be read at all: the physical mouse is a relative device.",
            "params": "[&X, &Y, &Button, &Info]",
            "returns": "nothing; the place is written into the variables of the caller",
            "errors": "when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.",
            "example": 'MouseGetPos &x, &y',
        },
        "zh": {
            "summary": "从内核读取本运行时所驱动的指针位置。它是这台机器上唯一能读出位置的指针：物理鼠标是相对设备。",
            "returns": "无返回值；位置写入调用者的变量",
            "example": 'MouseGetPos &x, &y',
        },
        "pt-BR": {
            "summary": "Lê do kernel a posição do ponteiro que esta árvore dirige. É o único ponteiro da máquina cuja posição pode ser lida: o mouse físico é um dispositivo relativo.",
            "returns": "nada; a posição é escrita nas variáveis de quem chama",
            "example": 'MouseGetPos &x, &y',
        },
    },
    "KeyWait": {
        "en": {
            "summary": "Refused. The runtime does not keep the keys that are down, so it cannot wait for one.",
            "params": "KeyName [, Options]",
            "returns": "nothing; the statement fails",
            "errors": "this runtime does not keep the keys that are down, so it cannot wait for one.",
            "example": 'KeyWait "a"',
        },
        "zh": {
            "summary": "被拒绝。本运行时不保存处于按下状态的按键，因此无法等待某个键。",
            "returns": "无返回值；语句失败",
            "example": 'KeyWait "a"',
        },
        "pt-BR": {
            "summary": "Recusado. A árvore não guarda as teclas que estão pressionadas, então não pode esperar por uma.",
            "returns": "nada; a instrução falha",
            "example": 'KeyWait "a"',
        },
    },
    "GetKeyState": {
        "en": {
            "summary": "Reads what the hotkey watcher saw: whether a key is down. The second parameter carries only the words 'T' and 'P' of v2.",
            "params": "KeyName [, Mode]",
            "returns": "one when the key is down, zero when it is not",
            "errors": "a key name the tree does not know, and a mode that is neither 'T' nor 'P', are refused as invalid parameters; on a host that is not Linux the watcher cannot answer at all.",
            "example": 'state := GetKeyState("Shift")',
        },
        "zh": {
            "summary": "读取热键监视器所看到的状态：某个键是否按下。第二个参数只接受 v2 的 'T' 和 'P'。",
            "returns": "按下时为 1，否则为 0",
            "example": 'state := GetKeyState("Shift")',
        },
        "pt-BR": {
            "summary": "Lê o que o vigia de hotkeys viu: se uma tecla está pressionada. O segundo parâmetro só aceita 'T' e 'P' de v2.",
            "returns": "1 quando a tecla está pressionada, 0 quando não está",
            "example": 'state := GetKeyState("Shift")',
        },
    },
    "GetKeyName": {
        "en": {
            "summary": "The name of the key, the way v2 gives it. The names are the ones of the key list of v2: `Esc`, `Enter`, `CtrlRight`, `Numpad0`, and the `vkXX` and `scXXX` spellings.",
            "params": "KeyName",
            "returns": "the name of the key",
            "errors": "a name the tree does not know is the empty text, the way v2 answers a name it does not know.",
            "example": 'name := GetKeyName("Enter")',
        },
        "zh": {
            "summary": "按键的名称，按 v2 的写法。名称取自 v2 的按键表：`Esc`、`Enter`、`CtrlRight`、`Numpad0`，以及 `vkXX` 和 `scXXX` 写法。",
            "returns": "按键的名称",
            "example": 'name := GetKeyName("Enter")',
        },
        "pt-BR": {
            "summary": "O nome da tecla, como v2 o dá. Os nomes são os da lista de teclas de v2: `Esc`, `Enter`, `CtrlRight`, `Numpad0`, e as grafias `vkXX` e `scXXX`.",
            "returns": "o nome da tecla",
            "example": 'name := GetKeyName("Enter")',
        },
    },
    "GetKeyVK": {
        "en": {
            "summary": "The virtual key code of Windows of the key. It is the number of the key of the keyboard of Windows, and the tree keeps it for the scripts that ask for it.",
            "params": "KeyName",
            "returns": "the virtual key code, a number",
            "errors": "a name the tree does not know is the empty text, the way v2 answers a name it does not know.",
            "example": 'vk := GetKeyVK("Enter")',
        },
        "zh": {
            "summary": "按键的 Windows 虚拟键码。它是 Windows 键盘的按键编号，本树保留它是为了那些需要它的脚本。",
            "returns": "虚拟键码，一个数字",
            "example": 'vk := GetKeyVK("Enter")',
        },
        "pt-BR": {
            "summary": "O código de tecla virtual do Windows da tecla. É o número da tecla no teclado do Windows, e a árvore o mantém para os scripts que o pedem.",
            "returns": "o código de tecla virtual, um número",
            "example": 'vk := GetKeyVK("Enter")',
        },
    },
    "GetKeySC": {
        "en": {
            "summary": "The code of the Linux input event of the key. It is the code the box writes to its own device, and not the scan code of the PC keyboard: the two numberings part company on the extended keys.",
            "params": "KeyName",
            "returns": "the input event code, a number",
            "errors": "a name the tree does not know is the empty text, the way v2 answers a name it does not know.",
            "example": 'sc := GetKeySC("Enter")',
        },
        "zh": {
            "summary": "按键的 Linux 输入事件码。它是后端写给自己的设备所用的编号，而不是 PC 键盘的扫描码：在扩展键上两套编号并不一致。",
            "returns": "输入事件码，一个数字",
            "example": 'sc := GetKeySC("Enter")',
        },
        "pt-BR": {
            "summary": "O código do evento de entrada Linux da tecla. É o código que o backend escreve no seu próprio dispositivo, e não o scan code do teclado de PC: as duas numerações divergem nas teclas estendidas.",
            "returns": "o código de evento de entrada, um número",
            "example": 'sc := GetKeySC("Enter")',
        },
    },
    "BlockInput": {
        "en": {
            "summary": "Turns the input of the whole machine off, and that is all it can do. Turning input off for the whole machine is a privilege this compositor does not give a client, and this tree will not claim it did.",
            "params": "Mode",
            "returns": "nothing for 'Off'",
            "errors": "any other mode fails with 'blocking the input of the whole machine is a privilege this compositor does not give a client'.",
            "example": 'BlockInput "Off"',
        },
        "zh": {
            "summary": "只能关闭整机的输入。整机屏蔽输入是本合成器不授予客户端的权限，本树不会谎称做到了。",
            "returns": "'Off' 时无返回值",
            "example": 'BlockInput "Off"',
        },
        "pt-BR": {
            "summary": "Só consegue desligar a entrada da máquina inteira. Bloquear a entrada da máquina toda é um privilégio que este compositor não dá a um cliente, e a árvore não vai fingir que deu.",
            "returns": "nada para 'Off'",
            "example": 'BlockInput "Off"',
        },
    },
    "CoordMode": {
        "en": {
            "summary": "The way the coordinates are read. The five targets of v2 are the mouse, the pixels, the caret, the menu and the tooltip; the tree has no windows, so the modes of the caret, the menu and the tooltip are kept all the same and nothing uses them.",
            "params": "Target [, Relative]",
            "returns": "the mode that was there before",
            "errors": "a target or a relative that v2 does not know is refused as an invalid parameter.",
            "example": 'CoordMode "Mouse", "Screen"',
        },
        "zh": {
            "summary": "说明坐标如何解读。v2 有五个目标：鼠标、像素、插入符、菜单和提示；本树没有窗口，插入符、菜单和提示的模式照样保存，但没有人使用它们。",
            "returns": "之前的模式",
            "example": 'CoordMode "Mouse", "Screen"',
        },
        "pt-BR": {
            "summary": "Diz como as coordenadas são lidas. Os cinco alvos de v2 são o mouse, os pixels, o cursor de texto, o menu e o tooltip; a árvore não tem janelas, então os modos de cursor, menu e tooltip são guardados do mesmo jeito e nada os usa.",
            "returns": "o modo anterior",
            "example": 'CoordMode "Mouse", "Screen"',
        },
    },
    "InstallKeybdHook": {
        "en": {
            "summary": "Answers that the hook is there. The hooks of v2 are installed when a hotkey needs them, and the box watches the keyboard from the first hotkey, so the command says that the hook is already there.",
            "params": "[OnOff] [HookName]",
            "returns": "nothing",
            "errors": "none; the command never fails.",
            "example": 'InstallKeybdHook',
        },
        "zh": {
            "summary": "回答钩子已经就位。v2 的钩子在热键需要时才安装，而后端从第一个热键起就开始监视键盘，因此这条命令只是说钩子已存在。",
            "returns": "无返回值",
            "example": 'InstallKeybdHook',
        },
        "pt-BR": {
            "summary": "Responde que o hook está lá. Os hooks de v2 são instalados quando um hotkey precisa deles, e o backend vigia o teclado desde o primeiro hotkey, então o comando diz que o hook já existe.",
            "returns": "nada",
            "example": 'InstallKeybdHook',
        },
    },
    "InstallMouseHook": {
        "en": {
            "summary": "Answers that the hook is there. The mouse is not watched at all: the box watches the keyboard, and the pointer is a device that writes and is never read.",
            "params": "[OnOff] [HookName]",
            "returns": "nothing",
            "errors": "none; the command never fails.",
            "example": 'InstallMouseHook',
        },
        "zh": {
            "summary": "回答钩子已经就位。鼠标完全没有被监视：后端监视的是键盘，而指针只是一个写入、从不读取的设备。",
            "returns": "无返回值",
            "example": 'InstallMouseHook',
        },
        "pt-BR": {
            "summary": "Responde que o hook está lá. O mouse não é vigiado de jeito nenhum: o backend vigia o teclado, e o ponteiro é um dispositivo que escreve e nunca é lido.",
            "returns": "nada",
            "example": 'InstallMouseHook',
        },
    },
    "Hotkey": {
        "en": {
            "summary": "Registers a hotkey: the key it watches for and the function it runs. The runtime owns the registry and the machine owns the watcher: the script says which function a hotkey runs, and the machine says which key was pressed.",
            "params": "KeyName, Action [, Options]",
            "returns": "nothing",
            "errors": "a hotkey without an action is 'Nonexistent hotkey.'; the watcher reads the physical keyboard of the machine, so on a host that is not Linux the registration fails with 'the hotkeys of this box watch a Linux device; this is not Linux'.",
            "example": 'Hotkey "^s", SaveTheFile',
        },
        "zh": {
            "summary": "注册一个热键：监视的按键和要运行的函数。注册表归运行时，监视器归机器：脚本说明热键运行哪个函数，机器说明按下了哪个键。",
            "returns": "无返回值",
            "example": 'Hotkey "^s", SaveTheFile',
        },
        "pt-BR": {
            "summary": "Registra um hotkey: a tecla vigiada e a função que ela roda. O registro é da árvore e o vigia é da máquina: o script diz qual função o hotkey roda, e a máquina diz qual tecla foi pressionada.",
            "returns": "nada",
            "example": 'Hotkey "^s", SaveTheFile',
        },
    },
    "Hotstring": {
        "en": {
            "summary": "Registers a text that fires a function when it is typed. The text of v2 is written between two of the same character, and the character is not part of it.",
            "params": "Text, Action [, Options]",
            "returns": "nothing",
            "errors": "a text that is not a string is refused as an invalid parameter; the watcher reads the physical keyboard, and the device of the box is skipped so that a hotstring does not fire on what the script types.",
            "example": 'Hotstring "::btw::", ShowTheDefinition',
        },
        "zh": {
            "summary": "注册一段文本，输入它时触发一个函数。v2 的文本写在两个相同字符之间，这两个字符不属于文本本身。",
            "returns": "无返回值",
            "example": 'Hotstring "::btw::", ShowTheDefinition',
        },
        "pt-BR": {
            "summary": "Registra um texto que roda uma função quando é digitado. O texto de v2 é escrito entre dois caracteres iguais, e o caractere não faz parte dele.",
            "returns": "nada",
            "example": 'Hotstring "::btw::", ShowTheDefinition',
        },
    },
    "PixelGetColor": {
        "en": {
            "summary": "The colour of one pixel, read from the frame the compositor draws. The box captures a 1x1 region with `grim` and reads the one pixel.",
            "params": "X, Y [, Default]",
            "returns": "the colour, as `#rrggbb`",
            "errors": "a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.",
            "example": 'colour := PixelGetColor(10, 20)',
        },
        "zh": {
            "summary": "读取一个像素的颜色，读的是合成器绘制的画面。后端用 `grim` 截取 1x1 区域，再读出那一个像素。",
            "returns": "颜色，格式为 `#rrggbb`",
            "example": 'colour := PixelGetColor(10, 20)',
        },
        "pt-BR": {
            "summary": "A cor de um pixel, lida do quadro que o compositor desenha. O backend captura uma região 1x1 com `grim` e lê o pixel.",
            "returns": "a cor, como `#rrggbb`",
            "example": 'colour := PixelGetColor(10, 20)',
        },
    },
    "PixelSearch": {
        "en": {
            "summary": "Where a colour is on the screen. The region is captured once and walked for the first pixel of that colour. It is the check that works whatever the desktop is.",
            "params": "&X, &Y, X1, Y1, X2, Y2, Color",
            "returns": "one when the colour is there, zero when it is not",
            "errors": "the two places are written into the variables of the caller, and they are made blank when the colour is not there; on a host without `grim` the search fails with the reason of `grim`.",
            "example": 'PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"',
        },
        "zh": {
            "summary": "在屏幕上寻找某种颜色的位置。区域只截取一次，然后逐像素查找该颜色。它是不依赖桌面种类的检查手段。",
            "returns": "找到颜色时为 1，否则为 0",
            "example": 'PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"',
        },
        "pt-BR": {
            "summary": "Onde uma cor está na tela. A região é capturada uma vez e percorrida atrás do primeiro pixel daquela cor. É a checagem que funciona em qualquer desktop.",
            "returns": "1 quando a cor está lá, 0 quando não está",
            "example": 'PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"',
        },
    },
    "ImageSearch": {
        "en": {
            "summary": "Looks for a picture on the screen. The region is captured once and the picture is looked for in it, with the variation and the transparent colour.",
            "params": "&X, &Y, X1, Y1, X2, Y2, Picture [, Options]",
            "returns": "one when the picture is there, zero when it is not",
            "errors": "a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.",
            "example": 'ImageSearch &x, &y, 0, 0, 800, 600, "button.png"',
        },
        "zh": {
            "summary": "在屏幕上寻找一张图片。区域只截取一次，然后在其中查找这张图片，带上容差和透明色。",
            "returns": "找到图片时为 1，否则为 0",
            "example": 'ImageSearch &x, &y, 0, 0, 800, 600, "button.png"',
        },
        "pt-BR": {
            "summary": "Procura uma imagem na tela. A região é capturada uma vez e a imagem é procurada nela, com a variação e a cor transparente.",
            "returns": "1 quando a imagem está lá, 0 quando não está",
            "example": 'ImageSearch &x, &y, 0, 0, 800, 600, "button.png"',
        },
    },
    "Screenshot": {
        "en": {
            "summary": "Captures a region of the screen into a picture file, with `grim`. The name is ours: AutoHotkey v2 has no such built-in, and it is the only name in the language that v2 would not have.",
            "params": "X, Y, Width, Height, FileName",
            "returns": "the path of the picture that was written",
            "errors": "on a host without `grim` the capture fails with the reason of `grim`.",
            "example": 'Screenshot 0, 0, 640, 480, "shot.png"',
        },
        "zh": {
            "summary": "用 `grim` 把屏幕的一块区域截成图片文件。这个名字是我们自己的：AutoHotkey v2 没有这样的内置函数，它是本语言中唯一 v2 不会有的名字。",
            "returns": "写出的图片文件路径",
            "example": 'Screenshot 0, 0, 640, 480, "shot.png"',
        },
        "pt-BR": {
            "summary": "Captura uma região da tela num arquivo de imagem, com `grim`. O nome é nosso: o AutoHotkey v2 não tem esse built-in, e é o único nome da linguagem que o v2 não teria.",
            "returns": "o caminho da imagem que foi escrita",
            "example": 'Screenshot 0, 0, 640, 480, "shot.png"',
        },
    },
    "WinExist": {
        "en": {
            "summary": "Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "the identifier of the window that is there, zero when there is none",
            "errors": "a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.",
            "example": 'if WinExist("Untitled")',
        },
        "zh": {
            "summary": "判断是否存在某个标题的窗口。窗口来自合成器，数字标识其中一个。",
            "returns": "存在时返回该窗口的标识，不存在时为 0",
            "example": 'if WinExist("Untitled")',
        },
        "pt-BR": {
            "summary": "Se existe uma janela com este título. As janelas são as do compositor, e um número nomeia uma delas.",
            "returns": "o identificador da janela que existe, zero quando não existe",
            "example": 'if WinExist("Untitled")',
        },
    },
    "WinActive": {
        "en": {
            "summary": "The window that has the focus. The box is a Wayland session, and the compositor is the one that knows the windows and which of them is in front.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "the identifier of the active window, zero when there is none",
            "errors": "the text of the window and the exclusions are not carried yet; a compositor that does not publish the focus answers with the reason, and not with a guess.",
            "example": 'id := WinActive()',
        },
        "zh": {
            "summary": "取得拥有焦点的窗口。后端是 Wayland 会话，合成器才知道有哪些窗口以及哪个在前。",
            "returns": "活动窗口的标识，没有时为 0",
            "example": 'id := WinActive()',
        },
        "pt-BR": {
            "summary": "A janela que tem o foco. O backend é uma sessão Wayland, e o compositor é quem sabe das janelas e qual delas está na frente.",
            "returns": "o identificador da janela ativa, zero quando não há nenhuma",
            "example": 'id := WinActive()',
        },
    },
    "WinGetTitle": {
        "en": {
            "summary": "The title of the active window, or of the window that matches the text that was given.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "the title of the window",
            "errors": "a window that was named and is not there answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.",
            "example": 'title := WinGetTitle()',
        },
        "zh": {
            "summary": "取得活动窗口的标题，或与给定文本匹配的窗口的标题。",
            "returns": "窗口的标题",
            "example": 'title := WinGetTitle()',
        },
        "pt-BR": {
            "summary": "O título da janela ativa, ou da janela que casa com o texto dado.",
            "returns": "o título da janela",
            "example": 'title := WinGetTitle()',
        },
    },
    "WinGetClass": {
        "en": {
            "summary": "The class of the window that matches the title that was given.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "the class of the window",
            "errors": "a title that matches no window answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.",
            "example": 'class := WinGetClass("Untitled")',
        },
        "zh": {
            "summary": "取得与给定标题匹配的窗口的类。",
            "returns": "窗口的类",
            "example": 'class := WinGetClass("Untitled")',
        },
        "pt-BR": {
            "summary": "A classe da janela que casa com o título dado.",
            "returns": "a classe da janela",
            "example": 'class := WinGetClass("Untitled")',
        },
    },
    "WinGetList": {
        "en": {
            "summary": "The identifiers of every window that matches the title that was given.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "an array of the identifiers, empty when nothing matches",
            "errors": "the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.",
            "example": 'list := WinGetList("Untitled")',
        },
        "zh": {
            "summary": "取得所有与给定标题匹配的窗口的标识。",
            "returns": "标识的数组，没有匹配时为空数组",
            "example": 'list := WinGetList("Untitled")',
        },
        "pt-BR": {
            "summary": "Os identificadores de todas as janelas que casam com o título dado.",
            "returns": "um array de identificadores, vazio quando nada casa",
            "example": 'list := WinGetList("Untitled")',
        },
    },
    "WinGetCount": {
        "en": {
            "summary": "How many windows match the title that was given.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "the number of the windows, a number",
            "errors": "the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.",
            "example": 'count := WinGetCount("Untitled")',
        },
        "zh": {
            "summary": "统计与给定标题匹配的窗口数量。",
            "returns": "窗口数量，一个数字",
            "example": 'count := WinGetCount("Untitled")',
        },
        "pt-BR": {
            "summary": "Quantas janelas casam com o título dado.",
            "returns": "o número de janelas, um número",
            "example": 'count := WinGetCount("Untitled")',
        },
    },
    "WinGetPos": {
        "en": {
            "summary": "Where the window is and how big it is, as the compositor reports it.",
            "params": "[WindowTitle] [, WinText, &X, &Y, &Width, &Height, ExcludeTitle, ExcludeText]",
            "returns": "nothing; the place and the size are written into the variables of the caller",
            "errors": "the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'",
            "example": 'WinGetPos "Untitled", , &x, &y, &w, &h',
        },
        "zh": {
            "summary": "取得窗口的位置和大小，数值来自合成器的报告。",
            "returns": "无返回值；位置和大小写入调用者的变量",
            "example": 'WinGetPos "Untitled", , &x, &y, &w, &h',
        },
        "pt-BR": {
            "summary": "Onde a janela está e quão grande ela é, como o compositor relata.",
            "returns": "nada; o lugar e o tamanho são escritos nas variáveis de quem chama",
            "example": 'WinGetPos "Untitled", , &x, &y, &w, &h',
        },
    },
    "WinGetClientPos": {
        "en": {
            "summary": "The same place, of the part of the window the program draws in. A Wayland client does not draw its own frame -- the compositor does -- so the geometry the compositor reports is the window the user sees, and it is the answer of both commands.",
            "params": "[WindowTitle] [, WinText, &X, &Y, &Width, &Height, ExcludeTitle, ExcludeText]",
            "returns": "nothing; the place and the size are written into the variables of the caller",
            "errors": "the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'",
            "example": 'WinGetClientPos "Untitled", , &x, &y, &w, &h',
        },
        "zh": {
            "summary": "取得窗口中程序绘制的那一部分的位置与大小。Wayland 客户端不自己画边框——边框由合成器画——因此合成器报告的几何就是用户看到的窗口，两条命令的回答相同。",
            "returns": "无返回值；位置和大小写入调用者的变量",
            "example": 'WinGetClientPos "Untitled", , &x, &y, &w, &h',
        },
        "pt-BR": {
            "summary": "O mesmo lugar, da parte da janela em que o programa desenha. Um cliente Wayland não desenha a própria moldura -- quem desenha é o compositor -- então a geometria que o compositor relata é a janela que o usuário vê, e é a resposta dos dois comandos.",
            "returns": "nada; o lugar e o tamanho são escritos nas variáveis de quem chama",
            "example": 'WinGetClientPos "Untitled", , &x, &y, &w, &h',
        },
    },
    "WinActivate": {
        "en": {
            "summary": "Brings a window to the front. It acts on the window of the script itself: the listing of the compositor publishes no way to raise a window of another program.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "nothing",
            "errors": "a window that is not there answers 'Target window not found.'; a window of another program cannot be activated this way.",
            "example": 'WinActivate "Untitled"',
        },
        "zh": {
            "summary": "把窗口置于最前。它作用于脚本自己的窗口：合成器的窗口列表没有提供把别的程序的窗口提到前面的办法。",
            "returns": "无返回值",
            "example": 'WinActivate "Untitled"',
        },
        "pt-BR": {
            "summary": "Traz uma janela para a frente. Age na janela do próprio script: a listagem do compositor não publica um jeito de erguer a janela de outro programa.",
            "returns": "nada",
            "example": 'WinActivate "Untitled"',
        },
    },
    "WinClose": {
        "en": {
            "summary": "Closes a window. It closes the window of the script itself: the listing of the compositor publishes no close request for a window of another program.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "nothing",
            "errors": "a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.",
            "example": 'WinClose "Untitled"',
        },
        "zh": {
            "summary": "关闭窗口。它关闭的是脚本自己的窗口：合成器的窗口列表没有为别的程序的窗口提供关闭请求。",
            "returns": "无返回值",
            "example": 'WinClose "Untitled"',
        },
        "pt-BR": {
            "summary": "Fecha uma janela. Fecha a janela do próprio script: a listagem do compositor não publica pedido de fechamento para a janela de outro programa.",
            "returns": "nada",
            "example": 'WinClose "Untitled"',
        },
    },
    "WinHide": {
        "en": {
            "summary": "Takes the window of the script off the screen.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "nothing",
            "errors": "a window that is not there answers 'Target window not found.'; a window of another program cannot be hidden this way.",
            "example": 'WinHide "Untitled"',
        },
        "zh": {
            "summary": "把脚本自己的窗口从屏幕上收起来。",
            "returns": "无返回值",
            "example": 'WinHide "Untitled"',
        },
        "pt-BR": {
            "summary": "Tira a janela do script da tela.",
            "returns": "nada",
            "example": 'WinHide "Untitled"',
        },
    },
    "WinShow": {
        "en": {
            "summary": "Puts the window of the script back on the screen, after a `WinHide`.",
            "params": "[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]",
            "returns": "nothing",
            "errors": "a window that is not there answers 'Target window not found.'; a window of another program cannot be shown this way.",
            "example": 'WinShow "Untitled"',
        },
        "zh": {
            "summary": "在 `WinHide` 之后，把脚本自己的窗口重新放回屏幕上。",
            "returns": "无返回值",
            "example": 'WinShow "Untitled"',
        },
        "pt-BR": {
            "summary": "Põe a janela do script de volta na tela, depois de um `WinHide`.",
            "returns": "nada",
            "example": 'WinShow "Untitled"',
        },
    },
}
