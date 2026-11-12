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
LANGS = (
    "en", "es", "pt-BR", "fr", "it", "pl", "tr",
    "de", "ru", "ja", "zh", "zh-Hant", "ko",
)

# The name of each language, in that language.
LANG_NAMES = {
    "en": "English",
    "es": "Español",
    "pt-BR": "Português (Brasil)",
    "fr": "Français",
    "it": "Italiano",
    "pl": "Polski",
    "tr": "Türkçe",
    "de": "Deutsch",
    "ru": "Русский",
    "ja": "日本語",
    "zh": "中文（简体）",
    "zh-Hant": "繁體中文",
    "ko": "한국어",
}

# The dictionary of each language. The keys are the same in all of them.
TEXT = {}

TEXT["en"] = {
    "languages": "Language",
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
    "head_probe": "Probe",
    "probe_note": "The probe is the call of the name against a desktop that does nothing: it says whether the call answers or raises.",

    "section_behavior": "What the tree does with it",
    "head_returns": "Answers",
    "head_errors": "Errors",
    "behavior_only_english": "The behavior of this family is written in English, Chinese and Portuguese of Brazil; read it in",
    "unclassified_intro": (
        "The names that no family claims. A name lands here when no table "
        "and no rule of `scripts/surface_areas.py` places it."
    ),
}

TEXT["es"] = {
    "languages": "Idioma",
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
    "head_probe": "Sonda",
    "probe_note": "La sonda es la llamada del nombre contra un escritorio que no hace nada: dice si la llamada responde o lanza.",

    "section_behavior": "Lo que hace el arbol con el",
    "head_returns": "Responde",
    "head_errors": "Errores",
    "behavior_only_english": "El comportamiento de esta familia esta escrito en ingles, chino y portugues de Brasil; lealo en",
    "unclassified_intro": (
        "Los nombres que ninguna familia reclama. Un nombre llega aquí "
        "cuando ninguna tabla y ninguna regla de "
        "`scripts/surface_areas.py` lo coloca."
    ),
}

TEXT["pt-BR"] = {
    "languages": "Idioma",
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
    "head_probe": "Sonda",
    "probe_note": "A sonda é a chamada do nome contra uma área de trabalho que não faz nada: diz se a chamada responde ou levanta.",

    "section_behavior": "O que a arvore faz com ele",
    "head_returns": "Responde",
    "head_errors": "Erros",
    "behavior_only_english": "O comportamento desta familia esta escrito em ingles, chines e portugues do Brasil; leia em",
    "unclassified_intro": (
        "Os nomes que nenhuma família reivindica. Um nome chega aqui quando "
        "nenhuma tabela e nenhuma regra de "
        "`scripts/surface_areas.py` o coloca."
    ),
}

TEXT["fr"] = {
    "languages": "Langue",
    "index_title": "L'index de tous les noms",
    "index_intro": (
        "Tous les noms des trois dialectes, par ordre alphabétique, avec la "
        "famille à laquelle ils appartiennent et ce que cet arbre répond à "
        "leur sujet. La **fiche** d'un nom -- son genre, sa classe, ses "
        "paramètres -- est dans la page de sa famille ; cette page en est "
        "l'entrée."
    ),
    "crosswalk_title": "La table de correspondance des trois dialectes",
    "crosswalk_intro": (
        "Le même nom dans les trois dialectes, et le nom que seul l'un d'eux "
        "possède. Un nom du v1 que le v2 écrit autrement figure dans la "
        "table « v1 seulement », avec le nom du v2 qui l'a remplacé."
    ),
    "dialect_title": "La surface de %s",
    "dialect_intro": (
        "Les noms de %s, comptés par famille et par genre, et les noms que "
        "cet arbre refuse ou ne porte pas. La fiche de chaque nom est dans "
        "la page de sa famille."
    ),
    "family_intro": (
        "Les noms de la famille **%s** dans les trois dialectes, avec ce que "
        "cet arbre répond pour chacun d'eux. La famille est le domaine dans "
        "lequel un script pense ; les noms en sont la totalité."
    ),
    "head_name": "Nom",
    "head_kind": "Genre",
    "head_class": "Classe",
    "head_parameters": "Paramètres",
    "head_state": "État",
    "head_family": "Famille",
    "head_dialect": "Dialecte",
    "head_value": "Valeur",
    "head_refused": "Refusés",
    "head_pending": "En attente",
    "head_reason": "Motif",
    "head_names": "Noms",
    "state_value": "valeur",
    "state_pending": "en attente",
    "state_not_measured": "non mesuré",
    "state_refused": "refusé : %s",
    "section_in_both": "Dans le v1 et dans le v2",
    "section_only_v1": "Seulement dans le v1",
    "section_only_ks": "Seulement dans KeySharp",
    "section_by_kind": "Par genre",
    "section_refused": "Refusés",
    "section_pending": "En attente",
    "the_three": "Les trois",
    "unclassified_title": "Non classé",
    "head_probe": "Sonde",
    "probe_note": "La sonde est l'appel du nom contre un bureau qui ne fait rien : elle dit si l'appel répond ou lève.",

    "section_behavior": "Ce que l'arbre en fait",
    "head_returns": "Répond",
    "head_errors": "Erreurs",
    "behavior_only_english": "Le comportement de cette famille est écrit en anglais, en chinois et en portugais du Brésil ; lisez-le en",
    "unclassified_intro": (
        "Les noms qu'aucune famille ne revendique. Un nom arrive ici quand "
        "ni une table ni une règle de `scripts/surface_areas.py` ne le "
        "place."
    ),
}

