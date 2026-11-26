# The desktop of a Windows machine

The desktop of a Windows machine lives inside the crate `ahk-box`, in the
folder `crates/ahk-box/src/windows/`. The crate `ahk-box` is the box of
the tree: the backends that answer a machine live in it, and the one of
Windows lives with the one of the laboratory (`x11.rs`).

The desktop answers the traits of the crate `ahk`: `Machine`, `Windows`,
`Keyboard`, `Pointer`, `Screen`, `Processes`, `Files`, `Clipboard`,
`Timers`, `Hotkeys`, `Native`, `Sound`, `Dialogs`, `Network` and
`Pictures`. The sum of them is the trait `Desktop`, and the script talks
to the machine through it.

The measure of the backend is the source itself:

```
python scripts/what-windows-refuses.py
```

It reads the traits of the crate `ahk` and the desktop of the Windows
machine, and it says how many of the methods are carried and which ones
are not. The answer of the machine of the tree: **219 of
228**.

## What each capability answers

| Capability | What it answers |
| --- | --- |
| `Machine` | the name of the user, the name of the computer, the name of the shell, the path of the program, the version of the system, the language of the user, the folders of the machine, the folder of the programs, the folder of Windows, the drives and their labels and their serials, the registry, the times of the inactivity, the number of the thread, the icon of the tray |
| `Windows` | the titles of the windows, their classes, their places, their sizes, the process of one, the window of the focus, the window at a place, the enabled of one, the always-on-top of one, the controls of a window, the windows of the script |
| `Keyboard` | the typing of the text, the pressing of a key, the state of a key, the state of the locks, the light of the locks, the layout of the keyboard |
| `Pointer` | the click, the buttons, the movement, the place of the pointer, the shape of the cursor, the clipping of the pointer |
| `Screen` | the capture of the screen, the colour of a pixel, the size of the screen, the scale of it, the monitors and their work areas, the window that moves, the tip of the machine |
| `Processes` | the family of the processes, the run of a program, the start of one, the run with the script writing in it, the environment of the machine |
| `Files` | the reading, the writing, the copying, the moving, the deleting, the folders, the attributes, the times, the `.ini`, the list of the files, the path, the trash, the shortcuts |
| `Clipboard` | the reading and the writing of the text of the machine |
| `Timers` | the timers are the ones of the tree: the desktop does not take them over |
| `Hotkeys` | the registration of a combination of keys and the ones that fired |
| `Native` | the call of a function of a library of the machine |
| `Com` | the classes of the registry of the machine, the objects that it makes, the interfaces of them, the values of them and the methods of one of them |
| `Sound` | the beep, the playing of a file, the names of the devices of the sound, the volume of a device and whether it is muted; the devices are the COM of the machine |
| `Dialogs` | the box of the input of the script; the pickers of the files are the shell of the machine |
| `Network` | the addresses of the machine and the download of a file |
| `Pictures` | the loading of a picture and the search of a picture in the screen |

## The measurements

A rule without its measurement is an opinion. These are the measurements
of the machine of the tree, and the answers of the backend
carry them:

- **The common controls of the machine do not answer.** The class of the
  tree view is the one of the machine -- `SysTreeView32` -- and the
  messages of it do not answer: the failure of the machine is the number
  6. The same is of the list view (`SysListView32`) and of the tab
  (`SysTabControl32`). The controls that the library of the system has --
  the button, the edit, the list box and the combo box -- answer, and the
  tree of the script is built with them.

- **The machine makes the classes of the COM.** The class of the picker
  of the files is in the registry of the machine --
  `HKEY_CLASSES_ROOT\CLSID\{DC1C5A9C-E88A-4DDE-A5A1-60F82A20AEF7}` --
  and the machine makes it, and the picker of the save as well
  (`{C0B4E2F3-BA21-4773-8DBA-335EC946EB8B}`). The tree was refusing
  both with the number `0x80040154` (REGDB_E_CLASSNOTREG) for a long
  time, and the refusal was the tree's: the tenth byte of the number of
  the class was `0x1A` where the machine has `0xA1`. The bytes were
  measured in the table of the classes of `comdlg32.dll` itself, where
  the class of the picker of the files is the first of the four.

- **The apartment of the COM belongs to the thread.** The tree was
  refusing with the number `0x800401F0` (CO_E_NOTINITIALIZED) because it
  took the apartment once for the whole program, in a thread that was
  not the one that made the object. The apartment is taken by the
  thread that asks for it, and the refusal of `0x800401F0` is gone
  (measured on the machine of the tree).

- **The architecture of the program is not the cause of a refusal of the
  COM.** The program of thirty-two bits and the program of sixty-four
  bits refuse the same classes with the same numbers, and the machine
  refuses them the same way in a program of C, in a program of Rust and
  in a program of C#: what decides is the number of the class and not
  the architecture (measured on the machine of the tree).

