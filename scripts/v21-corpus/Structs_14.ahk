struct lldiv_t {
    quot : Int64, rem : Int64
}
res := DllCall("ucrtbase\lldiv", "int64", 31558149, "int64", 3600, lldiv_t)
MsgBox Format("Earth orbit: {} hours and {} seconds.", res.quot, res.rem)
