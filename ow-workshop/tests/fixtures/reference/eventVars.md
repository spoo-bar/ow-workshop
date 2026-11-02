# eventVars

Event values (event player, attacker, victim, healer, …) per rule event.

Expected output: `eventVars.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
#too lazy to put every single event-related variables
#list: eventPlayer, attacker, victim, eventDamage, eventWasEnvironment, healer, healee, eventHealing, eventWasHealthPack, eventWasCriticalHit, eventAbility, eventDirection

def epSub2():
    A = eventPlayer

def epSub():
    A = eventPlayer

def globalSub():
    epSub2()

def actuallyEpSub():
    epSub2()

subroutine recursion2

def recursion1():
    recursion2()

def recursion2():
    A = eventDamage
    recursion1()

rule "global":
    epSub() #should throw warning
    globalSub() #should also throw warning
    
rule "ep":
    @Event eachPlayer
    actuallyEpSub() #should not throw warning
    A = eventWasCriticalHit
    recursion1() #should throw warning
```
