; The window of the script, on the box.

window := Gui()
window.Title := "the window of the probe"
window.Add("Text", "hello from the port")
button := window.Add("Button", "press me")
window.Show()
Sleep 1500
MsgBox "the window is open"
window.Destroy()
