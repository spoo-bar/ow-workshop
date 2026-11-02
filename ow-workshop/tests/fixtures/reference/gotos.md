# gotos

Labels and skips, dead code around referenced labels, and dynamic skips.

Expected output: `gotos.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
#!optimizeForSize

rule "gotos":
    goto lbl_0 #skip 3
    if false:
        goto lbl_1 #skip 5
        A = 1
        lbl_0:
        A = 2

    if false:
        A = 1
        lbl_1:
        A = 2

    goto loc+A
    goto lbl_2 #skip 3
    goto lbl_3 #skip 0
    lbl_3:

    if false:
        A = 1
        lbl_2:
        A = 2
```