TEXT["it"] = {
    "languages": "Lingua",
    "index_title": "L'indice di tutti i nomi",
    "index_intro": (
        "Tutti i nomi dei tre dialetti, in ordine alfabetico, con la "
        "famiglia a cui appartengono e ciò che questo albero risponde loro. "
        "La **scheda** di un nome -- il suo genere, la sua classe, i suoi "
        "parametri -- si trova nella pagina della sua famiglia; questa pagina "
        "ne è l'ingresso."
    ),
    "crosswalk_title": "La tabella di corrispondenza dei tre dialetti",
    "crosswalk_intro": (
        "Lo stesso nome nei tre dialetti, e il nome che soltanto uno di essi "
        "possiede. Un nome del v1 che il v2 scrive in altro modo compare "
        "nella tabella «solo v1», con il nome del v2 che lo ha sostituito."
    ),
    "dialect_title": "La superficie di %s",
    "dialect_intro": (
        "I nomi di %s, contati per famiglia e per genere, e i nomi che questo "
        "albero rifiuta o non porta. La scheda di ogni nome si trova nella "
        "pagina della sua famiglia."
    ),
    "family_intro": (
        "I nomi della famiglia **%s** nei tre dialetti, con ciò che questo "
        "albero risponde per ciascuno di essi. La famiglia è l'ambito in cui "
        "uno script pensa; i nomi ne sono la totalità."
    ),
    "head_name": "Nome",
    "head_kind": "Genere",
    "head_class": "Classe",
    "head_parameters": "Parametri",
    "head_state": "Stato",
    "head_family": "Famiglia",
    "head_dialect": "Dialetto",
    "head_value": "Valore",
    "head_refused": "Rifiutati",
    "head_pending": "In sospeso",
    "head_reason": "Motivo",
    "head_names": "Nomi",
    "state_value": "valore",
    "state_pending": "in sospeso",
    "state_not_measured": "non misurato",
    "state_refused": "rifiutato: %s",
    "section_in_both": "Nel v1 e nel v2",
    "section_only_v1": "Solo nel v1",
    "section_only_ks": "Solo in KeySharp",
    "section_by_kind": "Per genere",
    "section_refused": "Rifiutati",
    "section_pending": "In sospeso",
    "the_three": "I tre",
    "unclassified_title": "Non classificati",
    "head_probe": "Sonda",
    "probe_note": "La sonda è la chiamata del nome contro un desktop che non fa nulla: dice se la chiamata risponde o solleva.",
    "section_behavior": "Cosa fa l'albero con esso",
    "head_returns": "Risponde",
    "head_errors": "Errori",
    "behavior_only_english": (
        "Il comportamento di questa famiglia è scritto in inglese, in cinese "
        "e in portoghese brasiliano; leggetelo in"
    ),
    "unclassified_intro": (
        "I nomi che nessuna famiglia rivendica. Un nome arriva qui quando "
        "nessuna tabella e nessuna regola di `scripts/surface_areas.py` lo "
        "colloca."
    ),
}

TEXT["pl"] = {
    "languages": "Język",
    "index_title": "Indeks wszystkich nazw",
    "index_intro": (
        "Wszystkie nazwy trzech dialektów, w porządku alfabetycznym, wraz z "
        "rodziną, do której należą, i z tym, co to drzewo im odpowiada. "
        "**Karta** nazwy -- jej rodzajem, klasą i parametrami -- znajduje się "
        "na stronie jej rodziny; ta strona jest wejściem."
    ),
    "crosswalk_title": "Tablica odpowiadających sobie nazw w trzech dialektach",
    "crosswalk_intro": (
        "Ta sama nazwa w trzech dialektach oraz nazwa, którą ma tylko jeden z "
        "nich. Nazwa z v1, którą v2 zapisuje inaczej, figuruje w tabeli "
        "„tylko v1”, wraz z nazwą v2, która ją zastąpiła."
    ),
    "dialect_title": "Powierzchnia %s",
    "dialect_intro": (
        "Nazwy %s, policzone według rodzin i rodzajów, oraz nazwy, których to "
        "drzewo odmawia lub nie zawiera. Karta każdej nazwy znajduje się na "
        "stronie jej rodziny."
    ),
    "family_intro": (
        "Nazwy rodziny **%s** w trzech dialektach wraz z tym, co to drzewo "
        "odpowiada na każdą z nich. Rodzina jest dziedziną, w której myśli "
        "skrypt; nazwy stanowią jej całość."
    ),
    "head_name": "Nazwa",
    "head_kind": "Rodzaj",
    "head_class": "Klasa",
    "head_parameters": "Parametry",
    "head_state": "Stan",
    "head_family": "Rodzina",
    "head_dialect": "Dialekt",
    "head_value": "Wartość",
    "head_refused": "Odrzucone",
    "head_pending": "Oczekujące",
    "head_reason": "Powód",
    "head_names": "Nazwy",
    "state_value": "wartość",
    "state_pending": "oczekujące",
    "state_not_measured": "niezmierzone",
    "state_refused": "odrzucone: %s",
    "section_in_both": "W v1 i w v2",
    "section_only_v1": "Tylko w v1",
    "section_only_ks": "Tylko w KeySharp",
    "section_by_kind": "Według rodzajów",
    "section_refused": "Odrzucone",
    "section_pending": "Oczekujące",
    "the_three": "Trzy",
    "unclassified_title": "Niesklasyfikowane",
    "head_probe": "Sonda",
    "probe_note": "Sonda to wywołanie nazwy na pustym pulpicie: mówi, czy wywołanie odpowiada, czy rzuca.",
    "section_behavior": "Co robi z tym drzewo",
    "head_returns": "Odpowiada",
    "head_errors": "Błędy",
    "behavior_only_english": (
        "Zachowanie tej rodziny jest opisane po angielsku, po chińsku i po "
        "portugalsku brazylijsku; przeczytaj je w"
    ),
    "unclassified_intro": (
        "Nazwy, których żadna rodzina nie przejmuje. Nazwa trafia tutaj, gdy "
        "żadna tabela i żadna reguła z `scripts/surface_areas.py` jej nie "
        "umieszcza."
    ),
}

