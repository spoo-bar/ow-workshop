# Reference fixtures

Known-good workshop output from OverPy v9.7.20 (`spoo-bar/overpy@e349a32`, en-US), re-verified against that compiler on import. See spoo-bar/ow-workshop#99.

Each fixture has:
- `<name>.txt`: the expected output.
- `<name>.md`: what it covers, plus the original program as reference. That's not a supported input format: port it by hand to a `Workshop` impl (#100, #101).
- `<name>.<lang>.po`: translation catalogs, for the translation fixtures.

Compare with whitespace-only lines and CRLF ignored (#19). Allowed differences: numbers printed from `f64` rather than source spelling, label comment names, and features of the old text language (includes, text macros). Note them per fixture.

| Fixture | Covers |
|---|---|
| arrays | array literals and functions |
| builtinenums | built-in enums as values |
| builtinmacros | table helpers |
| comments | statement and condition comments |
| compression | array compression |
| conditions | if / else if / else |
| dicts | dictionary lookups |
| elementSaving | optimize-for-size replacements |
| enums | user enums |
| eventVars | event values |
| functions | functions, folding, default arguments |
| getCurrentMap | Current Map workaround |
| gotos | labels and skips |
| loops | while, for, do-while, break, continue |
| operators | operators, strict mode |
| rulePrefix | rule name prefixes |
| rules | events, filters, rule flags |
| subroutines | subroutines and Start Rule |
| switches | switch lowering |
| variables | variable allocation, numbers |
| strings | string modifiers, tags, splitting |
| strings_args | splitting with many placeholders |
| lowercase | cased progress-bar text |
| customGameSettings | settings block |
| shionSettings | hero settings |
| extensions | extensions |
| workshopSettings | workshop setting values |
| translations | translated strings, catalogs |
| translations2 | player-variable translation mode |

The last nine arrive in a second PR to stay under the repo's PR size limit. The large real-mod fixtures (stretch goal in #99) aren't imported yet.
