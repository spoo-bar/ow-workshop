// Converts the raw dump produced by entry.ts into the data/*.json schema (spoo-bar/ow-workshop#20).
// Usage: node transform.mjs <raw.json> <out-dir>
import fs from "node:fs";
import path from "node:path";

const [rawPath, outDir] = process.argv.slice(2);
const raw = JSON.parse(fs.readFileSync(rawPath, "utf8"));
fs.mkdirSync(outDir, { recursive: true });

const LANGS = ["de-DE", "en-US", "es-ES", "es-MX", "fr-FR", "it-IT", "ja-JP", "ko-KR", "pl-PL", "pt-BR", "ru-RU", "th-TH", "tr-TR", "zh-CN", "zh-TW"];
const problems = [];

const names = (o) => {
    const out = {};
    for (const l of LANGS) if (typeof o?.[l] === "string") out[l] = o[l];
    if (typeof out["en-US"] !== "string" && !o?.onlyInOverpy) problems.push(`missing en-US: ${JSON.stringify(o).slice(0, 80)}`);
    return out;
};
const doc = (o) => o?.description?.["en-US"] || undefined;
const sortKeys = (o) => Object.fromEntries(Object.entries(o).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0)));
const nonEmpty = (o) => (Object.values(o).some(Boolean) ? compact(o) : undefined);
const compact = (o) => JSON.parse(JSON.stringify(o, (_, v) => (v === undefined || v === false ? undefined : v)));

// Pretty-print up to `depth` levels, then one compact line per value.
const fmt = (v, depth, ind = "") => {
    if (depth === 0 || v === null || typeof v !== "object") return JSON.stringify(v);
    const next = ind + "  ";
    const isArr = Array.isArray(v);
    const items = isArr ? v.map((x) => next + fmt(x, depth - 1, next)) : Object.entries(v).map(([k, x]) => next + JSON.stringify(k) + ": " + fmt(x, depth - 1, next));
    if (items.length === 0) return isArr ? "[]" : "{}";
    return (isArr ? "[\n" : "{\n") + items.join(",\n") + "\n" + ind + (isArr ? "]" : "}");
};
const write = (file, value, depth) => fs.writeFileSync(path.join(outDir, file), fmt(value, depth) + "\n");

// Enum renames: internal OverPy names to schema names.
const ENUM_RENAMES = { ColorLiteral: "Color", ButtonLiteral: "Button", TeamLiteral: "Team", __Operation__: "ModifyOp", __Operator__: "CompareOp", __Rounding__: "Rounding" };
const ENUM_SKIP = new Set(["__ChaseReeval__", "HeroLiteral", "MapLiteral", "GamemodeLiteral"]);
const TYPE_RENAMES = { __Operation__: "ModifyOp", __Operator__: "CompareOp", __Rounding__: "Rounding" };

const ty = (t) => {
    if (typeof t === "string") return TYPE_RENAMES[t] ?? t;
    if (Array.isArray(t)) return t.map(ty);
    return Object.fromEntries(Object.entries(t).map(([k, v]) => [k, Array.isArray(v) ? v.map(ty) : ty(v)]));
};
const flatTypes = (t) => (typeof t === "string" ? [t] : Array.isArray(t) ? t.flatMap(flatTypes) : Object.keys(t));

const constEnumFor = (argType, member) => {
    for (const t of flatTypes(argType)) {
        for (const cand of [t, t + "Literal"]) {
            if (raw.constantValues[cand]?.[member] && !ENUM_SKIP.has(cand)) return [ENUM_RENAMES[cand] ?? cand, member];
        }
    }
    return null;
};

// Known bad defaults in the source data, corrected here.
const DEFAULT_FIXES = {
    "createIcon.reevaluation": "VISIBILITY_AND_POSITION",
    "createIcon.color": "WHITE",
    "getNumberOfHeroes.team": "ALL",
};

const convertDefault = (d, arg, fnKey) => {
    d = DEFAULT_FIXES[`${fnKey}.${arg.name}`] ?? d;
    if (typeof d === "number") return { num: d };
    if (typeof d === "boolean") return { bool: d };
    if (d === null) return "null";
    if (d === "Math.INFINITY") return { num: 999999999999 };
    if (d === "getAllPlayers()") return { value: "getPlayers", args: [{ const: ["Team", "ALL"] }] };
    if (d === "vect(0,0,0)") return { vector: [0, 0, 0] };
    const member = d.replaceAll(" ", "_");
    const c = constEnumFor(arg.type, member);
    if (c) return { const: c };
    problems.push(`unresolved default ${JSON.stringify(d)} for ${fnKey}.${arg.name}`);
    return { raw: d };
};