TEXT["tr"] = {
    "languages": "Dil",
    "index_title": "Tüm adların dizini",
    "index_intro": (
        "Üç lehçedeki bütün adlar, alfabetik sırayla, ait oldukları aile ve "
        "bu ağacın onlara verdiği yanıtla birlikte. Bir adın **fişi** -- "
        "türü, sınıfı, parametreleri -- adın ailesinin sayfasındadır; bu sayfa "
        "giriş kapısıdır."
    ),
    "crosswalk_title": "Üç lehçenin karşılık tablosu",
    "crosswalk_intro": (
        "Üç lehçede aynı ad ve yalnızca birinde bulunan ad. v2'nin farklı "
        "yazdığı bir v1 adı, onun yerini alan v2 adıyla birlikte «yalnızca "
        "v1» tablosunda yer alır."
    ),
    "dialect_title": "%s yüzeyi",
    "dialect_intro": (
        "%s adları, aileye ve türe göre sayılmış olarak ve bu ağacın "
        "reddettiği ya da taşımadığı adlarla birlikte. Her adın fişi, adın "
        "ailesinin sayfasındadır."
    ),
    "family_intro": (
        "Üç lehçedeki **%s** ailesinin adları ve bu ağacın her biri için "
        "verdiği yanıt. Aile, bir betiğin düşündüğü alandır; adlar onun "
        "tamamıdır."
    ),
    "head_name": "Ad",
    "head_kind": "Tür",
    "head_class": "Sınıf",
    "head_parameters": "Parametreler",
    "head_state": "Durum",
    "head_family": "Aile",
    "head_dialect": "Lehçe",
    "head_value": "Değer",
    "head_refused": "Reddedilenler",
    "head_pending": "Bekleyenler",
    "head_reason": "Gerekçe",
    "head_names": "Adlar",
    "state_value": "değer",
    "state_pending": "beklemede",
    "state_not_measured": "ölçülmedi",
    "state_refused": "reddedildi: %s",
    "section_in_both": "v1'de ve v2'de bulunanlar",
    "section_only_v1": "Yalnızca v1'de bulunanlar",
    "section_only_ks": "Yalnızca KeySharp'te bulunanlar",
    "section_by_kind": "Türe göre",
    "section_refused": "Reddedilenler",
    "section_pending": "Bekleyenler",
    "the_three": "Üç lehçe",
    "unclassified_title": "Sınıflandırılmamış",
    "head_probe": "Sonda",
    "probe_note": "Sonda, adın boş bir masaüstünde çağrılmasıdır: çağrının yanıt verip vermediğini söyler.",
    "section_behavior": "Ağaç bununla ne yapar",
    "head_returns": "Yanıt",
    "head_errors": "Hatalar",
    "behavior_only_english": (
        "Bu ailenin davranışı İngilizce, Çince ve Brezilya Portekizcesinde "
        "yazılmıştır; şurada okuyun"
    ),
    "unclassified_intro": (
        "Hiçbir ailenin sahiplenmediği adlar. Bir ad, ne bir tablo ne de "
        "`scripts/surface_areas.py` içindeki bir kural onu yerleştirdiğinde "
        "buraya düşer."
    ),
}

TEXT["de"] = {
    "languages": "Sprache",
    "index_title": "Der Index aller Namen",
    "index_intro": (
        "Alle Namen der drei Dialekte, alphabetisch, mit der Familie, zu der "
        "sie gehören, und damit, was dieser Baum für sie antwortet. Die "
        "**Karte** eines Namens -- seine Art, seine Klasse, seine Parameter "
        "-- steht auf der Seite seiner Familie; diese Seite ist der Einstieg."
    ),
    "crosswalk_title": "Die Zuordnung der drei Dialekte",
    "crosswalk_intro": (
        "Derselbe Name in den drei Dialekten und der Name, den nur einer von "
        "ihnen hat. Ein Name des v1, den der v2 anders schreibt, steht in "
        "der Tabelle „nur v1“, mit dem v2-Namen, der ihn ersetzt hat."
    ),
    "dialect_title": "Die Oberfläche von %s",
    "dialect_intro": (
        "Die Namen von %s, nach Familie und nach Art gezählt, und die Namen, "
        "die dieser Baum verweigert oder nicht mitbringt. Die Karte jedes "
        "Namens steht auf der Seite seiner Familie."
    ),
    "family_intro": (
        "Die Namen der Familie **%s** in den drei Dialekten, mit dem, was "
        "dieser Baum für jeden von ihnen antwortet. Die Familie ist das "
        "Gebiet, in dem ein Skript denkt; die Namen sind ihr Ganzes."
    ),
    "head_name": "Name",
    "head_kind": "Art",
    "head_class": "Klasse",
    "head_parameters": "Parameter",
    "head_state": "Zustand",
    "head_family": "Familie",
    "head_dialect": "Dialekt",
    "head_value": "Wert",
    "head_refused": "Verweigert",
    "head_pending": "Ausstehend",
    "head_reason": "Grund",
    "head_names": "Namen",
    "state_value": "Wert",
    "state_pending": "ausstehend",
    "state_not_measured": "nicht gemessen",
    "state_refused": "verweigert: %s",
    "section_in_both": "Im v1 und im v2",
    "section_only_v1": "Nur im v1",
    "section_only_ks": "Nur in KeySharp",
    "section_by_kind": "Nach Art",
    "section_refused": "Verweigerte",
    "section_pending": "Ausstehende",
    "the_three": "Die drei",
    "unclassified_title": "Nicht eingeordnet",
    "head_probe": "Sonde",
    "probe_note": "Die Sonde ist der Aufruf des Namens gegen einen Desktop, der nichts tut: sie sagt, ob der Aufruf antwortet oder wirft.",

    "section_behavior": "Was der Baum damit macht",
    "head_returns": "Antwortet",
    "head_errors": "Fehler",
    "behavior_only_english": "Das Verhalten dieser Familie ist auf Englisch, Chinesisch und brasilianischem Portugiesisch geschrieben; lesen Sie es auf",
    "unclassified_intro": (
        "Die Namen, die keine Familie beansprucht. Ein Name landet hier, "
        "wenn ihn weder eine Tabelle noch eine Regel aus "
        "`scripts/surface_areas.py` einordnet."
    ),
}

