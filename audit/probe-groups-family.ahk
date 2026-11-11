GroupAdd("mine", "Steam")
GroupAdd("mine", "Chromium")
GroupAdd("mine", "tester@archlinux")
a := GroupActivate("mine")
FileAppend "active=[" a "]", "*"
GroupAdd("outro", "zzz-nothing-zzz")
b := GroupActivate("outro")
FileAppend "missing=[" b "]", "*"
GroupClose("mine")
c := GroupActivate("mine")
FileAppend "closed=[" c "]", "*"
