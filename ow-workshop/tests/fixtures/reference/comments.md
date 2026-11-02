# comments

Statement and condition comments printed as quoted lines above the statement.

Expected output: `comments.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
#!optimizeForSize

/*
multi
line
comment
*/

rule "comments":
    @Condition A #A
    #B
    @Condition B
    #C++
    C++
    D++ #D++
    #nothing
    #some array
    A = [1,2/*,2.5*//**/,3]
```
