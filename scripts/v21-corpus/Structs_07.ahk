struct bool {
    value : UInt8  ; This property name has no special meaning.
    __value {
        get => this.value
        set => this.value := !!value
    }
}

struct Example {
    bInformed : bool
    __new() {
        for value in [false, "truthy", 3] {
            this.bInformed := value
            MsgBox value " converts to " this.bInformed
        }
    }
}

Example()
