"""The five languages of the surface documents.

    from surface_i18n import t, LANGS, reason

The documents of `docs/surface/` are written in five languages: English,
Spanish, Portuguese of Brazil, Japanese and Chinese. The English pages
are the reference and they live at the root of the subtree; a translation
lives in a directory of its own (`es/`, `pt-BR/`, `ja/`, `zh/`) and holds
the same pages.

What is translated here is the prose of the documents and the few words
that describe a state -- the kind of a name, the state of it, the reason
of a refusal. The names themselves, the classes and the dialects are not
translated: a script writes `FileAppend` in every language of the world.

The English text is the base: a string that a language does not carry
falls back to English, so a page is never half empty because a
translation was forgotten.
"""

# The languages of the subtree, in the order they are printed.
LANGS = ("en", "es", "pt-BR", "ja", "zh")

# The name of each language, in that language.
LANG_NAMES = {
    "en": "English",
    "es": "Español",
    "pt-BR": "Português (Brasil)",
    "ja": "日本語",
    "zh": "中文",
}

# The dictionary of each language. The keys are the same in all of them.
TEXT = {}

TEXT["en"] = {
    "languages": "Languages",
    "index_title": "The index of every name",
    "index_intro": (
        "Every name of the three dialects, alphabetical, with the family it "
        "belongs to and what this tree answers for it. The **ficha** of a "
        "name -- its kind, its class, its parameters -- is in the page of "
        "its family; this page is the way in."
    ),
    "crosswalk_title": "The crosswalk of the three dialects",
    "crosswalk_intro": (
        "The same name in the three dialects, and the name that only one of "
        "them has. A name of the v1 that the v2 spells another way is a "
        "line of the v1-only table, with the v2 name that replaced it."
    ),
    "dialect_title": "The surface of %s",
    "dialect_intro": (
        "The names of %s, counted by family and by kind, and the names that "
        "this tree refuses or does not carry. The ficha of every name is in "
        "the page of its family."
    ),
    "family_intro": (
        "The names of the family **%s** in the three dialects, with what "
        "this tree answers for each of them. The family is the area a script "
        "thinks in; the names are the whole of it."
    ),
    "head_name": "Name",
    "head_kind": "Kind",
    "head_class": "Class",
    "head_parameters": "Parameters",
    "head_state": "State",
    "head_family": "Family",
    "head_dialect": "Dialect",
    "head_value": "Value",
    "head_refused": "Refused",
    "head_pending": "Pending",
    "head_reason": "Reason",
    "head_names": "Names",
    "state_value": "value",
    "state_pending": "pending",
    "state_not_measured": "not measured",
    "state_refused": "refused: %s",
    "section_in_both": "In the v1 and in the v2",
    "section_only_v1": "Only in the v1",
    "section_only_ks": "Only in the KeySharp",
    "section_by_kind": "By kind",
    "section_refused": "Refused",
    "section_pending": "Pending",
    "the_three": "The three",
    "unclassified_title": "Unclassified",
    "unclassified_intro": (
        "The names that no family claims. A name lands here when no table "
        "and no rule of `scripts/surface_areas.py` places it."
    ),
}

TEXT["es"] = {
    "languages": "Idiomas",
    "index_title": "El índice de todos los nombres",
    "index_intro": (
        "Todos los nombres de los tres dialectos, en orden alfabético, con "
        "la familia a la que pertenecen y lo que este árbol responde por "
        "ellos. La **ficha** de un nombre -- su tipo, su clase, sus "
        "parámetros -- está en la página de su familia; esta página es la "
        "entrada."
    ),
    "crosswalk_title": "La tabla de correspondencia de los tres dialectos",
    "crosswalk_intro": (
        "El mismo nombre en los tres dialectos, y el nombre que solo uno de "
        "ellos tiene. Un nombre del v1 que el v2 escribe de otra manera "
        "aparece en la tabla de solo-v1, junto al nombre del v2 que lo "
        "sustituyó."
    ),
    "dialect_title": "La superficie de %s",
    "dialect_intro": (
        "Los nombres de %s, contados por familia y por tipo, y los nombres "
        "que este árbol rechaza o no lleva. La ficha de cada nombre está en "
        "la página de su familia."
    ),
    "family_intro": (
        "Los nombres de la familia **%s** en los tres dialectos, con lo que "
        "este árbol responde por cada uno. La familia es el área en la que "
        "un script piensa; los nombres son la totalidad de ella."
    ),
    "head_name": "Nombre",
    "head_kind": "Tipo",
    "head_class": "Clase",
    "head_parameters": "Parámetros",
    "head_state": "Estado",
    "head_family": "Familia",
    "head_dialect": "Dialecto",
    "head_value": "Valor",
    "head_refused": "Rechazados",
    "head_pending": "Pendientes",
    "head_reason": "Motivo",
    "head_names": "Nombres",
    "state_value": "valor",
    "state_pending": "pendiente",
    "state_not_measured": "no medido",
    "state_refused": "rechazado: %s",
    "section_in_both": "En el v1 y en el v2",
    "section_only_v1": "Solo en el v1",
    "section_only_ks": "Solo en KeySharp",
    "section_by_kind": "Por tipo",
    "section_refused": "Rechazados",
    "section_pending": "Pendientes",
    "the_three": "Los tres",
    "unclassified_title": "Sin clasificar",
    "unclassified_intro": (
        "Los nombres que ninguna familia reclama. Un nombre llega aquí "
        "cuando ninguna tabla y ninguna regla de "
        "`scripts/surface_areas.py` lo coloca."
    ),
}