const fnKey = (k) => k.replace(/^\./, "").replace(/^__(.*)__$/, "$1");
const functions = (kw) =>
    Object.entries(kw)
        .map(([k, f]) =>
            compact({
                key: fnKey(k),
                source_key: k,
                member: k.startsWith("."),
                internal: k.startsWith("__") || k.startsWith(".__") || undefined,
                names: names(f),
                doc: doc(f),
                args: (f.args ?? []).map((a) =>
                    compact({
                        name: a.name,
                        ty: ty(a.type),
                        default: "default" in a ? convertDefault(a.default, a, k) : undefined,
                        min: a.min,
                        max: a.max,
                        literal_max: a.literalMax,
                        replace: nonEmpty({
                            zero_by_false: a.canReplace0ByFalse,
                            zero_by_null: a.canReplace0ByNull,
                            one_by_true: a.canReplace1ByTrue,
                            null_vector_by_null: a.canReplaceNullVectorByNull,
                            empty_string_by_empty_array: a.canReplaceEmptyStringByEmptyArray,
                        }),
                    }),
                ),
                ret: ty(f.return),
                has_literal_limit: f.hasLiteralLimit,
                is_constant: f.isConstant,
                is_literal: f.isLiteral,
                can_be_put_in_boolean: f.canBePutInBoolean === false ? false : true,
            }),
        )
        .map((f) => ({ ...f, can_be_put_in_boolean: f.can_be_put_in_boolean ?? false }))
        .sort((a, b) => (a.key < b.key ? -1 : 1));

write("actions.json", functions(raw.actionKw), 1);
write("values.json", functions(raw.valueFuncKw), 1);

// Constants
const constants = {};
for (const [enumKey, members] of Object.entries(raw.constantValues)) {
    if (ENUM_SKIP.has(enumKey)) continue;
    const out = {};
    for (const [m, v] of Object.entries(members)) {
        if (m === "description" || typeof v !== "object") continue;
        out[m] = compact({
            names: names(v),
            doc: doc(v),
            extension: v.extension,
            rgba: "red" in v ? [v.red ?? 0, v.green ?? 0, v.blue ?? 0, v.alpha ?? 255] : undefined,
            only_in_tooling: v.onlyInOverpy,
        });
    }
    constants[ENUM_RENAMES[enumKey] ?? enumKey] = out;
}
write("constants.json", sortKeys(constants), 2);

// Heroes, maps, gamemodes
const ABILITIES = { primaryFire: "primary_fire", secondaryFire: "secondary_fire", ability1: "ability1", ability2: "ability2", ability3: "ability3", ultimate: "ultimate", passive: "passive" };
write(
    "heroes.json",
    sortKeys(Object.fromEntries(Object.entries(raw.heroKw).map(([k, h]) => [k, compact({ names: names(h), abilities: Object.fromEntries(Object.entries(ABILITIES).filter(([a]) => h[a]).map(([a, s]) => [s, names(h[a])])) })]))),
    1,
);
write(
    "maps.json",
    sortKeys(
        Object.fromEntries(
            Object.entries(raw.mapKw).map(([k, m]) => [
                k,
                compact({
                    names: names(m),
                    gamemodes: m.gamemodes,
                    variants: m.variants,
                    is_workshop_map: m.isWorkshopMap,
                    only_in_ow1: m.onlyInOw1,
                    symmetry: m.isSymmetrical ? compact({ axis: m.symmetryAxis }) : undefined,
                }),
            ]),
        ),
    ),
    1,
);
write(
    "gamemodes.json",
    sortKeys(Object.fromEntries(Object.entries(raw.gamemodeKw).map(([k, g]) => [k, compact({ names: names(g), default_team1_slots: g.defaultTeam1Players, default_team2_slots: g.defaultTeam2Players, default_ffa_slots: g.defaultFfaPlayers, only_in_ow1: g.onlyInOw1 })]))),
    1,
);

// Keywords
const strip = (k) => k.replace(/^__(.*)__$/, "$1");
const mapNames = (o, keyFn = (k) => k) => Object.fromEntries(Object.entries(o).filter(([, v]) => typeof v === "object" && v !== null).map(([k, v]) => [keyFn(k), names(v)]));
write(
    "keywords.json",
    {
        structure: mapNames(raw.ruleKw, strip),
        event: mapNames(raw.eventKw, strip),
        event_team: mapNames(raw.eventTeamKw, (k) => (k === "all" ? "all" : "team" + k)),
        event_player: mapNames(raw.eventPlayerKw, (k) => (/^\d+$/.test(k) ? "slot" + k : k)),
        settings: mapNames(raw.customGameSettingsKw, strip),
    },
    2,
);

