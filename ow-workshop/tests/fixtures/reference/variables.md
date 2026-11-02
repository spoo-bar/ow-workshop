# variables

Global and player variable declaration, index allocation and number literal printing.

Expected output: `variables.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
#!optimizeForSize

globalvar foo 
globalvar bar 2
playervar foo #comment
playervar bar 2 #comment

rule "variables":
    @Event eachPlayer
    
    foo = bar+A+E
    eventPlayer.foo = eventPlayer.bar+eventPlayer.A+eventPlayer.E

rule "numbers":
    A = --0x124BC
    A = -0x124BC
    A = -0x124
    A = 0x124BC
    A = 0x124
    A = 1e10
```