- **The machine makes the class of the shell of the machine.** The class
  `CLSID_Shell` (`{13709620-C279-11CE-A49E-4445-5354-0000}`, the *Shell
  Automation Service*) is in the registry of the machine and a plain
  program makes it. The tree was carrying two bytes of that number
  wrong -- the fifth and the sixth of the last four were `0x35, 0x40`
  where the machine has `0x53, 0x54` -- and the machine was refusing it
  with the number `0x80040154`. The bytes of the tree were read by the
  *name* of the key of the registry
  (`{13709620-C279-11CE-A49E-444553540000}`), and a name is a text: the
  transcription of it is a thing of the tree, and the machine keeps the
  truth in its table.

- **The table of the classes of the shell is where the truth is.** The
  `shell32.dll` of the machine has a table of the classes that it
  serves, and the class of the shell is the thirty-fourth of them. The
  decompilation of the `DllGetClassObject` of the machine (with the
  kuna) says how that table is read: each entry has the pointer of its
  number at `+0x10`, and the comparison is byte by byte. The entry
  thirty-four has `20 96 70 13 79 C2 CE 11 A4 9E 44 45 53 54 00 00` --
  and that is the answer of the machine when the program passes the
  number of the table instead of its own.

- **The shortcuts of the machine are made by the shell of the machine.**
  The class of the link is served by the `shell32.dll` of the machine to
  a plain program, and the file that the shell makes is read back by
  the machine with the fields that were given to it (measured on the
  machine of the tree).

- **The devices of the sound are the COM of the machine, and the machine
  has six of them that play and six that record** (measured on the
  machine of the tree). The class of them is the
  *MMDeviceEnumerator class* --
  `HKEY_CLASSES_ROOT\CLSID\{BCDE0395-E52F-467C-8E3D-C4579291692E}` --
  and the machine makes it. The name of a device is the property
  `PKEY_Device_FriendlyName` of it ("Speakers", "Microphone",
  "Realtek Digital Output"), and the volume and the mute of it are the
  master of `IAudioEndpointVolume`.

- **The count of the devices comes from a place that the caller
  gives.** `IMMDeviceCollection::GetCount` answers with a number of the
  machine and writes the count in the place of the caller; the tree was
  calling it as a method that answers with the count itself, and the
  machine was writing the count in a place of nowhere -- the tree read a
  number of nothing and called the chain broken (measured on the
  machine of the tree).

- **The shape of the device is the one of the v2.0.31.** The device of
  the empty name is the device of the system, a number is the place of
  the device in the list of every device of the machine, and a name is
  the first device whose name begins with it -- the comparison of the v2
  is a prefix and it does not mind the case. The shape is read in the
  source of the v2.0.31 (`source/lib/sound.cpp` of the reference tree),
  and the tree follows it.

- **Opening the volume of many devices at once makes the sound of the
  machine stutter.** The volume of one device is work of the sound
  service, and twelve of them together stutter (measured on the machine
  of the tree).
  The device is lazy: the name is read when asked, the volume opened
  when asked.

- **The walk of the classes of the registry costs 130 ms, and the tree
  made it once per class.** The tree of the classes has 8016 keys. The
  tests asked for it many times at once, in several threads, and the
  machine stuttered. The walk is made once now, behind a lock: the first
  lookup costs 214 ms and the rest cost nanoseconds (measured on the
  machine of the tree).

- **The machine makes the objects of the COM, and the classes are the
  ones of its registry.** A class is a `ProgID` or the text of a
  `CLSID`, an interface is the text of an `IID` -- the `IDispatch` when
  the script does not say. The numbers are read from the machine and
  never written by hand (measured on the machine of the tree).

- **The word of the failure is the one of `FormatMessage`.** The v2
  writes the number and the word of the machine, and the tree writes the
  same: `0x80070057` is "The parameter is incorrect." and `0x80040154` is
  "Class not registered" (measured on the machine of the tree).

- **A method of the object is called by its place in the table.** The
  method with only the object is the `AddRef` -- the second of the table
  -- and it answers with the count of the references (measured on the
  machine of the tree).

- **The interface of a device of the sound is a pointer, and zero when
  the machine has none.** `SoundGetInterface` asks the
  `QueryInterface` of the device and, when the machine refuses, its
  `Activate`; the answer is the pointer, and zero for an interface that
  is not there (measured on the machine of the tree).