TEXT["pt-BR"] = {
    "languages": "Idiomas",
    "index_title": "O índice de todos os nomes",
    "index_intro": (
        "Todos os nomes dos três dialetos, em ordem alfabética, com a "
        "família a que pertencem e o que esta árvore responde por eles. A "
        "**ficha** de um nome -- seu tipo, sua classe, seus parâmetros -- "
        "está na página de sua família; esta página é a entrada."
    ),
    "crosswalk_title": "A tabela de correspondência dos três dialetos",
    "crosswalk_intro": (
        "O mesmo nome nos três dialetos, e o nome que só um deles tem. Um "
        "nome do v1 que o v2 escreve de outra maneira aparece na tabela de "
        "só-v1, junto com o nome do v2 que o substituiu."
    ),
    "dialect_title": "A superfície de %s",
    "dialect_intro": (
        "Os nomes de %s, contados por família e por tipo, e os nomes que "
        "esta árvore recusa ou não traz. A ficha de cada nome está na "
        "página de sua família."
    ),
    "family_intro": (
        "Os nomes da família **%s** nos três dialetos, com o que esta "
        "árvore responde por cada um. A família é a área em que um script "
        "pensa; os nomes são a totalidade dela."
    ),
    "head_name": "Nome",
    "head_kind": "Tipo",
    "head_class": "Classe",
    "head_parameters": "Parâmetros",
    "head_state": "Estado",
    "head_family": "Família",
    "head_dialect": "Dialeto",
    "head_value": "Valor",
    "head_refused": "Recusados",
    "head_pending": "Pendentes",
    "head_reason": "Motivo",
    "head_names": "Nomes",
    "state_value": "valor",
    "state_pending": "pendente",
    "state_not_measured": "não medido",
    "state_refused": "recusado: %s",
    "section_in_both": "No v1 e no v2",
    "section_only_v1": "Só no v1",
    "section_only_ks": "Só no KeySharp",
    "section_by_kind": "Por tipo",
    "section_refused": "Recusados",
    "section_pending": "Pendentes",
    "the_three": "Os três",
    "unclassified_title": "Sem classificação",
    "unclassified_intro": (
        "Os nomes que nenhuma família reivindica. Um nome chega aqui quando "
        "nenhuma tabela e nenhuma regra de "
        "`scripts/surface_areas.py` o coloca."
    ),
}

