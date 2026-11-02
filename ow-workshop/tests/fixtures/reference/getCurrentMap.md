# getCurrentMap

The Current Map workaround for maps that cannot be compared directly.

Expected output: `getCurrentMap.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
#!optimizeForSize

rule "test current map workaround for colosseo/esperança/samoa":
    A = getCurrentMap()
    B = (A == Map.SAMOA)
    C = (getCurrentMap() == Map.KINGS_ROW)
    C = (Map.KINGS_ROW == getCurrentMap())
    C = (getCurrentMap() == Map.COLOSSEO)
    C = (Map.COLOSSEO == getCurrentMap())
```
