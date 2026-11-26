"""A imagem do CD do laboratorio: um ISO9660 com os arquivos do run.

O host do laboratorio nao tem `mkisofs`, e o disco do XP e pequeno
demais para o binario da arvore. O CD e a via que sobra: o hypervisor
troca a midia do D: pela imagem, e o convidado copia os arquivos do
disco para o disco dele.

O ISO aqui e o ISO9660 simples, sem Joliet e sem Rock Ridge: os nomes
dos arquivos sao de 8.3, e o Windows XP os le sem dificuldade.
"""

import struct
import sys

SECTOR = 2048
NULO = bytes([0])


def both_endian16(value):
    return struct.pack("<H", value) + struct.pack(">H", value)


def both_endian32(value):
    return struct.pack("<I", value) + struct.pack(">I", value)


def the_date():
    """A data de agora, no formato do ISO: 7 bytes."""
    import datetime

    agora = datetime.datetime.now()
    return struct.pack(
        "<BBBBBBB",
        agora.year - 1900,
        agora.month,
        agora.day,
        agora.hour,
        agora.minute,
        agora.second,
        0,
    )


def the_record(identificador, extent, length, is_directory):
    """O registro de um arquivo ou de uma pasta."""
    preenchido = identificador + (NULO if len(identificador) % 2 == 0 else b"")
    flags = 0x02 if is_directory else 0x00
    return (
        struct.pack("<B", 33 + len(preenchido))
        + NULO
        + both_endian32(extent)
        + both_endian32(length)
        + the_date()
        + struct.pack("<BB", flags, 0)
        + NULO
        + both_endian16(1)
        + struct.pack("<B", len(identificador))
        + identificador
        + (NULO if len(identificador) % 2 == 0 else b"")
    )


def build(path, arquivos):
    """Monta a imagem ISO9660 com os arquivos dados.

    `arquivos` e uma lista de pares: o nome de 8.3 e o conteudo.
    """
    conteudos = []
    for nome, dados in arquivos:
        if len(dados) % SECTOR:
            dados = dados + NULO * (SECTOR - len(dados) % SECTOR)
        conteudos.append((nome, dados, len(dados)))

    # O comeco: 16 setores de sistema, o descritor primario, o fim, as
    # duas tabelas de caminho, e a pasta raiz.
    primeiro = 16
    pvd = primeiro
    fim = primeiro + 1
    tabela_l = primeiro + 2
    tabela_m = primeiro + 3
    raiz = primeiro + 4
    primeiro_arquivo = primeiro + 5

    # O tamanho das tabelas: a raiz e uma pasta so.
    caminho_l = struct.pack("<B", 1) + NULO + struct.pack("<I", raiz) + struct.pack("<H", 1)
    if len(caminho_l) % 2:
        caminho_l += NULO
    caminho_m = struct.pack(">B", 1) + NULO + struct.pack(">I", raiz) + struct.pack(">H", 1)
    if len(caminho_m) % 2:
        caminho_m += NULO

    # Onde cada arquivo mora.
    lugares = []
    setor = primeiro_arquivo
    for nome, dados, tamanho in conteudos:
        lugares.append((nome, setor, tamanho, dados))
        setor += len(dados) // SECTOR

    total = setor

    # Os registros da pasta raiz: o ponto e o ponto duplo vem
    # primeiro, e a pasta raiz e a propria mae de si.
    registros = the_record(NULO, raiz, SECTOR, True)
    registros += the_record(bytes([1]), raiz, SECTOR, True)

    # Os registros dos arquivos.
    for nome, lugar, tamanho, dados in lugares:
        registros += the_record(nome.upper().encode("ascii"), lugar, tamanho, False)

    # O registro da propria raiz.
    registros += the_record(NULO, raiz, SECTOR, True)

    # O descritor primario do volume.
    pvd_bytes = bytearray(SECTOR)
    pvd_bytes[0] = 1
    pvd_bytes[1:6] = b"CD001"
    pvd_bytes[6] = 1
    pvd_bytes[8:40] = b"AHK LAB".ljust(32, b" ")
    pvd_bytes[40:72] = b"AHKLAB".ljust(32, b" ")
    pvd_bytes[80:88] = both_endian32(total)
    pvd_bytes[120:124] = both_endian16(1)
    pvd_bytes[124:128] = both_endian16(1)
    pvd_bytes[128:132] = both_endian16(SECTOR)
    pvd_bytes[132:140] = both_endian32(len(caminho_l))
    pvd_bytes[140:144] = struct.pack("<I", tabela_l)
    pvd_bytes[148:152] = struct.pack(">I", tabela_m)
    # O registro da pasta raiz vive no descritor tambem.
    pvd_bytes[156:156 + 34] = registros[-34:]
    pvd_bytes[881] = 1

    # O fim da cadeia.
    fim_bytes = bytearray(SECTOR)
    fim_bytes[0] = 0xFF
    fim_bytes[1:6] = b"CD001"
    fim_bytes[6] = 1

    imagem = bytearray(total * SECTOR)
    imagem[pvd * SECTOR:(pvd + 1) * SECTOR] = pvd_bytes
    imagem[fim * SECTOR:(fim + 1) * SECTOR] = fim_bytes
    imagem[tabela_l * SECTOR:tabela_l * SECTOR + len(caminho_l)] = caminho_l
    imagem[tabela_m * SECTOR:tabela_m * SECTOR + len(caminho_m)] = caminho_m
    imagem[raiz * SECTOR:raiz * SECTOR + len(registros)] = registros
    for nome, lugar, tamanho, dados in lugares:
        imagem[lugar * SECTOR:lugar * SECTOR + len(dados)] = dados

    with open(path, "wb") as mao:
        mao.write(imagem)
    return path


def main():
    if len(sys.argv) < 3:
        raise SystemExit(__doc__)
    saida = sys.argv[1]
    arquivos = []
    for caminho in sys.argv[2:]:
        with open(caminho, "rb") as mao:
            arquivos.append((caminho.split("\\")[-1].split("/")[-1], mao.read()))
    build(saida, arquivos)
    print(f"a imagem {saida} tem {len(arquivos)} arquivo(s)")


if __name__ == "__main__":
    main()
