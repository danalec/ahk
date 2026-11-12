# O som

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Idioma: Português (Brasil)

Os nomes da família **O som** nos três dialetos, com o que esta árvore responde por cada um. A família é a área em que um script pensa; os nomes são a totalidade dela.

| Nome | Dialeto | Tipo | Classe | Parâmetros | Estado | Sonda |
|---|---|---|---|---|---|---|
| `SoundBeep` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | valor | -- |
| `SoundBeep` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | valor | -- |
| `SoundGet` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | recusado: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SoundGetInterface` | AutoHotkey v2.0.19 | function | -- | 1 to 3 | valor | -- |
| `SoundGetMute` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | valor | -- |
| `SoundGetName` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | valor | -- |
| `SoundGetVolume` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | valor | -- |
| `SoundGetWaveVolume` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | recusado: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SoundPlay` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | valor | -- |
| `SoundPlay` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor | -- |
| `SoundSet` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | recusado: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SoundSetMute` | AutoHotkey v2.0.19 | function | -- | 1 to 3 | valor | -- |
| `SoundSetVolume` | AutoHotkey v2.0.19 | function | -- | 1 to 3 | valor | -- |
| `SoundSetWaveVolume` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | recusado: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |

## O que a arvore faz com ele

### SoundBeep

Toca um tom; a caixa não tem alto-falante próprio, então o tom vira um arquivo wave em /tmp, é tocado com paplay e depois removido.

- **Parâmetros:** [Frequency, Duration]
- **Responde:** 1
- **Erros:** a frequency outside the range is refused with "the frequency of a tone is between 37 and 32767."; a host without the mixer answers "the mixer refused: ... pactl: not found" and "the mixer could not play ...".

```ahk
SoundBeep 1000, 200
```

### SoundPlay

Toca um arquivo de som com o mixer da sessão; o volume, a velocidade e o tom do v2 não são carregados.

- **Parâmetros:** File, [Options]
- **Responde:** 1
- **Erros:** the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```
