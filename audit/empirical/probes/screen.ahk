; Probe: the screen and the pictures of v2. The pixel is read, not written;
; nothing of the screen changes.
acc := ""

try {
    r := PixelGetColor(0, 0)
    acc := acc "PixelGetColor=OK|[" r "]`n"
} catch as e {
    acc := acc "PixelGetColor=ERR|" e.Message "`n"
}
try {
    r := PixelSearch(&x, &y, 0, 0, 20, 20, "0x000000")
    acc := acc "PixelSearch=OK|[" r "]|x:" x "|y:" y "`n"
} catch as e {
    acc := acc "PixelSearch=ERR|" e.Message "`n"
}
try {
    r := ImageSearch(&ix, &iy, 0, 0, 100, 100, "C:\Windows\win.ini")
    acc := acc "ImageSearch=OK|[" r "]`n"
} catch as e {
    acc := acc "ImageSearch=ERR|" e.Message "`n"
}
try {
    r := LoadPicture("C:\ahk-no-such-picture-zzz.bmp")
    acc := acc "LoadPicture.missing=OK|[" r "]`n"
} catch as e {
    acc := acc "LoadPicture.missing=ERR|" e.Message "`n"
}
try {
    r := LoadPicture("C:\Windows\win.ini")
    acc := acc "LoadPicture.notimage=OK|[" r "]`n"
} catch as e {
    acc := acc "LoadPicture.notimage=ERR|" e.Message "`n"
}

FileAppend acc, "*"