TEXT["ja"] = {
    "languages": "言語",
    "index_title": "すべての名前の索引",
    "index_intro": (
        "3つの方言のすべての名前を五十音順に、それぞれのファミリーと、"
        "このツリーがその名前に対して答える内容ととともに示します。"
        "名前の**詳細票**（種類・クラス・引数）はそのファミリーのページに"
        "あります。このページはその入口です。"
    ),
    "crosswalk_title": "3つの方言の対応表",
    "crosswalk_intro": (
        "同じ名前が3つの方言でどう書かれるか、そして一方にしか存在しない"
        "名前。v1 の名前が v2 で別の綴りになるものは「v1 のみ」の表に、"
        "置き換わった v2 の名前と並べてあります。"
    ),
    "dialect_title": "%s のサーフェス",
    "dialect_intro": (
        "%s の名前をファミリー別・種類別に数え、このツリーが拒否するものや"
        "持たないものも示します。すべての名前の詳細票はファミリーのページに"
        "あります。"
    ),
    "family_intro": (
        "3つの方言におけるファミリー **%s** の名前と、このツリーがその"
        "それぞれに答える内容。ファミリーはスクリプトが考え方のまとまりと"
        "する領域であり、その名前群がそのすべてです。"
    ),
    "head_name": "名前",
    "head_kind": "種類",
    "head_class": "クラス",
    "head_parameters": "引数",
    "head_state": "状態",
    "head_family": "ファミリー",
    "head_dialect": "方言",
    "head_value": "値",
    "head_refused": "拒否",
    "head_pending": "保留",
    "head_reason": "理由",
    "head_names": "名前数",
    "state_value": "値",
    "state_pending": "保留",
    "state_not_measured": "未計測",
    "state_refused": "拒否: %s",
    "section_in_both": "v1 と v2 の両方にあるもの",
    "section_only_v1": "v1 のみにあるもの",
    "section_only_ks": "KeySharp のみにあるもの",
    "section_by_kind": "種類別",
    "section_refused": "拒否されるもの",
    "section_pending": "保留されるもの",
    "the_three": "3つの方言",
    "unclassified_title": "未分類",
    "unclassified_intro": (
        "どのファミリーも属さないとしない名前。表にも "
        "`scripts/surface_areas.py` の規則にも置かれない名前は、ここに"
        "着く。"
    ),
}

TEXT["zh"] = {
    "languages": "语言",
    "index_title": "全部名称索引",
    "index_intro": (
        "按字母顺序列出三个方言的每一个名称，并标注它所属的分类，以及这棵"
        "树对它给出的回答。名称的**技术卡**（种类、类、参数）位于其分类"
        "页面；本页是入口。"
    ),
    "crosswalk_title": "三个方言的对照表",
    "crosswalk_intro": (
        "同一个名称在三个方言中的写法，以及只有其中一个方言才有的名称。"
        "v1 中的名称在 v2 中写法不同时，会列入「仅 v1」表，并给出替代它的 "
        "v2 名称。"
    ),
    "dialect_title": "%s 的表面",
    "dialect_intro": (
        "按分类与种类统计 %s 的名称，并列出这棵树拒绝或尚未携带的名称。"
        "每个名称的技术卡位于其分类页面。"
    ),
    "family_intro": (
        "三个方言中属于 **%s** 类的名称，以及这棵树对每个名称给出的回答。"
        "分类是脚本思考的领域，这些名称即是它的全部。"
    ),
    "head_name": "名称",
    "head_kind": "种类",
    "head_class": "类",
    "head_parameters": "参数",
    "head_state": "状态",
    "head_family": "分类",
    "head_dialect": "方言",
    "head_value": "值",
    "head_refused": "拒绝",
    "head_pending": "待办",
    "head_reason": "原因",
    "head_names": "名称数",
    "state_value": "值",
    "state_pending": "待办",
    "state_not_measured": "未测量",
    "state_refused": "拒绝：%s",
    "section_in_both": "v1 与 v2 共有",
    "section_only_v1": "仅 v1 独有",
    "section_only_ks": "仅 KeySharp 独有",
    "section_by_kind": "按种类",
    "section_refused": "被拒绝的名称",
    "section_pending": "待办的名称",
    "the_three": "三个方言",
    "unclassified_title": "未分类",
    "unclassified_intro": (
        "没有任何分类认领的名称。当表格与 "
        "`scripts/surface_areas.py` 的规则都无法归类时，名称会落到这里。"
    ),
}


