"""The behavior of the class types of the v2 dialect and of their members.

    from surface_behavior import behavior_of

A class type of the runtime is a class the script cannot extend: its
instance is an object, its methods are methods, and its `__Class` is its
name. The file says what each class is, what each method does and what
each refuses, in the three languages of the subtree.
"""

BEHAVIOR = {
    "Array": {
        "en": {
            "summary": "The class of a row of values. `Array(...)` makes one with the values it is given, and the literal `[1, 2, 3]` makes one too.",
            "params": "the values of the array, any number of them",
            "returns": "the object of the class `Array`",
            "errors": "nothing to make the array; the places count from one, and an index that is not there is refused with \"Invalid index.\".",
            "example": 'a := Array(1, 2, 3)\nFileAppend a.Length, "*"',
        },
        "zh": {
            "summary": "一排值的类。`Array(...)` 用给定的值造出数组，字面量 `[1, 2, 3]` 也一样。",
            "returns": "类为 `Array` 的对象",
            "example": 'a := Array(1, 2, 3)\nFileAppend a.Length, "*"',
        },
        "pt-BR": {
            "summary": "A classe de uma fileira de valores. `Array(...)` cria uma com os valores dados, e o literal `[1, 2, 3]` cria uma também.",
            "returns": "o objeto da classe `Array`",
            "example": 'a := Array(1, 2, 3)\nFileAppend a.Length, "*"',
        },
    },
    "Map": {
        "en": {
            "summary": "The class of a table of keys and values. `Map(key, value, ...)` makes one, and the order of the entries is the order they were put in.",
            "params": "the pairs of key and value, an even number of them",
            "returns": "the object of the class `Map`",
            "errors": "an odd number of arguments is refused with \"Invalid number of parameters.\"; a key that is not there is refused with \"Item has no value.\".",
            "example": 'm := Map("x", 1)\nFileAppend m.Count, "*"',
        },
        "zh": {
            "summary": "键和值的表的类。`Map(key, value, ...)` 造出这样一个表，各条目的顺序就是放进去的顺序。",
            "returns": "类为 `Map` 的对象",
            "example": 'm := Map("x", 1)\nFileAppend m.Count, "*"',
        },
        "pt-BR": {
            "summary": "A classe de uma tabela de chaves e valores. `Map(key, value, ...)` cria uma, e a ordem das entradas é a ordem em que foram colocadas.",
            "returns": "o objeto da classe `Map`",
            "example": 'm := Map("x", 1)\nFileAppend m.Count, "*"',
        },
    },
    "File": {
        "en": {
            "summary": "The class of an open file of the machine. The class is not made: `File(...)` answers \"Invalid base.\"; the file is made by the `FileOpen` built-in, and it is the only way to have one.",
            "params": "nothing that makes it; the arguments of `FileOpen` open the file",
            "returns": "the object of the class `File`",
            "errors": "the call of the class is refused with \"Invalid base.\".",
            "example": 'f := FileOpen("notes.txt", "w")\nf.Close()',
        },
        "zh": {
            "summary": "机器上已打开文件的类。这个类造不出来：`File(...)` 回答“Invalid base.”；文件由内置函数 `FileOpen` 造出，这是唯一的办法。",
            "returns": "类为 `File` 的对象",
            "example": 'f := FileOpen("notes.txt", "w")\nf.Close()',
        },
        "pt-BR": {
            "summary": "A classe de um arquivo aberto da máquina. A classe não é criada: `File(...)` responde \"Invalid base.\"; o arquivo é criado pelo embutido `FileOpen`, e é o único jeito de ter um.",
            "returns": "o objeto da classe `File`",
            "example": 'f := FileOpen("notes.txt", "w")\nf.Close()',
        },
    },
    "Object": {
        "en": {
            "summary": "The root of the chain of the classes: every object of the runtime is one. `Object()` makes an object with nothing in it, and the literal `{a: 1}` makes one with a property.",
            "params": "nothing",
            "returns": "the object of the class `Object`",
            "errors": "nothing; a member that is not there is refused with the type of the value and the name of the member.",
            "example": 'o := Object()\nFileAppend o.__Class, "*"',
        },
        "zh": {
            "summary": "类链的根：运行时的每个对象都是它。`Object()` 造出一个空对象，字面量 `{a: 1}` 造出一个带属性的对象。",
            "returns": "类为 `Object` 的对象",
            "example": 'o := Object()\nFileAppend o.__Class, "*"',
        },
        "pt-BR": {
            "summary": "A raiz da cadeia das classes: todo objeto do runtime é um. `Object()` cria um objeto vazio, e o literal `{a: 1}` cria um com uma propriedade.",
            "returns": "o objeto da classe `Object`",
            "example": 'o := Object()\nFileAppend o.__Class, "*"',
        },
    },
    "Func": {
        "en": {
            "summary": "The class of a function: a function of the script, a method, a lambda or a built-in. The class is not made: `Func(...)` answers \"Invalid base.\"; the class of a function value is `Func`.",
            "params": "nothing that makes it",
            "returns": "the class of a function value, as a text",
            "errors": "the call of the class is refused with \"Invalid base.\".",
            "example": 'FileAppend Abs.__Class, "*"',
        },
        "zh": {
            "summary": "函数的类：脚本里的函数、方法、lambda 或内置函数。这个类造不出来：`Func(...)` 回答“Invalid base.”；函数值的类就是 `Func`。",
            "returns": "函数值的类，作为文本",
            "example": 'FileAppend Abs.__Class, "*"',
        },
        "pt-BR": {
            "summary": "A classe de uma função: uma função do script, um método, um lambda ou um embutido. A classe não é criada: `Func(...)` responde \"Invalid base.\"; a classe de um valor de função é `Func`.",
            "returns": "a classe de um valor de função, como um texto",
            "example": 'FileAppend Abs.__Class, "*"',
        },
    },
    "Enumerator": {
        "en": {
            "summary": "The class of the walker of a loop: an enumerator hands out the pieces of a sequence, one turn at a time. The class is not made; the `__Enum` of a value makes one, and the call of it is the turn of the enumeration.",
            "params": "nothing that makes it",
            "returns": "the class of an enumerator, as a text",
            "errors": "the call of the class is refused with \"Invalid base.\".",
            "example": 'for item in [1, 2]\n    FileAppend Type(item), "*"',
        },
        "zh": {
            "summary": "循环的行走者的类：枚举器一次交出一个序列的一段。这个类造不出来；值的 `__Enum` 造出它，对它的调用就是枚举的一轮。",
            "returns": "枚举器的类，作为文本",
            "example": 'for item in [1, 2]\n    FileAppend Type(item), "*"',
        },
        "pt-BR": {
            "summary": "A classe do andarilho de um laço: um enumerador entrega as partes de uma sequência, uma volta por vez. A classe não é criada; o `__Enum` de um valor cria uma, e a chamada dela é a volta da enumeração.",
            "returns": "a classe de um enumerador, como um texto",
            "example": 'for item in [1, 2]\n    FileAppend Type(item), "*"',
        },
    },
    "Array.Push": {
        "en": {
            "summary": "Puts the values at the end of the array and answers the array, so that the calls can be chained.",
            "params": "the values, any number of them",
            "returns": "the array itself",
            "errors": "nothing; the push of no value changes nothing and still answers the array.",
            "example": 'a := [1]\na.Push(2, 3)\nFileAppend a.Length, "*"',
        },
        "zh": {
            "summary": "把值放到数组末尾，并回答这个数组，因此调用可以串起来。",
            "returns": "数组本身",
            "example": 'a := [1]\na.Push(2, 3)\nFileAppend a.Length, "*"',
        },
        "pt-BR": {
            "summary": "Coloca os valores no fim do array e responde o próprio array, para que as chamadas possam ser encadeadas.",
            "returns": "o próprio array",
            "example": 'a := [1]\na.Push(2, 3)\nFileAppend a.Length, "*"',
        },
    },
    "Array.Pop": {
        "en": {
            "summary": "Takes the last value off the array and answers it.",
            "params": "nothing",
            "returns": "the value that was at the end, as a text when the place had no value of its own",
            "errors": "an empty array is refused with \"Array is empty.\".",
            "example": 'a := [1, 2]\nFileAppend a.Pop(), "*"',
        },
        "zh": {
            "summary": "从数组末尾取走最后一个值并回答它。",
            "returns": "末尾那个值；该位置没有自己的值时是空文本",
            "example": 'a := [1, 2]\nFileAppend a.Pop(), "*"',
        },
        "pt-BR": {
            "summary": "Tira o último valor do array e responde o valor.",
            "returns": "o valor que estava no fim, como um texto quando o lugar não tinha valor próprio",
            "example": 'a := [1, 2]\nFileAppend a.Pop(), "*"',
        },
    },
    "Array.Length": {
        "en": {
            "summary": "The number of places of the array. It is a property and not a method: `a.Length`, and not `a.Length()`.",
            "params": "nothing; it is a property, not a call",
            "returns": "the number of the places, as a number",
            "errors": "the write of a negative number is refused with \"Invalid value.\"; a longer array gets places that have no value of their own, and a shorter one loses the places at the end.",
            "example": 'a := [1, 2, 3]\nFileAppend a.Length, "*"',
        },
        "zh": {
            "summary": "数组位置的个数。它是属性而不是方法：写 `a.Length`，不写 `a.Length()`。",
            "returns": "位置的个数，作为数字",
            "example": 'a := [1, 2, 3]\nFileAppend a.Length, "*"',
        },
        "pt-BR": {
            "summary": "O número de lugares do array. É uma propriedade e não um método: `a.Length`, e não `a.Length()`.",
            "returns": "o número de lugares, como um número",
            "example": 'a := [1, 2, 3]\nFileAppend a.Length, "*"',
        },
    },
    "Map.Set": {
        "en": {
            "summary": "Puts the pairs of key and value into the map and answers the map, so that the calls can be chained.",
            "params": "the pairs of key and value, an even number of them",
            "returns": "the map itself",
            "errors": "an odd number of arguments is refused with \"Invalid number of parameters.\".",
            "example": 'm := Map()\nm.Set("x", 1)\nFileAppend m.Count, "*"',
        },
        "zh": {
            "summary": "把键和值的对放进映射，并回答这个映射，因此调用可以串起来。",
            "returns": "映射本身",
            "example": 'm := Map()\nm.Set("x", 1)\nFileAppend m.Count, "*"',
        },
        "pt-BR": {
            "summary": "Coloca os pares de chave e valor no mapa e responde o próprio mapa, para que as chamadas possam ser encadeadas.",
            "returns": "o próprio mapa",
            "example": 'm := Map()\nm.Set("x", 1)\nFileAppend m.Count, "*"',
        },
    },
    "Map.Get": {
        "en": {
            "summary": "The value that is under a key of the map.",
            "params": "the key, and the value to answer when the key is not there",
            "returns": "the value under the key",
            "errors": "a key that is not there is refused with \"Item has no value.\", and the key is named in the `Extra` of the failure; the second argument is the default of the call, and the `Default` of the map is the last word.",
            "example": 'm := Map("x", 1)\nFileAppend m.Get("x"), "*"',
        },
        "zh": {
            "summary": "映射中某个键下面的值。",
            "returns": "该键下面的值",
            "example": 'm := Map("x", 1)\nFileAppend m.Get("x"), "*"',
        },
        "pt-BR": {
            "summary": "O valor que está sob uma chave do mapa.",
            "returns": "o valor sob a chave",
            "example": 'm := Map("x", 1)\nFileAppend m.Get("x"), "*"',
        },
    },
    "Map.Has": {
        "en": {
            "summary": "Whether the map has a key.",
            "params": "the key",
            "returns": "1 when the key is there, 0 when it is not",
            "errors": "nothing; a key that is not there is not a failure, the question is answered.",
            "example": 'm := Map("x", 1)\nFileAppend m.Has("x"), "*"',
        },
        "zh": {
            "summary": "映射中是否有某个键。",
            "returns": "键在时为 1，不在时为 0",
            "example": 'm := Map("x", 1)\nFileAppend m.Has("x"), "*"',
        },
        "pt-BR": {
            "summary": "Se o mapa tem uma chave.",
            "returns": "1 quando a chave está lá, 0 quando não está",
            "example": 'm := Map("x", 1)\nFileAppend m.Has("x"), "*"',
        },
    },
    "Map.Delete": {
        "en": {
            "summary": "Takes a key out of the map and answers the value that was under it.",
            "params": "the key",
            "returns": "the value that was under the key",
            "errors": "a key that is not there is refused with \"Item has no value.\", and the key is named in the `Extra` of the failure.",
            "example": 'm := Map("x", 1)\nFileAppend m.Delete("x"), "*"',
        },
        "zh": {
            "summary": "把一个键从映射里取走，并回答它下面原本的值。",
            "returns": "该键下面原本的值",
            "example": 'm := Map("x", 1)\nFileAppend m.Delete("x"), "*"',
        },
        "pt-BR": {
            "summary": "Tira uma chave do mapa e responde o valor que estava sob ela.",
            "returns": "o valor que estava sob a chave",
            "example": 'm := Map("x", 1)\nFileAppend m.Delete("x"), "*"',
        },
    },
    "File.Read": {
        "en": {
            "summary": "Reads the text of the file at its position and moves the position on.",
            "params": "the number of characters, which is optional",
            "returns": "the text, as a text; without a number it is the rest of the file",
            "errors": "a closed file, a file opened to be written and the console answer the empty text; a negative count is refused with \"the count of characters cannot be negative.\"; an encoding the runtime cannot read is refused with the reason.",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.Read(5), "*"',
        },
        "zh": {
            "summary": "读取文件当前位置上的文本，并把位置往后移。",
            "returns": "文本；不给数字时是文件的剩余部分",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.Read(5), "*"',
        },
        "pt-BR": {
            "summary": "Lê o texto do arquivo na sua posição e avança a posição.",
            "returns": "o texto, como um texto; sem um número é o resto do arquivo",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.Read(5), "*"',
        },
    },
    "File.ReadLine": {
        "en": {
            "summary": "Reads the line of the file at its position and moves the position on.",
            "params": "nothing",
            "returns": "the line, as a text, without the newline that ends it",
            "errors": "a closed file, a file opened to be written and the console answer the empty text; an encoding the runtime cannot read is refused with the reason.",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.ReadLine(), "*"',
        },
        "zh": {
            "summary": "读取文件当前位置上的那一行，并把位置往后移。",
            "returns": "该行，作为文本，不带结尾的换行符",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.ReadLine(), "*"',
        },
        "pt-BR": {
            "summary": "Lê a linha do arquivo na sua posição e avança a posição.",
            "returns": "a linha, como um texto, sem a quebra de linha que a termina",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.ReadLine(), "*"',
        },
    },
    "File.Write": {
        "en": {
            "summary": "Writes a text into the file at its position and moves the position on.",
            "params": "the text",
            "returns": "the number of characters that were written",
            "errors": "a closed file takes the text and keeps none; the write of the machine fails with the reason of the desktop.",
            "example": 'f := FileOpen("notes.txt", "w")\nFileAppend f.Write("hello"), "*"',
        },
        "zh": {
            "summary": "把一段文本写到文件的位置上，并把位置往后移。",
            "returns": "写入的字符数",
            "example": 'f := FileOpen("notes.txt", "w")\nFileAppend f.Write("hello"), "*"',
        },
        "pt-BR": {
            "summary": "Escreve um texto no arquivo na sua posição e avança a posição.",
            "returns": "o número de caracteres que foram escritos",
            "example": 'f := FileOpen("notes.txt", "w")\nFileAppend f.Write("hello"), "*"',
        },
    },
    "File.WriteLine": {
        "en": {
            "summary": "Writes a text and a newline into the file at its position and moves the position on.",
            "params": "the text",
            "returns": "the number of characters that were written, the newline included",
            "errors": "a closed file takes the text and keeps none; the write of the machine fails with the reason of the desktop.",
            "example": 'f := FileOpen("notes.txt", "w")\nFileAppend f.WriteLine("hello"), "*"',
        },
        "zh": {
            "summary": "把一段文本和一个换行写到文件的位置上，并把位置往后移。",
            "returns": "写入的字符数，换行也算在内",
            "example": 'f := FileOpen("notes.txt", "w")\nFileAppend f.WriteLine("hello"), "*"',
        },
        "pt-BR": {
            "summary": "Escreve um texto e uma quebra de linha no arquivo na sua posição e avança a posição.",
            "returns": "o número de caracteres que foram escritos, a quebra de linha incluída",
            "example": 'f := FileOpen("notes.txt", "w")\nFileAppend f.WriteLine("hello"), "*"',
        },
    },
    "File.Close": {
        "en": {
            "summary": "Closes the file and gives the handle back to the machine.",
            "params": "nothing",
            "returns": "the empty text",
            "errors": "the console is closed with the empty text; a close that the machine refuses fails with the reason of the desktop.",
            "example": 'f := FileOpen("notes.txt", "w")\nf.Close()',
        },
        "zh": {
            "summary": "关闭文件，把句柄还给机器。",
            "returns": "空文本",
            "example": 'f := FileOpen("notes.txt", "w")\nf.Close()',
        },
        "pt-BR": {
            "summary": "Fecha o arquivo e devolve o handle para a máquina.",
            "returns": "o texto vazio",
            "example": 'f := FileOpen("notes.txt", "w")\nf.Close()',
        },
    },
    "File.AtEOF": {
        "en": {
            "summary": "Whether the file is at its end. It is a property and not a method: `f.AtEOF`.",
            "params": "nothing; it is a property, not a call",
            "returns": "1 when the file is at its end, 0 when it is not",
            "errors": "a closed file answers one, for there is nothing more to read; the size of a file the machine cannot measure fails with the reason of the desktop.",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.AtEOF, "*"',
        },
        "zh": {
            "summary": "文件是否已到末尾。它是属性而不是方法：`f.AtEOF`。",
            "returns": "文件到末尾时为 1，否则为 0",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.AtEOF, "*"',
        },
        "pt-BR": {
            "summary": "Se o arquivo está no fim. É uma propriedade e não um método: `f.AtEOF`.",
            "returns": "1 quando o arquivo está no fim, 0 quando não está",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.AtEOF, "*"',
        },
    },
    "File.Pos": {
        "en": {
            "summary": "The position of the file, in bytes. It is a property and not a method: `f.Pos`.",
            "params": "nothing; it is a property, not a call",
            "returns": "the position, as a number; a closed file answers -1",
            "errors": "the write of a negative position is refused with \"the position of a file cannot be negative.\".",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.Pos, "*"',
        },
        "zh": {
            "summary": "文件的位置，单位是字节。它是属性而不是方法：`f.Pos`。",
            "returns": "位置，作为数字；已关闭的文件回答 -1",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.Pos, "*"',
        },
        "pt-BR": {
            "summary": "A posição do arquivo, em bytes. É uma propriedade e não um método: `f.Pos`.",
            "returns": "a posição, como um número; um arquivo fechado responde -1",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.Pos, "*"',
        },
    },
    "File.Length": {
        "en": {
            "summary": "The size of the file, in bytes. It is a property and not a method: `f.Length`.",
            "params": "nothing; it is a property, not a call",
            "returns": "the size, as a number; a closed file answers zero",
            "errors": "the write of a size below the size of the file is refused: this desktop cannot make a file shorter, it cannot cut one.",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.Length, "*"',
        },
        "zh": {
            "summary": "文件的大小，单位是字节。它是属性而不是方法：`f.Length`。",
            "returns": "大小，作为数字；已关闭的文件回答零",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.Length, "*"',
        },
        "pt-BR": {
            "summary": "O tamanho do arquivo, em bytes. É uma propriedade e não um método: `f.Length`.",
            "returns": "o tamanho, como um número; um arquivo fechado responde zero",
            "example": 'f := FileOpen("notes.txt", "r")\nFileAppend f.Length, "*"',
        },
    },
    "ObjGetBase": {
        "en": {
            "summary": "The base of an object: the prototype of its class.",
            "params": "the object",
            "returns": "the object of the base",
            "errors": "a value that is not an object is refused with \"Expected an object but got ...\".",
            "example": 'FileAppend ObjGetBase({a: 1}).__Class, "*"',
        },
        "zh": {
            "summary": "对象的基类：它那个类的原型。",
            "returns": "基类的对象",
            "example": 'FileAppend ObjGetBase({a: 1}).__Class, "*"',
        },
        "pt-BR": {
            "summary": "A base de um objeto: o protótipo da sua classe.",
            "returns": "o objeto da base",
            "example": 'FileAppend ObjGetBase({a: 1}).__Class, "*"',
        },
    },
    "HasBase": {
        "en": {
            "summary": "Whether the base of an object is another object.",
            "params": "the object, and the base",
            "returns": "1 when the base is that one, 0 when it is not",
            "errors": "a base that is not an object is refused before the question of the object is asked.",
            "example": 'FileAppend HasBase({a: 1}, {}), "*"',
        },
        "zh": {
            "summary": "某个对象的基类是不是另一个对象。",
            "returns": "基类是它时为 1，否则为 0",
            "example": 'FileAppend HasBase({a: 1}, {}), "*"',
        },
        "pt-BR": {
            "summary": "Se a base de um objeto é outro objeto.",
            "returns": "1 quando a base é aquela, 0 quando não é",
            "example": 'FileAppend HasBase({a: 1}, {}), "*"',
        },
    },
    "HasMethod": {
        "en": {
            "summary": "Whether an object has a method.",
            "params": "the object, the name of the method, and the number of parameters it would take",
            "returns": "1 when the object has such a method, 0 when it has not",
            "errors": "nothing; a value that is not an object answers zero, and the question with no name asks whether the value is a class.",
            "example": 'o := {a: 1}\nFileAppend HasMethod(o, "a"), "*"',
        },
        "zh": {
            "summary": "某个对象是否有某个方法。",
            "returns": "对象有该方法时为 1，没有时为 0",
            "example": 'o := {a: 1}\nFileAppend HasMethod(o, "a"), "*"',
        },
        "pt-BR": {
            "summary": "Se um objeto tem um método.",
            "returns": "1 quando o objeto tem esse método, 0 quando não tem",
            "example": 'o := {a: 1}\nFileAppend HasMethod(o, "a"), "*"',
        },
    },
    "HasProp": {
        "en": {
            "summary": "Whether an object has a property.",
            "params": "the object, and the name of the property",
            "returns": "1 when the object has such a property, 0 when it has not",
            "errors": "nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.",
            "example": 'o := {a: 1}\nFileAppend HasProp(o, "a"), "*"',
        },
        "zh": {
            "summary": "某个对象是否有某个属性。",
            "returns": "对象有该属性时为 1，没有时为 0",
            "example": 'o := {a: 1}\nFileAppend HasProp(o, "a"), "*"',
        },
        "pt-BR": {
            "summary": "Se um objeto tem uma propriedade.",
            "returns": "1 quando o objeto tem essa propriedade, 0 quando não tem",
            "example": 'o := {a: 1}\nFileAppend HasProp(o, "a"), "*"',
        },
    },
    "GetMethod": {
        "en": {
            "summary": "The method of an object, as a value that can be called.",
            "params": "the object, the name of the method, and the number of parameters it would take",
            "returns": "the method, as a value of the class `Func`",
            "errors": "a method that is not there is refused with the type of the value and the name of the member.",
            "example": 'o := {a: 1}\nFileAppend GetMethod(o, "HasOwnProp").__Class, "*"',
        },
        "zh": {
            "summary": "对象的某个方法，作为一个可以调用的值。",
            "returns": "该方法，作为类为 `Func` 的值",
            "example": 'o := {a: 1}\nFileAppend GetMethod(o, "HasOwnProp").__Class, "*"',
        },
        "pt-BR": {
            "summary": "Um método de um objeto, como um valor que pode ser chamado.",
            "returns": "o método, como um valor da classe `Func`",
            "example": 'o := {a: 1}\nFileAppend GetMethod(o, "HasOwnProp").__Class, "*"',
        },
    },
    "IsSet": {
        "en": {
            "summary": "Whether the variable of the caller has a value. The tree carries it besides `IsSetRef`, and it asks for the variable and not for its value.",
            "params": "the variable, not its value",
            "returns": "1 when the variable has a value, 0 when it has none",
            "errors": "an argument that is not a variable is refused with \"IsSet requires a variable.\".",
            "example": 'x := 1\nFileAppend IsSet(x), "*"',
        },
        "zh": {
            "summary": "调用者的变量是否有值。这个树除 `IsSetRef` 之外也带着它，它要的是变量，而不是变量的值。",
            "returns": "变量有值时为 1，没有时为 0",
            "example": 'x := 1\nFileAppend IsSet(x), "*"',
        },
        "pt-BR": {
            "summary": "Se a variável de quem chama tem valor. A árvore carrega este além de `IsSetRef`, e ele pede a variável e não o valor dela.",
            "returns": "1 quando a variável tem valor, 0 quando não tem",
            "example": 'x := 1\nFileAppend IsSet(x), "*"',
        },
    },
}
