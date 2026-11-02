# rulePrefix

Rule name prefixes and the prefix template (uses two included files, shown below).

Expected output: `rulePrefix.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text

rule "rule 1":
    A = 1

#!rulePrefix "test prefix 1"

rule "rule 2":
    A = 2

rule "":
    A = 3

#!rulePrefix "test prefix 2"

rule "rule 4":
    A = 4

##!rulePrefixTemplate f"prefix:[{$prefixTitle}] file:[{$fileUpper}] path:[{$pathLower}] | {$rule or '<empty>'}" if $prefix and $isDelimiter != true else $rule
#!rulePrefixTemplate 

#!include "test include/test prefix.opy"

rule "rule 7":
    A = 7

#!include "test include/include4.opy"

rule "rule 8":
    A = 8

rule "---- DELIMITER ----":
    @Delimiter
```

Included file `test include/test prefix.opy`:

```text
rule "rule 5":
    A = 5

#!rulePrefix "test prefix 3"

rule "rule 6":
    A = 6
```

Included file `test include/include4.opy`:

```text
rule "4":
    A = 4
```
