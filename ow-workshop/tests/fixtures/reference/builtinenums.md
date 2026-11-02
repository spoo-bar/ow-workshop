# builtinenums

Built-in enums (buttons, heroes, colors, teams) used as values and arguments.

Expected output: `builtinenums.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
#!optimizeForSize

rule "enums":
    @Event eachPlayer
    @Condition eventPlayer.isHoldingButton(Button.INTERACT)
    @Condition A
    A = Team.ALL
    B = Hero.BRIGITTE
    C = Map.BLACK_FOREST_WINTER
    D = getCurrentGamemode() == Gamemode.ASSAULT
    E = Button.INTERACT

    F = eventPlayer.isHoldingButton(Button.INTERACT)
    eventPlayer.startForcingButton(Button.MELEE)
    eventPlayer.startForcingHero(Hero.ANA)
    wait(0.016, Wait.ABORT_WHEN_FALSE)

    G = Vector.BACKWARD
    H = random.randint(1, 2) + random.choice(A) + random.uniform(1.2, 3.4) + random.shuffle(A)
    I = Math.E
    J = Math.PI
    K = Color.BLUE
    S = Math.FUCKTON_OF_SPACES
```