- **The class and the interface of an object come from the machine.**
  `ComObjType` asks the type information of the `IDispatch` for the
  `Name` and the `IID`, and the information of the class -- which the
  type library of the machine gives -- for the `Class` and the `CLSID`
  (measured on the machine of the tree). The object of the
  `Scripting.FileSystemObject`
  carries the interface `{2A0B9D10-4B87-11D3-A97A-00104B365C9F}` and the
  class `{0D43FE01-F093-11CF-8940-00A0C9054228}`, both the numbers of
  this machine's registry.

- **A member of an object is the `IDispatch` of the machine.** The name
  goes to `GetIDsOfNames` and the number to `Invoke`; the arguments come
  reversed -- the last one first -- and a write names its value with the
  `DISPID_PROPERTYPUT` of minus three. The answer is the `VARIANT` the
  machine left, and the machine owns its memory (measured on the
  machine of the tree:
  `o.readyState` gives four, `o.async := true` gives one back,
  `o.loadXML` gives one). The null `DISPPARAMS` makes the
  `MSXML2.DOMDocument` fall: even without arguments the machine wants a
  real one.

- **The events of an object carry the script to the machine's call.**
  `ComObjConnect` walks the class of the object in the type library of
  the machine, takes the implementation whose flags are the default and
  the source -- one and two, three --, finds the connection point of the
  number of that interface and gives the `sink` to its `Advise`. The
  `sink` is an `IDispatch` made on the heap; its `Invoke` takes the name
  of the member from the type information of the interface of the events
  and puts the event in the queue that the desktop reads between the
  statements of the script and inside the `Sleep`. Measured end to end on
  the machine of the tree: the object of the
  `MSXML2.DOMDocument` fires `onreadystatechange` into the `sink` and the
  function `Eonreadystatechange` of the script runs -- the prefix of the
  connection and the name of the member -- with the source object of the
  event as its parameter. The machine wants the `sink` to answer the
  interface of the events, the count of the types one, and the
  information of the type of them: with those refused, the object calls
  nothing. The tree had the read of a method of the table one dereference
  short -- it took the field of the object as the address of the method
  -- and the event of the machine was lost in quiet.

- **The numbers of the interfaces of the connection are the ones of
  this machine, not the ones of the headers.** The registry of the
  machine names `{B196B283-...}` `IProvideClassInfo`, `{B196B284-...}`
  `IConnectionPointContainer` and `{B196B286-...}` `IConnectionPoint`,
  and the objects of the machine answer the `QueryInterface` for these
  numbers (measured on the machine of the tree). The tree reads them
  from the machine;
  the numbers it had written by hand from memory were another family,
  and the machine refused them all.
- **The version of the system** is `10.0.26300`, and the name of the
  computer is `RAPHAEL`: the runner answers the machine for real.

- **The speed and the delay of the pointer are settings of the script.**
  `SetDefaultMouseSpeed` and `SetMouseDelay` carry `g->DefaultMouseSpeed`
  and `g->MouseDelay` of the source of the v2.0.31, and the tree keeps
  the same: the machine of the tree sends with `SendInput`, and the
  source says the `SendInput` ignores both settings -- "Mouse speed
  (aSpeed) is ignored for SendInput/Play mode: the mouse always moves
  instantaneously" and `DoMouseDelay()` "Inserts delay for all modes
  except SendInput, for which it does nothing"
  (`source/keyboard_mouse.cpp`). Measured on the machine of the tree `SetDefaultMouseSpeed(5)` answers `2` and
  `SetMouseDelay(20)` answers `10` -- the defaults of the v2 -- and a
  `MouseMove` of speed 3 moves at once.

- **The label of a drive is written by the machine.** `DriveSetLabel`
  is `SetVolumeLabelW` of the root of the drive, and the empty label is
  the null pointer of the text -- the way the machine takes a label
  away. Measured on the machine of the tree: a drive that
  is not there answers with its word, "(3) The system cannot find the
  path specified."

## The refusals that carry a reason

The rules of the tree: a refusal carries a reason, never `false`.

The refusals that are left are of the tree and not of the machine, and
each one says what is missing:

- the tip of the machine is a window of its own, and the tree does not
  make it yet;
- the mail is a thing of the network of the machine, and the tree does
  not carry it;
- the drives of the machine are driven by the machine alone, and the
  tree does not drive them.

## The floor

- The machine of the tree is Windows 11 (the version `10.0.26300`).
- The gate of the CI builds the whole workspace for the Windows of
  trinta e dois bits (`i686-pc-windows-msvc`), and the gate of the
  Linux box runs the tests, the clippy and the format of the tree.
- The runner and the tests of the desktop carry the manifest of the
  common controls of the version six: the tree view of the old ones
  speaks only the old language, and the texts of a script are wide. The
  manifest is at `ahk.manifest`, and the build scripts of the runner and
  of the desktop give it to the linker of Microsoft.
