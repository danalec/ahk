# The menu of the v1 — the measurements and the work

A script of the v1 has a menu bar with five items — `&File`,
`&Edit`, `&Interface`, `&Settings`, `&Help` — and every one of them opens
a dropdown that the script builds when the dropdown opens.

This is what the v1 does with the menus of the machine, what it does with
the numbers of the items, and what the tree does now. Every number here
was measured on the machine and on the binary of the v1.1.37.02, and the
probes are the ones of the end of the document.

## 1. The machine refuses a popup menu as the bar of a window

The `SetMenu` of the machine takes a **window** menu and refuses a popup
menu: the answer is zero and the error is 87,
`ERROR_INVALID_PARAMETER`.

**Measured on the machine**: the `SetMenu` of a window
menu answers one, and the `SetMenu` of a popup menu answers zero with the
error 87 — with every style of the window (the style of the v1, an
overlapped window, a popup window, a window with nothing).

The v1 makes its custom menus as popups and turns them into window menus
when the script attaches one to a window: the handle that
`MenuGetHandle` answers **changes** at that moment.

**Measured on the binary of the v1.1.37.02**: the
`Menu, MyBar, Delete` of a bar that a window uses leaves the bar alone,
and the menu that it opens alive.

## 2. The menu of the script is a window menu

The tree makes the menu of the script with `CreateMenu` and not with
`CreatePopupMenu`: the bar of the window of the script **is** the menu of
the script, and the dropdowns of the bar are the menus that the script
made.

The window menu takes a popup under it, and a window menu shows as a
popup, so the menus of the script are window menus on both counts.

**Measured on the machine**: the `AppendMenu` of a popup
menu under a window menu works, and the `TrackPopupMenu` of a window menu
works.

The bar is not a copy of the menu: a copy is a menu of the machine of its
own, and the machine takes the menus that are attached to a menu when it
destroys it. The copy took the menus of the script away, and the dropdown
of the bar opened nothing.

## 3. The items carry their own number

The number of an item is the number of the command that the machine sends
when the item is chosen, and the number of a command is the **low word**
of it: the numbers of the items of the machine stay under 65536.

The v1 gives every item its own number — the items that open a menu
too — with the `InsertMenuItem` and the `MIIM_ID`:

> "Although AppendMenu() ignores the ID when adding a separator and
> provides no way to specify the ID when adding a submenu, that is purely
> a limitation of AppendMenu(). Using InsertMenuItem() instead allows us
> to always set the ID, which simplifies identifying separator and
> submenu items later on."
> (`script_menu.cpp:753-759` of v1.1.37.02)

The tree keeps one counter for the whole of the script, and an item that
is deleted gives its number back: the numbers of the machine do not grow
for ever.

## 4. The items go with `RemoveMenu`

The machine takes the menu of an item away when the item is deleted with
`DeleteMenu`, and the menus of the script live as long as the script
does. The v1 removes the items with `RemoveMenu`:

> "RemoveMenu(mMenu, aMenuItem_ID, aMenuItem_MF_BY); v1.0.48: Lexikos:
> DeleteMenu() destroys any sub-menu handle associated with the item, so
> use RemoveMenu. Otherwise the submenu handle stored somewhere else in
> memory would suddenly become invalid."
> (`script_menu.cpp:790` of v1.1.37.02)

**Measured on the machine**: the `DeleteMenu` of an item
that opens a menu takes the menu of the machine away (the count of its
items becomes -1), and the `RemoveMenu` of it does not.

The `ModifyMenuW` and the `SetMenuItemInfo` do not detach the item
either: the item keeps its menu, and the `DeleteMenu` that follows takes
it away. The only way to keep the menu of an item is to remove the item
with `RemoveMenu`.

## 5. The menu of the script is emptied in place

The v1 empties the menu of the script in place: the handle of the menu
does not change, and the menu that the items opened stays alive, with its
items.

**Measured on the binary of the v1.1.37.02**: the
`Menu, MyBar, DeleteAll` of a bar that opens the menu `MyMenu` leaves
`MyMenu` alive, with its items, and the handle of the bar is the same.

The machine takes the menus that the items opened away when the menu
itself is destroyed, and the v1 makes them again when they are needed
(`script_menu.cpp:1352-1360` of v1.1.37.02). The tree does the same:
the menus of the machine that are dead are made again, the deepest
first.

## 6. The click on an item of a dropdown runs its label

The click on an item of a dropdown of the bar is a command with the
number of the item, and the procedure of the window looks the number up
in the table of the callbacks.

The table holds the callbacks of **every** item of the bar and of the
menus that it opens: without the items of the dropdowns, the click on an
item of a dropdown runs nothing.

**Measured on the binary of the v1.1.37.02**: the click on
an item of a dropdown of the bar of a script of the v1 runs its
label.

## 7. The menu that the script deletes

The v1: "If MenuItemName is omitted, the entire MenuName menu will be
deleted as will any menu items in other menus that use MenuName as a
submenu" (the CHM of the v1.1.37.02, `Menu.htm`).

The v1 refuses to destroy a menu that a window of the script uses as its
menu bar, and the menu of the machine survives the delete of the item
that opened it.