TEXT["ru"] = {
    "languages": "Язык",
    "index_title": "Указатель всех имён",
    "index_intro": (
        "Все имена трёх диалектов по алфавиту, с семейством, к которому они "
        "относятся, и с тем, что это дерево на них отвечает. **Карточка** "
        "имени -- его вид, класс, параметры -- лежит на странице его "
        "семейства; эта страница и есть вход."
    ),
    "crosswalk_title": "Таблица соответствия трёх диалектов",
    "crosswalk_intro": (
        "Одно и то же имя в трёх диалектах и имя, которое есть только у "
        "одного из них. Имя из v1, которое v2 пишет иначе, стоит в таблице "
        "«только v1» вместе с именем v2, пришедшим ему на смену."
    ),
    "dialect_title": "Поверхность %s",
    "dialect_intro": (
        "Имена %s, посчитанные по семействам и по видам, и те имена, которые "
        "это дерево отказывает или не несёт. Карточка каждого имени лежит "
        "на странице его семейства."
    ),
    "family_intro": (
        "Имена семейства **%s** в трёх диалектах и то, что это дерево "
        "отвечает на каждое из них. Семейство -- это область, в которой "
        "думает скрипт; имена составляют её целиком."
    ),
    "head_name": "Имя",
    "head_kind": "Вид",
    "head_class": "Класс",
    "head_parameters": "Параметры",
    "head_state": "Состояние",
    "head_family": "Семейство",
    "head_dialect": "Диалект",
    "head_value": "Значение",
    "head_refused": "Отказано",
    "head_pending": "Ожидается",
    "head_reason": "Причина",
    "head_names": "Имён",
    "state_value": "значение",
    "state_pending": "ожидается",
    "state_not_measured": "не измерено",
    "state_refused": "отказано: %s",
    "section_in_both": "В v1 и в v2",
    "section_only_v1": "Только в v1",
    "section_only_ks": "Только в KeySharp",
    "section_by_kind": "По видам",
    "section_refused": "Отказанные",
    "section_pending": "Ожидаемые",
    "the_three": "Три",
    "unclassified_title": "Без семейства",
    "head_probe": "Проба",
    "probe_note": "Проба — это вызов имени на пустом рабочем столе: она говорит, отвечает вызов или бросает.",

    "section_behavior": "Что дерево делает с ним",
    "head_returns": "Отвечает",
    "head_errors": "Ошибки",
    "behavior_only_english": "Поведение этого семейства написано по-английски, по-китайски и по-португальски (Бразилия); читайте его на",
    "unclassified_intro": (
        "Имена, которые не взяло ни одно семейство. Имя попадает сюда, когда "
        "его не поместил ни один список и ни одно правило из "
        "`scripts/surface_areas.py`."
    ),
}

TEXT["zh-Hant"] = {
    "languages": "語言",
    "index_title": "全部名稱索引",
    "index_intro": (
        "按字母順序列出三個方言的每一個名稱，並標註它所屬的分類，以及這棵"
        "樹對它給出的回答。名稱的**技術卡**（種類、類別、參數）位於其分類"
        "頁面；本頁是入口。"
    ),
    "crosswalk_title": "三個方言的對照表",
    "crosswalk_intro": (
        "同一個名稱在三個方言中的寫法，以及只有其中一個方言才有的名稱。"
        "v1 中的名稱在 v2 中寫法不同時，會列入「僅 v1」表，並給出替代它的 "
        "v2 名稱。"
    ),
    "dialect_title": "%s 的表面",
    "dialect_intro": (
        "按分類與種類統計 %s 的名稱，並列出這棵樹拒絕或尚未攜帶的名稱。"
        "每個名稱的技術卡位於其分類頁面。"
    ),
    "family_intro": (
        "三個方言中屬於 **%s** 類的名稱，以及這棵樹對每個名稱給出的回答。"
        "分類是腳本思考的領域，這些名稱即是它的全部。"
    ),
    "head_name": "名稱",
    "head_kind": "種類",
    "head_class": "類別",
    "head_parameters": "參數",
    "head_state": "狀態",
    "head_family": "分類",
    "head_dialect": "方言",
    "head_value": "值",
    "head_refused": "拒絕",
    "head_pending": "待辦",
    "head_reason": "原因",
    "head_names": "名稱數",
    "state_value": "值",
    "state_pending": "待辦",
    "state_not_measured": "未測量",
    "state_refused": "拒絕：%s",
    "section_in_both": "v1 與 v2 共有",
    "section_only_v1": "僅 v1 獨有",
    "section_only_ks": "僅 KeySharp 獨有",
    "section_by_kind": "按種類",
    "section_refused": "被拒絕的名稱",
    "section_pending": "待辦的名稱",
    "the_three": "三個方言",
    "unclassified_title": "未分類",
    "head_probe": "プローブ",
    "probe_note": "プローブは，何もしないデスクトップに対して名前を呼び出した結果です。呼び出しが答えを返すか例外を投げるかを示します。",

    "section_behavior": "這棵樹對它做什麼",
    "head_returns": "返回",
    "head_errors": "錯誤",
    "behavior_only_english": "這個分類的行為寫在英語、中文和巴西葡萄牙語裡；請在",
    "unclassified_intro": (
        "沒有任何分類認領的名稱。當表格與 `scripts/surface_areas.py` 的規則"
        "都無法歸類時，名稱會落到這裡。"
    ),
}

