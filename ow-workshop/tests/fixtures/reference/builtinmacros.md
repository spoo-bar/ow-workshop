# builtinmacros

Table helpers: `splitDictArray` (rows of dicts) and `tabular` (flat rows) assigning one array per column.

Expected output: `builtinmacros.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
#!optimizeForSize

globalvar waveHeroes
globalvar waveLengths

rule "splitDictArray":
    splitDictArray({
        hero: waveHeroes,
        length: waveLengths
    }, [
        {
            hero: Hero.ANA,
            length: 3,
        }, {
            length: 8,
            hero: Hero.SOLDIER,
        }, {
            hero: Hero.HAMMOND,
        }
    ])

rule "tabular":
    tabular([waveHeroes, waveLengths], [
        Hero.ANA, 3,
        Hero.SOLDIER, 8,
        Hero.WRECKING_BALL, null,
    ])
```