## 8. The map of a script of the v1 has numbers for keys

A script of the v1 keeps the map of the dropdowns that it has to
build, and the keys of the map are the handles of the menus of the
machine: `menuJITmap[hSub] := "InvokeMenuBar" suffix`.

The `HasKey` of the v1 finds a number for a key: the object of the v1
can have a number for a key, and the `HasKey` of it answers 1.

**Measured on the binary of the v1.1.37.02**: the
`map[12345] := "x"` of the v1 has the key 12345, and the
`map.HasKey(12345)` answers 1. The `HasKey` of the text "12345" answers
0: the number and the text of it are two keys.

The tree reads the key of the `HasKey` as a text: the number is found,
and the dropdown of a script of the v1 is built. The text "12345" is the
same key as the number 12345 in the tree, and the v1 keeps them apart.

**Measured on the binary of the v1.1.37.02 and on the tree**: the
`HasKey(12345)` answers 1 in the two; the `HasKey("12345")` answers 0 in
the v1 and 1 in the tree.

## 9. The icon of the window of the script

The v1: the window of the script carries the icon of the program that
runs it — the small one in the title bar and the big one in the list of
the windows. The icon of the tray is another thing: it does not change
the icon of the window.

**Measured on the binary of the v1.1.37.02**: the window
of the script of the v1 has an icon, and the `Menu, Tray, Icon` does not
change it.

The tree gives the icon of the program to the windows of the v1.

The **class** of the window carries the icon as well: the big one and the
small one. The tree registered the class without them, and the window of
the script showed no icon in the list of the windows.

**Measured on the binary of the v1.1.37.02**: the class of the window of
the script of the v1 has the icon of the program (the big one and the
small one), and the tree had none.

The tree puts the icon of the program in the class of the window, when it
registers the class.

## 10. The bar of the window keeps its handle

The bar of the window is the menu of the script, and the handle of it is
the handle that the script holds. The handle changes **once**: when the
script attaches the menu to a window, the menu of the machine goes from a
popup to a window menu, and the machine makes a new one.

After that the handle does not change, and the bar of the window is not
destroyed and made again for every command of the script.

**Measured on the machine**: the `SetMenu` of a window takes a window
menu, and the window that takes another menu does not destroy the menus of
the script.

**Measured on the binary of the v1.1.37.02**: the handle that
`MenuGetHandle` answers changes when the menu of the script is attached to
a window, and it does not change after that.

The tree destroyed the bar and made it again for every command `Menu` of
the script, and the window was given the new bar every time. The menu
that is open goes with the bar that the machine destroyed, and the
dropdown that the script builds **while it is open** was never ready: the bar opened, and the items that it showed
were the placeholder `building the menu...`.

The `SetMenu` in the middle of the open dropdown is what brings the menu
down (measured on the machine), and it is the reason why that bar did not open.

The tree keeps the handle of the machine when the kind of the menu does
not change, and the menus that are dead are made again **before** the bar
that opens them: a bar that opens a menu that is dead opens nothing.

## 11. The icon of the tray, and the image of an item

The icon of the tray belongs to the window of the script, the tip of it is
the name of the file of the script, and it goes away when the run ends.

**Measured on the binary of the v1.1.37.02**: the `Menu, Tray, Icon` of
a script puts the icon in the area of notification, the name of the icon
is the name of the file of the script (the `Menu, Tray, Tip` adds its text
to it), and the icon disappears when the run ends — and a script that
lives on (`Persistent`) has one icon, not two.

The tree put the icon in the area of notification with the tip `"ahk"`,
owned by a window of messages of its own, and the `NIM_DELETE` of the end
of the run pointed at another window: the icon stayed behind when the
process lived on.

The image of an item of a menu goes away when the file of it is not
named: the v1 takes the image away from the item.

**Measured on the binary of the v1.1.37.02**: the
`Menu, MyMenu, Icon, Item` of a menu whose item carries an image takes
the image away from it, and the number of the item and the menu that it
opens stay the same.

The tree asks the machine to take the image away, and the machine does
not: the item goes out of the menu and comes back without the image.

## The probes

The probes are in the probe of the measurement:

| The probe | What it measures |
|---|---|
| `probe-menu.ahk` | the life of the menus, the labels of the items, the `MenuGetHandle` of a menu that is not there |
| `probe-menu2.ahk` | the menu that the bar opens, through the `DeleteAll` of the bar |
| `probe-menu3.ahk` | the dropdown of the bar and the menu of the script |
| `probe-menu4.ahk` | the handles of the menus when the menu of the script is emptied |
| `probe-ids.ahk` | the numbers of the items of the menus |
| `probe-jit.ahk` | the pattern of a script of the v1: the dropdown is built when it opens |
| `probe-icon.ahk` | the icon of the windows of the script |
| `probe-win32.ahk` | the `ModifyMenuW`, the `DeleteMenu` and the `DestroyMenu` of the machine |
| `probe-map.ahk` | the keys of a map of the v1 |

Every one of them runs under the binary of the v1.1.37.02 and under the
tree, and the answers of the two are the same.
