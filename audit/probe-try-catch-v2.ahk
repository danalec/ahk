; The reference: does the catch block run when the try block did not fail?
try {
    x := 1
} catch as e {
    FileAppend("caught|", "*")
}
FileAppend("after|", "*")

try {
    throw Error("boom")
} catch as e {
    FileAppend("caught with: " e.Message "|", "*")
}
FileAppend("after2", "*")
ExitApp