# The title of a family, in each language. The key of the family stays in
# English: it is the name of the page and of the file.
FAMILY_TITLES = {
    "en": {
        "script": "Parser and script execution",
        "files": "File and directory operations",
        "input": "Hotkeys, hotstrings, and input",
        "gui": "GUI windows and window automation",
        "screen": "Screen capture and pixel functions",
        "native": "Native function calls (DllCall)",
        "registry": "Registry APIs",
        "com": "COM automation (ComObject)",
        "core": "The core of the language: text, math and patterns",
        "objects": "The objects of the language: Array, Map and Func",
        "time": "Dates and times",
        "machine": "The machine, the processes and the environment",
        "clipboard": "The clipboard",
        "sound": "Sound",
    },
    "es": {
        "script": "El analizador y la ejecución del script",
        "files": "Operaciones de archivos y directorios",
        "input": "Hotkeys, hotstrings y entrada",
        "gui": "Ventanas de la GUI y automatización de ventanas",
        "screen": "Captura de pantalla y funciones de píxel",
        "native": "Llamadas a funciones nativas (DllCall)",
        "registry": "API del registro",
        "com": "Automatización COM (ComObject)",
        "core": "El núcleo del lenguaje: texto, matemáticas y patrones",
        "objects": "Los objetos del lenguaje: Array, Map y Func",
        "time": "Fechas y horas",
        "machine": "La máquina, los procesos y el entorno",
        "clipboard": "El portapapeles",
        "sound": "El sonido",
    },
    "pt-BR": {
        "script": "O analisador e a execução do script",
        "files": "Operações de arquivos e diretórios",
        "input": "Hotkeys, hotstrings e entrada",
        "gui": "Janelas da GUI e automação de janelas",
        "screen": "Captura de tela e funções de pixel",
        "native": "Chamadas a funções nativas (DllCall)",
        "registry": "APIs do registro",
        "com": "Automação COM (ComObject)",
        "core": "O núcleo da linguagem: texto, matemática e padrões",
        "objects": "Os objetos da linguagem: Array, Map e Func",
        "time": "Datas e horas",
        "machine": "A máquina, os processos e o ambiente",
        "clipboard": "A área de transferência",
        "sound": "O som",
    },
    "ja": {
        "script": "パーサとスクリプトの実行",
        "files": "ファイルとディレクトリの操作",
        "input": "ホットキー、ホット文字列、入力",
        "gui": "GUIウィンドウとウィンドウの自動化",
        "screen": "画面キャプチャとピクセル関数",
        "native": "ネイティブ関数の呼び出し (DllCall)",
        "registry": "レジストリの API",
        "com": "COM 自動化 (ComObject)",
        "core": "言語の中核: テキスト、数値、正規表現",
        "objects": "言語のオブジェクト: Array、Map、Func",
        "time": "日付と時刻",
        "machine": "マシン、プロセス、環境",
        "clipboard": "クリップボード",
        "sound": "サウンド",
    },
    "zh": {
        "script": "解析器与脚本执行",
        "files": "文件与目录操作",
        "input": "热键、热字符串与输入",
        "gui": "GUI 窗口与窗口自动化",
        "screen": "屏幕捕获与像素函数",
        "native": "原生函数调用 (DllCall)",
        "registry": "注册表 API",
        "com": "COM 自动化 (ComObject)",
        "core": "语言核心：文本、数学与模式",
        "objects": "语言的对象：Array、Map 与 Func",
        "time": "日期与时间",
        "machine": "机器、进程与环境",
        "clipboard": "剪贴板",
        "sound": "声音",
    },
}


