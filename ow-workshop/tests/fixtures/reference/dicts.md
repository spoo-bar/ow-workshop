# dicts

Dictionary lookups lowered to `Value In Array(values, Index Of Array Value(keys, key))`.

Expected output: `dicts.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
#!optimizeForSize

rule "dicts":
    A = {
        Hero.PHARAH: 2,
        Hero.BRIGITTE: 3,
    }[B]

    A = {
        Hero.PHARAH: 2,
        Hero.BRIGITTE: 3,
    }[1] #null

    A = {
        Hero.PHARAH: 2,
        Hero.BRIGITTE: 3,
    }[Hero.PHARAH] #2
```