// Custom game settings
const KINDS = { __boolOnOff__: "bool_on_off", __boolReverseOnOff__: "bool_reverse_on_off", __boolYesNo__: "bool_yes_no", __boolEnabled__: "bool_enabled", __boolReverseEnabled__: "bool_reverse_enabled", __percent__: "percent", __int__: "int", __float__: "float", __string__: "string" };
const setting = (s) => {
    const kind = typeof s.values === "string" ? KINDS[s.values] : "combo";
    if (!kind) problems.push(`unknown setting kind ${s.values}`);
    return compact({
        names: names(s),
        kind,
        choices: kind === "combo" ? Object.fromEntries(Object.entries(s.values).map(([k, v]) => [k, names(v)])) : undefined,
        min: s.min,
        max: s.max,
        max_chars: s.maxChars,
        max_bytes: s.maxBytes,
        include: s.include,
        exclude: s.exclude,
    });
};
const LABELS = new Set(["enabledMaps", "disabledMaps"]);
const settingsMap = (vals) => Object.fromEntries(Object.entries(vals ?? {}).filter(([k]) => !LABELS.has(k)).map(([k, v]) => [k, setting(v)]));
const cgs = raw.customGameSettingsSchema;
const heroVals = cgs.heroes.values;
const customGameSettings = {
    main: { names: names(cgs.main), values: settingsMap(cgs.main.values) },
    lobby: { names: names(cgs.lobby), values: settingsMap(cgs.lobby.values) },
    gamemodes: {
        names: names(cgs.gamemodes),
        enabled_maps: names(cgs.gamemodes.values.general.values.enabledMaps),
        disabled_maps: names(cgs.gamemodes.values.general.values.disabledMaps),
        values: Object.fromEntries(Object.entries(cgs.gamemodes.values).map(([k, g]) => [k, compact({ names: names(g), default_team1_slots: g.defaultTeam1Players, default_team2_slots: g.defaultTeam2Players, default_ffa_slots: g.defaultFfaPlayers, values: settingsMap(g.values) })])),
    },
    heroes: {
        names: names(cgs.heroes),
        teams: mapNames(cgs.heroes.teams),
        enabled_heroes: names(heroVals.enabledHeroes),
        disabled_heroes: names(heroVals.disabledHeroes),
        general: settingsMap(heroVals.general.values),
        per_hero: Object.fromEntries(Object.entries(heroVals).filter(([k]) => !["enabledHeroes", "disabledHeroes", "general"].includes(k)).map(([k, h]) => [k, settingsMap(h.values)])),
    },
    workshop: { names: names(cgs.workshop) },
    extensions: { names: names(cgs.extensions), values: Object.fromEntries(Object.entries(cgs.extensions.values).map(([k, e]) => [k, { names: names(e), points: e.points }])) },
};
fs.writeFileSync(
    path.join(outDir, "custom_game_settings.json"),
    fmt(
        Object.fromEntries(
            Object.entries(customGameSettings).map(([k, v]) => [k, v]),
        ),
        3,
    ) + "\n",
);

// Strings, entities, textures, fonts
write("localized_strings.json", Object.values(raw.stringKw).filter((v) => typeof v === "object").map((v) => ({ names: names(v) })).sort((a, b) => (a.names["en-US"] < b.names["en-US"] ? -1 : 1)), 1);
write("string_entities.json", sortKeys(Object.fromEntries(Object.entries(raw.opyStringEntities).map(([k, v]) => [k, v.codepoint]))), 1);
write("textures.json", sortKeys(raw.opyTextures), 1);

const widths = Object.entries(raw.blizzGlobalWidths).sort(([a], [b]) => a.codePointAt(0) - b.codePointAt(0));
const widthLines = [];
for (let i = 0; i < widths.length; i += 40) widthLines.push("    " + widths.slice(i, i + 40).map(([c, w]) => JSON.stringify(c) + ": " + w).join(", "));
const lowercase = Object.fromEntries(Object.entries(raw.lowercase).map(([c, g]) => [c, { lower: g.lower, width: g.width, xmin: g.xmin, lower_width: g.lowerWidth, lower_xmin: g.lowerXmin }]));
fs.writeFileSync(
    path.join(outDir, "font_widths.json"),
    "{\n" +
        `  "default": ${raw.blizzGlobalDefaultWidth},\n` +
        `  "spaces": ${JSON.stringify(Object.fromEntries(Object.entries(raw.spaces).map(([w, c]) => [w, c])))},\n` +
        `  "lowercase": ${fmt(lowercase, 1, "  ")},\n` +
        `  "widths": {\n${widthLines.join(",\n")}\n  }\n}\n`,
);

// Type tree
const renameTree = (t) => (typeof t === "string" ? TYPE_RENAMES[t] ?? ENUM_RENAMES[t] ?? t : Object.fromEntries(Object.entries(t).map(([k, v]) => [TYPE_RENAMES[k] ?? k, v.map(renameTree)])));
write("types.json", raw.typeTree.filter((t) => typeof t !== "string" || !ENUM_SKIP.has(t)).map(renameTree), 1);

const counts = {
    actions: Object.keys(raw.actionKw).length,
    values: Object.keys(raw.valueFuncKw).length,
    constant_enums: Object.keys(constants).length,
    heroes: Object.keys(raw.heroKw).length,
    maps: Object.keys(raw.mapKw).length,
    gamemodes: Object.keys(raw.gamemodeKw).length,
    localized_strings: Object.values(raw.stringKw).filter((v) => typeof v === "object").length,
};
console.log(JSON.stringify(counts));
if (problems.length) {
    console.error(problems.join("\n"));
    process.exit(1);
}
