# Sound

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [Français](fr/index.md) | [Italiano](it/index.md) | [Polski](pl/index.md) | [Türkçe](tr/index.md) | [Deutsch](de/index.md) | [Русский](ru/index.md) | [日本語](ja/index.md) | [中文（简体）](zh/index.md) | [繁體中文](zh-Hant/index.md) | [한국어](ko/index.md) |

Language: English

The names of the family **Sound** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State | Probe |
|---|---|---|---|---|---|---|
| `SoundBeep` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | value | -- |
| `SoundBeep` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value | -- |
| `SoundGet` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SoundGetInterface` | AutoHotkey v2.0.19 | function | -- | 1 to 3 | value | -- |
| `SoundGetMute` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | value | -- |
| `SoundGetName` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | value | -- |
| `SoundGetVolume` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | value | -- |
| `SoundGetWaveVolume` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SoundPlay` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value | -- |
| `SoundPlay` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `SoundSet` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SoundSetMute` | AutoHotkey v2.0.19 | function | -- | 1 to 3 | value | -- |
| `SoundSetVolume` | AutoHotkey v2.0.19 | function | -- | 1 to 3 | value | -- |
| `SoundSetWaveVolume` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |

## What the tree does with it

### SoundBeep

Plays a tone; the box has no speaker of its own, so the tone is written as a wave file under /tmp, played with paplay, and removed again.

- **Parameters:** [Frequency, Duration]
- **Answers:** 1
- **Errors:** a frequency outside the range is refused with "the frequency of a tone is between 37 and 32767."; a host without the mixer answers "the mixer refused: ... pactl: not found" and "the mixer could not play ...".

```ahk
SoundBeep 1000, 200
```

### SoundBeep

Plays a tone; the box has no speaker of its own, so the tone is written as a wave file under /tmp, played with paplay, and removed again.

- **Parameters:** [Frequency, Duration]
- **Answers:** 1
- **Errors:** a frequency outside the range is refused with "the frequency of a tone is between 37 and 32767."; a host without the mixer answers "the mixer refused: ... pactl: not found" and "the mixer could not play ...".

```ahk
SoundBeep 1000, 200
```

### SoundPlay

Plays a sound file with the mixer of the session; the volume, the speed and the pitch of v2 are not carried.

- **Parameters:** File, [Options]
- **Answers:** 1
- **Errors:** the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```

### SoundPlay

Plays a sound file with the mixer of the session; the volume, the speed and the pitch of v2 are not carried.

- **Parameters:** File, [Options]
- **Answers:** 1
- **Errors:** the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```
