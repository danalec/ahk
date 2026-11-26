; One control of every kind of the Gui of the v1.
;
; Measured on the binary of AutoHotkey v1.1.37.02: the class of a
; control of the v1 is the class of the window of the machine that it
; makes. The classes of the controls that this probe makes are read from
; the machine with `scripts/gui-classes.py` -- the class of the `Link` is
; `SysLink`, and the tree asked for `SysLink32`, that the machine does
; not have.

Gui, +Resize
Gui, Add, Button, , a button
Gui, Add, CheckBox, , a check box
Gui, Add, Radio, , a radio
Gui, Add, GroupBox, , a group
Gui, Add, Text, , a text
Gui, Add, Edit, , an edit
Gui, Add, ListBox, , a list box
Gui, Add, ComboBox, , a combo box
Gui, Add, DDL, , a drop down list
Gui, Add, Progress, , a progress
Gui, Add, Slider, , a slider
Gui, Add, UpDown, , an up down
Gui, Add, DateTime, , a date and a time
Gui, Add, MonthCal, , a month calendar
Gui, Add, Hotkey, , a hot key
Gui, Add, Link, , <a href="https://example.com/">a link</a>
Gui, Add, Tab, , one|two
Gui, Add, StatusBar
Gui, Add, TreeView, ,
Gui, Add, ListView, , a list view
Gui, Show, , TheControls
