; The name of the v2: which characters may it carry?
; Each case is a declaration of a function whose name is the candidate.
★a★b★c() {
}
---
é() {
}
---
Ω() {
}
---
€() {
}
---
a★b() {
}
---
a€b() {
}
---
★() {
}
---
€() {
}
---
a€() {
}
---
_★() {
}
---
1★() {
}
---
★1() {
}
---
a b() {
}
---
a-b() {
}
---
a.b() {
}
---
a€b€c() {
}
