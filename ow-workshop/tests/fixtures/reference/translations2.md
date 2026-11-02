# translations2

Translated strings in player-variable language detection mode, with many languages.

Expected output: `translations2.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US). Catalogs: `translations2.en.po`, `translations2.es_es.po`, `translations2.fr.po`, `translations2.it.po`, `translations2.ja.po`, `translations2.ko.po`, `translations2.pl.po`, `translations2.pt.po`, `translations2.ru.po`, `translations2.th.po`, `translations2.tr.po`, `translations2.zh.po`.

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
settings {
    "lobby": {
        "allowPlayersInQueue": true,
        "enableMatchVoiceChat": true,
    },
    "gamemodes": {
        "skirmish": {
            "enabledMaps": [
                "workshopChamber"
            ],
        }
    },
}
#!setupTags
#!translations de fr zh es_es tr th ja ko pl pt ru it en
#!translateWithPlayerVar
rule "owo\nuwu":
    A = _("host player: {}").format(hostPlayer)
    debug(t"some string")

rule "":
    @Event eachPlayer
    @Condition eventPlayer.isHoldingButton(Button.INTERACT)
    bigMessage(getAllPlayers, "{}{}".format(_(A), buttonString(Button.INTERACT)))
    smallMessage(getAllPlayers, "{}{}".format(_(A), buttonString(Button.INTERACT)))
```
