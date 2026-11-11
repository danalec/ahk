plays := SoundGetName()
FileAppend "plays=[" plays "]", "*"
records := SoundGetName(1)
FileAppend "records=[" records "]", "*"
second := SoundGetName(2)
FileAppend "second=[" second "]", "*"
interface := SoundGetInterface("{C02216F6-8C67-4B5B-9D00-D008E73E0064}")
FileAppend "interface=[" interface "]", "*"
try {
  SoundGetInterface("not-a-guid")
  FileAppend "bad=[no failure]", "*"
} catch as e {
  FileAppend "bad-reason=[" e.Message "]", "*"
}
