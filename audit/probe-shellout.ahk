; The shell-out of a program that is not there.

try {
    Run("nonexistent-command-of-the-probe")
    MsgBox "run: no failure at all"
} catch as e {
    MsgBox "run: [" e.Message "]"
}