TEXT["ko"] = {
    "languages": "언어",
    "index_title": "모든 이름의 색인",
    "index_intro": (
        "세 가지 방언의 모든 이름을 가나다순으로, 각 이름이 속한 분류와 이 "
        "트리가 그 이름에 대해 답하는 것과 함께 나열한다. 이름의 **기술 "
        "카드**(종류, 클래스, 매개변수)는 그 분류의 페이지에 있다; 이 "
        "페이지가 입구다."
    ),
    "crosswalk_title": "세 방언의 대응표",
    "crosswalk_intro": (
        "세 방언에서 같은 이름과, 하나에만 있는 이름. v1의 이름이 v2에서 "
        "다르게 쓰이는 것은 'v1 전용' 표에, 그것을 대신한 v2 이름과 함께 "
        "적힌다."
    ),
    "dialect_title": "%s의 표면",
    "dialect_intro": (
        "%s의 이름을 분류별, 종류별로 세고, 이 트리가 거부하거나 아직 싣지 "
        "않은 이름을 함께 보인다. 모든 이름의 기술 카드는 분류 페이지에 "
        "있다."
    ),
    "family_intro": (
        "세 방언에서 분류 **%s**에 속하는 이름들과, 이 트리가 그 각각에 대해 "
        "답하는 것. 분류는 스크립트가 생각하는 영역이며, 그 이름들이 전부다."
    ),
    "head_name": "이름",
    "head_kind": "종류",
    "head_class": "클래스",
    "head_parameters": "매개변수",
    "head_state": "상태",
    "head_family": "분류",
    "head_dialect": "방언",
    "head_value": "값",
    "head_refused": "거부됨",
    "head_pending": "보류",
    "head_reason": "이유",
    "head_names": "이름 수",
    "state_value": "값",
    "state_pending": "보류",
    "state_not_measured": "측정되지 않음",
    "state_refused": "거부됨: %s",
    "section_in_both": "v1과 v2에 함께 있는 것",
    "section_only_v1": "v1에만 있는 것",
    "section_only_ks": "KeySharp에만 있는 것",
    "section_by_kind": "종류별",
    "section_refused": "거부된 이름",
    "section_pending": "보류된 이름",
    "the_three": "세 방언",
    "unclassified_title": "미분류",
    "head_probe": "프로브",
    "probe_note": "프로브는 아무것도 하지 않는 데스크톱에 그 이름을 호출한 결과다. 호출이 답을 돌려주는지 예외를 던지는지를 보여준다.",

    "section_behavior": "이 트리가 이름에 대해 하는 일",
    "head_returns": "반환",
    "head_errors": "오류",
    "behavior_only_english": "이 분류의 동작은 영어, 중국어, 브라질 포르투갈어로 쓰여 있다; 영어로 읽으세요",
    "unclassified_intro": (
        "어떤 분류도 차지하지 않는 이름. 목록도 `scripts/surface_areas.py`의 "
        "규칙도 그 이름을 놓치지 못하면 여기에 닿는다."
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
    "head_probe": "プローブ",
    "probe_note": "プローブは，何もしないデスクトップに対して名前を呼び出した結果です。呼び出しが答えを返すか例外を投げるかを示します。",
    "section_behavior": "このツリーがその名前で行うこと",
    "head_returns": "返答",
    "head_errors": "エラー",
    "behavior_only_english": "このファミリーのふるまいは英語・中国語・ブラジルポルトガル語で書いてある。英語版で読む",
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
    "head_probe": "调用探测",
    "probe_note": "调用探测是在一个什么都不做的桌面上调用该名称的结果：它说明调用是返回答案还是抛出异常。",

    "section_behavior": "这棵树对它做什么",
    "head_returns": "返回",
    "head_errors": "错误",
    "behavior_only_english": "这个分类的行为写在英语、中文和巴西葡萄牙语里；请在",
    "unclassified_intro": (
        "没有任何分类认领的名称。当表格与 "
        "`scripts/surface_areas.py` 的规则都无法归类时，名称会落到这里。"
    ),
}


# The title of a family, in each language. The key of the family stays in
# English: it is the name of the page and of the file.
FAMILY_TITLES = {
    "en": {
        "unclassified": "Unclassified",
        "network": "The network and the mail",
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
        "unclassified": "Sin clasificar",
        "network": "La red y el correo",
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
        "unclassified": "Sem classificação",
        "network": "A rede e o correio",
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
    "fr": {
        "unclassified": "Non classé",
        "script": "L'analyseur et l'exécution du script",
        "files": "Les opérations sur les fichiers et les répertoires",
        "input": "Raccourcis, chaînes de déclenchement et saisie",
        "gui": "Fenêtres d'interface et automatisation des fenêtres",
        "screen": "Capture d'écran et fonctions de pixel",
        "native": "Appels de fonctions natives (DllCall)",
        "registry": "Les API du registre",
        "com": "L'automatisation COM (ComObject)",
        "core": "Le cœur du langage : texte, mathématiques et motifs",
        "objects": "Les objets du langage : Array, Map et Func",
        "time": "Les dates et les heures",
        "machine": "La machine, les processus et l'environnement",
        "clipboard": "Le presse-papiers",
        "sound": "Le son",
        "network": "Le réseau et le courrier",
    },
    "it": {
        "unclassified": "Non classificati",
        "script": "L'analizzatore e l'esecuzione dello script",
        "files": "Operazioni su file e directory",
        "input": "Tasti rapidi, stringhe di attivazione e input",
        "gui": "Finestre GUI e automazione delle finestre",
        "screen": "Cattura dello schermo e funzioni sui pixel",
        "native": "Chiamate a funzioni native (DllCall)",
        "registry": "Le API del registro",
        "com": "Automazione COM (ComObject)",
        "core": "Il nucleo del linguaggio: testo, matematica e modelli",
        "objects": "Gli oggetti del linguaggio: Array, Map e Func",
        "time": "Date e ore",
        "machine": "La macchina, i processi e l'ambiente",
        "clipboard": "Gli appunti",
        "sound": "Il suono",
        "network": "La rete e la posta",
    },
    "pl": {
        "unclassified": "Niesklasyfikowane",
        "script": "Parser i wykonanie skryptu",
        "files": "Operacje na plikach i katalogach",
        "input": "Skróty, ciągi wyzwalające i wprowadzanie",
        "gui": "Okna GUI i automatyzacja okien",
        "screen": "Przechwytywanie ekranu i funkcje pikseli",
        "native": "Wywołania funkcji natywnych (DllCall)",
        "registry": "Interfejsy API rejestru",
        "com": "Automatyzacja COM (ComObject)",
        "core": "Rdzeń języka: tekst, matematyka i wzorce",
        "objects": "Obiekty języka: Array, Map i Func",
        "time": "Daty i godziny",
        "machine": "Maszyna, procesy i środowisko",
        "clipboard": "Schowek",
        "sound": "Dźwięk",
        "network": "Sieć i poczta",
    },
    "tr": {
        "unclassified": "Sınıflandırılmamış",
        "script": "Ayrıştırıcı ve betiğin çalıştırılması",
        "files": "Dosya ve dizin işlemleri",
        "input": "Kısayollar, tetikleme dizeleri ve giriş",
        "gui": "GUI pencereleri ve pencere otomasyonu",
        "screen": "Ekran yakalama ve piksel işlevleri",
        "native": "Yerel işlev çağrıları (DllCall)",
        "registry": "Kayıt defteri API'leri",
        "com": "COM otomasyonu (ComObject)",
        "core": "Dilin çekirdeği: metin, matematik ve kalıplar",
        "objects": "Dilin nesneleri: Array, Map ve Func",
        "time": "Tarihler ve saatler",
        "machine": "Makine, süreçler ve ortam",
        "clipboard": "Pano",
        "sound": "Ses",
        "network": "Ağ ve posta",
    },
    "de": {
        "unclassified": "Nicht eingeordnet",
        "script": "Der Parser und die Ausführung des Skripts",
        "files": "Datei- und Verzeichnisoperationen",
        "input": "Hotkeys, Hotstrings und Eingabe",
        "gui": "GUI-Fenster und Fensterautomatisierung",
        "screen": "Bildschirmaufnahme und Pixel-Funktionen",
        "native": "Aufrufe nativer Funktionen (DllCall)",
        "registry": "Die Registry-APIs",
        "com": "COM-Automatisierung (ComObject)",
        "core": "Der Kern der Sprache: Text, Mathematik und Muster",
        "objects": "Die Objekte der Sprache: Array, Map und Func",
        "time": "Datums- und Zeitangaben",
        "machine": "Die Maschine, die Prozesse und die Umgebung",
        "clipboard": "Die Zwischenablage",
        "sound": "Der Klang",
        "network": "Das Netz und die Post",
    },
    "ru": {
        "unclassified": "Без семейства",
        "script": "Разбор и выполнение скрипта",
        "files": "Операции с файлами и каталогами",
        "input": "Горячие клавиши, горячие строки и ввод",
        "gui": "Окна интерфейса и автоматизация окон",
        "screen": "Захват экрана и пиксельные функции",
        "native": "Вызовы нативных функций (DllCall)",
        "registry": "API реестра",
        "com": "COM-автоматизация (ComObject)",
        "core": "Ядро языка: текст, математика и шаблоны",
        "objects": "Объекты языка: Array, Map и Func",
        "time": "Даты и время",
        "machine": "Машина, процессы и окружение",
        "clipboard": "Буфер обмена",
        "sound": "Звук",
        "network": "Сеть и почта",
    },
    "zh-Hant": {
        "unclassified": "未分類",
        "script": "解析器與腳本執行",
        "files": "檔案與目錄操作",
        "input": "熱鍵、熱字串與輸入",
        "gui": "GUI 視窗與視窗自動化",
        "screen": "螢幕擷取與像素函式",
        "native": "原生函式呼叫 (DllCall)",
        "registry": "登錄檔 API",
        "com": "COM 自動化 (ComObject)",
        "core": "語言核心：文字、數學與模式",
        "objects": "語言的物件：Array、Map 與 Func",
        "time": "日期與時間",
        "machine": "機器、行程與環境",
        "clipboard": "剪貼簿",
        "sound": "聲音",
        "network": "網路與郵件",
    },
    "ko": {
        "unclassified": "미분류",
        "script": "파서와 스크립트 실행",
        "files": "파일과 디렉터리 작업",
        "input": "핫키, 핫스트링, 입력",
        "gui": "GUI 창과 창 자동화",
        "screen": "화면 캡처와 픽셀 함수",
        "native": "네이티브 함수 호출 (DllCall)",
        "registry": "레지스트리 API",
        "com": "COM 자동화 (ComObject)",
        "core": "언어의 핵심: 텍스트, 수학, 패턴",
        "objects": "언어의 객체: Array, Map, Func",
        "time": "날짜와 시간",
        "machine": "머신, 프로세스, 환경",
        "clipboard": "클립보드",
        "sound": "소리",
        "network": "네트워크와 우편",
    },
    "ja": {
        "unclassified": "未分類",
        "network": "ネットワークとメール",
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
        "unclassified": "未分类",
        "network": "网络与邮件",
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
REASONS = {    "this is a name of Windows, and this desktop is not Windows.": {
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


# The reason of a refusal in the languages that came later: the table above
# is the one the tree was measured with, and this one carries the same
# reasons in French, German, Russian, Traditional Chinese, Korean, Italian,
# Polish and Turkish.
REASONS_MORE = {
    "this is a name of Windows, and this desktop is not Windows.": {
        "fr": "c'est un nom de Windows, et ce bureau n'est pas Windows.",
        "de": "ist ein Name von Windows, und dieser Desktop ist nicht Windows.",
        "ru": "это имя Windows, а этот рабочий стол — не Windows.",
        "zh-Hant": "這是 Windows 的名稱，而本桌面不是 Windows。",
        "ko": "이것은 Windows의 이름이고, 이 데스크톱은 Windows가 아니다.",
        "it": "e un nome di Windows, e questo desktop non è Windows.",
        "pl": "to jest nazwa systemu Windows, a ten pulpit nie jest systemem Windows.",
        "tr": "bu, Windows'un bir adıdır ve bu masaüstü Windows değildir.",
    },
    "the registry is a thing of Windows, and this runtime does not carry it.": {
        "fr": "le registre est une chose de Windows, et ce runtime ne le porte pas.",
        "de": "die Registry ist eine Sache von Windows, und diese Laufzeitumgebung bringt sie nicht mit.",
        "ru": "реестр — это дело Windows, а эта среда выполнения его не несёт.",
        "zh-Hant": "登錄檔屬於 Windows，此執行環境並不攜帶它。",
        "ko": "레지스트리는 Windows의 것이고, 이 런타임은 그것을 싣지 않는다.",
        "it": "il registro è una cosa di Windows, e questo runtime non lo porta.",
        "pl": "rejestr jest sprawą systemu Windows, a to środowisko wykonawcze go nie zawiera.",
        "tr": "kayıt defteri Windows'a ait bir şeydir ve bu çalışma zamanı onu taşımaz.",
    },
    "this runtime has no tray icon.": {
        "fr": "ce runtime n'a pas d'icône dans la zone de notification.",
        "de": "dieser Laufzeitumgebung fehlt das Symbol im Infobereich.",
        "ru": "у этой среды выполнения нет значка в области уведомлений.",
        "zh-Hant": "此執行環境沒有系統匣圖示。",
        "ko": "이 런타임에는 트레이 아이콘이 없다.",
        "it": "questo runtime non ha icona nella barra di notifica.",
        "pl": "to środowisko wykonawcze nie ma ikony w obszarze powiadomień.",
        "tr": "bu çalışma zamanının sistem tepsisinde simgesi yoktur.",
    },
    "this runtime runs one script in one thread, and it cannot make it critical.": {
        "fr": "ce runtime exécute un script dans un seul fil, et il ne peut pas le rendre critique.",
        "de": "diese Laufzeitumgebung führt ein Skript in einem einzigen Thread aus und kann es nicht kritisch machen.",
        "ru": "эта среда выполняет один скрипт в одном потоке и не может сделать его критическим.",
        "zh-Hant": "此執行環境在一個執行緒中執行一個腳本，無法將其設為關鍵。",
        "ko": "이 런타임은 하나의 스크립트를 하나의 스레드로 실행하며, 그것을 임계로 만들 수 없다.",
        "it": "questo runtime esegue uno script in un solo thread e non può renderlo critico.",
        "pl": "to środowisko wykonawcze wykonuje jeden skrypt w jednym wątku i nie może nadać mu znaczenia krytycznego.",
        "tr": "bu çalışma zamanı betiği tek bir iş parçacığında çalıştırır ve onu kritik yapamaz.",
    },
    "the lines are listed in a window of v2, and this runtime has no window to show them in.": {
        "fr": "les lignes sont listées dans une fenêtre de v2, et ce runtime n'a pas de fenêtre pour les montrer.",
        "de": "die Zeilen werden in einem Fenster des v2 aufgeführt, und diese Laufzeitumgebung hat kein Fenster dafür.",
        "ru": "строки выводятся в окне v2, а у этой среды выполнения нет окна.",
        "zh-Hant": "行列表顯示在 v2 的視窗中，而此執行環境沒有視窗可顯示。",
        "ko": "줄 목록은 v2의 창에 표시되며, 이 런타임에는 그것을 보여 줄 창이 없다.",
        "it": "le righe sono elencate in una finestra di v2, e questo runtime non ha una finestra in cui mostrarle.",
        "pl": "linie są wymieniane w oknie v2, a to środowisko wykonawcze nie ma okna, w którym można je pokazać.",
        "tr": "satırlar v2 penceresinde listelenir ve bu çalışma zamanında onları gösterecek bir pencere yoktur.",
    },
    "the variables are listed in a window of v2, and this runtime has no window to show them in.": {
        "fr": "les variables sont listées dans une fenêtre de v2, et ce runtime n'a pas de fenêtre pour les montrer.",
        "de": "die Variablen werden in einem Fenster des v2 aufgeführt, und diese Laufzeitumgebung hat kein Fenster dafür.",
        "ru": "переменные выводятся в окне v2, а у этой среды выполнения нет окна.",
        "zh-Hant": "變數列表顯示在 v2 的視窗中，而此執行環境沒有視窗可顯示。",
        "ko": "변수 목록은 v2의 창에 표시되며, 이 런타임에는 그것을 보여 줄 창이 없다.",
        "it": "le variabili sono elencate in una finestra di v2, e questo runtime non ha una finestra in cui mostrarle.",
        "pl": "zmienne są wymieniane w oknie v2, a to środowisko wykonawcze nie ma okna, w którym można je pokazać.",
        "tr": "değişkenler v2 penceresinde listelenir ve bu çalışma zamanında onları gösterecek bir pencere yoktur.",
    },
    "this runtime is a script that ends when it ends, and it cannot be kept alive.": {
        "fr": "ce runtime est un script qui finit quand il finit, et il ne peut pas rester vivant.",
        "de": "diese Laufzeitumgebung ist ein Skript, das endet, wenn es endet, und das nicht am Leben gehalten werden kann.",
        "ru": "эта среда — скрипт, который заканчивается, когда заканчивается, и его нельзя удержать живым.",
        "zh-Hant": "此執行環境是執行結束即結束的腳本，無法保持存活。",
        "ko": "이 런타임은 끝나면 끝나는 스크립트라 살아 있게 둘 수 없다.",
        "it": "questo runtime è uno script che finisce quando finisce, e non può restare vivo.",
        "pl": "to środowisko wykonawcze jest skryptem, który kończy się, gdy się kończy, i nie można go utrzymać przy życiu.",
        "tr": "bu çalışma zamanı, bittiğinde biten bir betiktir ve canlı tutulamaz.",
    },
    "this runtime runs one script once, and it cannot run itself again.": {
        "fr": "ce runtime exécute un script une fois, et il ne peut pas s'exécuter encore.",
        "de": "diese Laufzeitumgebung führt ein Skript einmal aus und kann sich nicht selbst erneut ausführen.",
        "ru": "эта среда выполняет скрипт один раз и не может запустить себя снова.",
        "zh-Hant": "此執行環境只執行一次腳本，無法再次執行自身。",
        "ko": "이 런타임은 스크립트를 한 번만 실행하며, 자신을 다시 실행할 수 없다.",
        "it": "questo runtime esegue uno script una sola volta e non può eseguire se stesso di nuovo.",
        "pl": "to środowisko wykonawcze wykonuje jeden skrypt raz i nie może uruchomić się ponownie.",
        "tr": "bu çalışma zamanı betiği bir kez çalıştırır ve kendini yeniden çalıştıramaz.",
    },
    "this runtime runs one script in one thread, and it cannot start another.": {
        "fr": "ce runtime exécute un script dans un seul fil, et il ne peut pas en lancer un autre.",
        "de": "diese Laufzeitumgebung führt ein Skript in einem einzigen Thread aus und kann keinen weiteren starten.",
        "ru": "эта среда выполняет один скрипт в одном потоке и не может запустить другой.",
        "zh-Hant": "此執行環境在一個執行緒中執行一個腳本，無法再啟動一個。",
        "ko": "이 런타임은 하나의 스크립트를 하나의 스레드로 실행하며, 다른 것을 시작할 수 없다.",
        "it": "questo runtime esegue uno script in un solo thread e non può avviarne un altro.",
        "pl": "to środowisko wykonawcze wykonuje jeden skrypt w jednym wątku i nie może uruchomić innego.",
        "tr": "bu çalışma zamanı betiği tek bir iş parçacığında çalıştırır ve başka bir tane başlatamaz.",
    },
    "this runtime reads no text of a window.": {
        "fr": "ce runtime ne lit pas le texte d'une fenêtre.",
        "de": "diese Laufzeitumgebung liest keinen Text eines Fensters.",
        "ru": "эта среда не читает текст окна.",
        "zh-Hant": "此執行環境不讀取視窗文字。",
        "ko": "이 런타임은 창의 글자를 읽지 않는다.",
        "it": "questo runtime non legge il testo di una finestra.",
        "pl": "to środowisko wykonawcze nie odczytuje tekstu okna.",
        "tr": "bu çalışma zamanı bir pencerenin metnini okumaz.",
    },
    "the shape of the cursor of the machine is not read by this runtime.": {
        "fr": "ce runtime ne lit pas la forme du curseur de la machine.",
        "de": "diese Laufzeitumgebung liest die Form des Mauszeigers der Maschine nicht.",
        "ru": "эта среда не читает форму курсора машины.",
        "zh-Hant": "此執行環境不讀取機器游標的形狀。",
        "ko": "이 런타임은 컴퓨터 커서의 모양을 읽지 않는다.",
        "it": "la forma del cursore della macchina non è letta da questo runtime.",
        "pl": "kształt kursora maszyny nie jest odczytywany przez to środowisko wykonawcze.",
        "tr": "makinenin imleç biçimi bu çalışma zamanı tarafından okunmaz.",
    },
    "this desktop does not know the scale of the output.": {
        "fr": "ce bureau ne connaît pas l'échelle de la sortie.",
        "de": "dieser Desktop kennt den Maßstab der Ausgabe nicht.",
        "ru": "этот рабочий стол не знает масштаба вывода.",
        "zh-Hant": "本桌面不知道輸出的縮放比例。",
        "ko": "이 데스크톱은 출력의 배율을 알지 못한다.",
        "it": "questo desktop non conosce la scala dell'uscita.",
        "pl": "ten pulpit nie zna skali urządzenia wyjściowego.",
        "tr": "bu masaüstü çıktının ölçeğini bilmez.",
    },
    "this runtime has no window of its own: it is a script that runs and ends.": {
        "fr": "ce runtime n'a pas de fenêtre à lui : c'est un script qui s'exécute et finit.",
        "de": "diese Laufzeitumgebung hat kein eigenes Fenster: sie ist ein Skript, das läuft und endet.",
        "ru": "у этой среды выполнения нет собственного окна: это скрипт, который выполняется и заканчивается.",
        "zh-Hant": "此執行環境沒有自己的視窗：它是一個執行即結束的腳本。",
        "ko": "이 런타임에는 자체 창이 없다: 실행되고 끝나는 스크립트다.",
        "it": "questo runtime non ha una finestra propria: è uno script che viene eseguito e finisce.",
        "pl": "to środowisko wykonawcze nie ma własnego okna: jest skryptem, który się uruchamia i kończy.",
        "tr": "bu çalışma zamanının kendi penceresi yoktur: çalışıp biten bir betiktir.",
    },
    "this desktop does not know the address of the network of the machine.": {
        "fr": "ce bureau ne connaît pas l'adresse du réseau de la machine.",
        "de": "dieser Desktop kennt die Adresse des Netzes der Maschine nicht.",
        "ru": "этот рабочий стол не знает сетевого адреса машины.",
        "zh-Hant": "本桌面不知道機器的網路位址。",
        "ko": "이 데스크톱은 컴퓨터의 네트워크 주소를 알지 못한다.",
        "it": "questo desktop non conosce l'indirizzo di rete della macchina.",
        "pl": "ten pulpit nie zna adresu sieci maszyny.",
        "tr": "bu masaüstü makinenin ağ adresini bilmez.",
    },
    "this name is about Windows, and this machine is not.": {
        "fr": "ce nom concerne Windows, et cette machine n'en est pas une.",
        "de": "dieser Name betrifft Windows, und diese Maschine ist keine.",
        "ru": "это имя относится к Windows, а эта машина — не Windows.",
        "zh-Hant": "此名稱與 Windows 有關，而本機不是 Windows。",
        "ko": "이 이름은 Windows에 관한 것이고, 이 컴퓨터는 Windows가 아니다.",
        "it": "questo nome riguarda Windows, e questa macchina non è Windows.",
        "pl": "ta nazwa dotyczy systemu Windows, a ta maszyna nim nie jest.",
        "tr": "bu ad Windows ile ilgilidir ve bu makine Windows değildir.",
    },
}

REASONS.update(REASONS_MORE)


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

