struct INPUT {
    type : UInt32
    union {
        mi : MOUSEINPUT
        ki : KEYBDINPUT
    }
}
