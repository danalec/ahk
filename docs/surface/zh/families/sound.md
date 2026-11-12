# 声音

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

语言: 中文（简体）

三个方言中属于 **声音** 类的名称，以及这棵树对每个名称给出的回答。分类是脚本思考的领域，这些名称即是它的全部。

| 名称 | 方言 | 种类 | 类 | 参数 | 状态 | 调用探测 |
|---|---|---|---|---|---|---|
| `SoundBeep` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | 值 | -- |
| `SoundBeep` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 值 | -- |
| `SoundGet` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SoundGetInterface` | AutoHotkey v2.0.19 | function | -- | 1 to 3 | 值 | -- |
| `SoundGetMute` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | 值 | -- |
| `SoundGetName` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | 值 | -- |
| `SoundGetVolume` | AutoHotkey v2.0.19 | function | -- | 0 to 2 | 值 | -- |
| `SoundGetWaveVolume` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SoundPlay` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | 值 | -- |
| `SoundPlay` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 值 | -- |
| `SoundSet` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `SoundSetMute` | AutoHotkey v2.0.19 | function | -- | 1 to 3 | 值 | -- |
| `SoundSetVolume` | AutoHotkey v2.0.19 | function | -- | 1 to 3 | 值 | -- |
| `SoundSetWaveVolume` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |

## 这棵树对它做什么

### SoundBeep

播放一个音调；盒子自己没有扬声器，所以音调会写成 /tmp 下的波形文件，用 paplay 播放，然后删掉。

- **参数:** [Frequency, Duration]
- **返回:** 1
- **错误:** a frequency outside the range is refused with "the frequency of a tone is between 37 and 32767."; a host without the mixer answers "the mixer refused: ... pactl: not found" and "the mixer could not play ...".

```ahk
SoundBeep 1000, 200
```

### SoundPlay

用会话的混音器播放一个声音文件；v2 的音量、速度和音高没有被承载。

- **参数:** File, [Options]
- **返回:** 1
- **错误:** the volume, the speed and the pitch are refused with "the volume, the speed and the pitch of a sound are not carried yet."; an empty path is "the sound file is missing."; a host without the mixer answers the reason of the player.

```ahk
SoundPlay "/usr/share/sounds/freedesktop/stereo/complete.oga"
```