# The reason of a refusal, in each language. The key is the English text
# that the runner answers.
REASONS = {
    "this is a name of Windows, and this desktop is not Windows.": {
        "es": "es un nombre de Windows, y este escritorio no es Windows.",
        "pt-BR": "é um nome do Windows, e esta área de trabalho não é o Windows.",
        "ja": "これは Windows の名前であり、このデスクトップは Windows ではありません。",
        "zh": "这是 Windows 的名称，而本桌面不是 Windows。",
    },
    "the registry is a thing of Windows, and this runtime does not carry it.": {
        "es": "el registro es cosa de Windows, y este runtime no lo lleva.",
        "pt-BR": "o registro é coisa do Windows, e este runtime não o traz.",
        "ja": "レジストリは Windows のもので、このランタイムは持ちません。",
        "zh": "注册表属于 Windows，此运行时并不携带它。",
    },
    "this runtime has no tray icon.": {
        "es": "este runtime no tiene icono de bandeja.",
        "pt-BR": "este runtime não tem ícone da bandeja.",
        "ja": "このランタイムにトレイアイコンはありません。",
        "zh": "此运行时没有托盘图标。",
    },
    "this runtime runs one script in one thread, and it cannot make it critical.": {
        "es": "este runtime ejecuta un script en un hilo, y no puede volverlo crítico.",
        "pt-BR": "este runtime executa um script em uma thread, e não pode torná-lo crítico.",
        "ja": "このランタイムは1つのスクリプトを1つのスレッドで実行し、クリティカルにはできません。",
        "zh": "此运行时在一个线程中运行一个脚本，无法将其设为关键。",
    },
    "the lines are listed in a window of v2, and this runtime has no window to show them in.": {
        "es": "las líneas se listan en una ventana de v2, y este runtime no tiene ventana donde mostrarlas.",
        "pt-BR": "as linhas são listadas em uma janela do v2, e este runtime não tem janela para mostrá-las.",
        "ja": "行は v2 のウィンドウに表示されますが、このランタイムにウィンドウはありません。",
        "zh": "行列表显示在 v2 的窗口中，而此运行时没有窗口可显示。",
    },
    "the variables are listed in a window of v2, and this runtime has no window to show them in.": {
        "es": "las variables se listan en una ventana de v2, y este runtime no tiene ventana donde mostrarlas.",
        "pt-BR": "as variáveis são listadas em uma janela do v2, e este runtime não tem janela para mostrá-las.",
        "ja": "変数は v2 のウィンドウに表示されますが、このランタイムにウィンドウはありません。",
        "zh": "变量列表显示在 v2 的窗口中，而此运行时没有窗口可显示。",
    },
    "this runtime is a script that ends when it ends, and it cannot be kept alive.": {
        "es": "este runtime es un script que acaba cuando acaba, y no puede mantenerse vivo.",
        "pt-BR": "este runtime é um script que termina quando termina, e não pode ser mantido vivo.",
        "ja": "このランタイムは終わると終わるスクリプトで，持续させることはできません。",
        "zh": "此运行时是运行结束即结束的脚本，无法保持存活。",
    },
    "this runtime runs one script once, and it cannot run itself again.": {
        "es": "este runtime ejecuta un script una vez, y no puede ejecutarse otra vez.",
        "pt-BR": "este runtime executa um script uma vez, e não pode se executar de novo.",
        "ja": "このランタイムはスクリプトを1度だけ実行し、自分を再実行できません。",
        "zh": "此运行时只运行一次脚本，无法再次运行自身。",
    },
    "this runtime runs one script in one thread, and it cannot start another.": {
        "es": "este runtime ejecuta un script en un hilo, y no puede iniciar otro.",
        "pt-BR": "este runtime executa um script em uma thread, e não pode iniciar outra.",
        "ja": "このランタイムは1つのスクリプトを1つのスレッドで実行し、別のスレッドを開始できません。",
        "zh": "此运行时在一个线程中运行一个脚本，无法再启动一个。",
    },
    "this runtime reads no text of a window.": {
        "es": "este runtime no lee el texto de una ventana.",
        "pt-BR": "este runtime não lê o texto de uma janela.",
        "ja": "このランタイムはウィンドウのテキストを読みません。",
        "zh": "此运行时不读取窗口文本。",
    },
    "the shape of the cursor of the machine is not read by this runtime.": {
        "es": "la forma del cursor de la máquina no la lee este runtime.",
        "pt-BR": "este runtime não lê o formato do cursor da máquina.",
        "ja": "マシンのカーソル形状はこのランタイムでは読み取られません。",
        "zh": "此运行时不读取机器光标的形状。",
    },
    "this desktop does not know the scale of the output.": {
        "es": "este escritorio no conoce la escala de la salida.",
        "pt-BR": "esta área de trabalho não conhece a escala da saída.",
        "ja": "このデスクトップは出力のスケールを知りません。",
        "zh": "本桌面不知道输出的缩放比例。",
    },
    "this runtime has no window of its own: it is a script that runs and ends.": {
        "es": "este runtime no tiene ventana propia: es un script que se ejecuta y acaba.",
        "pt-BR": "este runtime não tem janela própria: é um script que roda e termina.",
        "ja": "このランタイムに独自のウィンドウはありません。実行されて終わるスクリプトです。",
        "zh": "此运行时没有自己的窗口：它是一个运行即结束的脚本。",
    },
    "this desktop does not know the address of the network of the machine.": {
        "es": "este escritorio no conoce la dirección de la red de la máquina.",
        "pt-BR": "esta área de trabalho não conhece o endereço de rede da máquina.",
        "ja": "このデスクトップはマシンのネットワークアドレスを知りません。",
        "zh": "本桌面不知道机器的网络地址。",
    },
    "this name is about Windows, and this machine is not.": {
        "es": "este nombre es cosa de Windows, y esta máquina no lo es.",
        "pt-BR": "este nome é coisa do Windows, e esta máquina não é.",
        "ja": "この名前は Windows のことで、このマシンは Windows ではありません。",
        "zh": "此名称与 Windows 有关，而本机不是 Windows。",
    },
}


def t(lang, key):
    """The text of a key, in a language; English when it is missing."""
    table = TEXT.get(lang) or TEXT["en"]
    if key in table:
        return table[key]
    return TEXT["en"][key]


def family_title(lang, key):
    """The title of a family, in a language."""
    table = FAMILY_TITLES.get(lang) or FAMILY_TITLES["en"]
    return table.get(key) or FAMILY_TITLES["en"][key]


def reason(lang, english):
    """The reason of a refusal, in a language."""
    if lang == "en":
        return english
    table = REASONS.get(english)
    if not table:
        return english
    return table.get(lang, english)
