"""Patch the upstream CEM 2.1.0 schema so typify output is lossless (D-046).

schema-2.1.0.json is the untouched upstream file. Run via `just gen-cem-types`.
"""

import copy
import json
from pathlib import Path

here = Path(__file__).parent
schema = json.loads((here / "schema-2.1.0.json").read_text())
defs = schema["definitions"]

# 1. Untagged serde takes the first variant that fits. ClassDeclaration has no
#    additionalProperties: false, so it would swallow custom elements and drop tagName etc.
decls = defs["JavaScriptModule"]["properties"]["declarations"]["items"]
decls["anyOf"].sort(key=lambda r: "CustomElement" not in r["$ref"])
assert "CustomElement" in decls["anyOf"][0]["$ref"], "upstream declarations union changed"

# 2. schema.d.ts has CustomElementField (ClassField + attribute + reflects); schema.json lost it.
field = copy.deepcopy(defs["ClassField"])
field["description"] = "Additional metadata for fields on custom elements."
field["properties"]["attribute"] = {
    "type": "string",
    "description": "The corresponding attribute name if there is one.",
}
field["properties"]["reflects"] = {
    "type": "boolean",
    "description": "If the property reflects to an attribute.",
}
defs["CustomElementField"] = field
for name in ("CustomElementDeclaration", "CustomElementMixinDeclaration"):
    refs = defs[name]["properties"]["members"]["items"]["anyOf"]
    patched = [r for r in refs if r["$ref"] == "#/definitions/ClassField"]
    assert len(patched) == 1, f"upstream {name}.members changed"
    patched[0]["$ref"] = "#/definitions/CustomElementField"

# 3. Event.type is required upstream, but analyzer output for a `@fires` tag with no dispatch
#    (e.g. Shoelace) omits it. One missing field fails CustomElementDeclaration, and untagged
#    serde then falls back to ClassDeclaration, silently dropping the element.
defs["Event"]["required"].remove("type")
assert defs["Event"]["required"] == ["name"], "upstream Event.required changed"

# 4. holyfloe extension (D-063): the analyzer drops unknown JSDoc tags, so the holyfloe analyzer
#    plugin writes `@model <prop> <event> [<path>]` here. Without the property serde would drop it.
defs["HolyfloeModel"] = {
    "type": "object",
    "description": "A two-way binding from a `@model` JSDoc tag, written by the holyfloe analyzer plugin.",
    "properties": {
        "prop": {"type": "string"},
        "event": {"type": "string"},
        "value": {
            "type": "string",
            "description": "`detail` (default), `detail.<field>` or `target.<prop>`.",
        },
    },
    "required": ["prop", "event"],
}
assert "x-holyfloe-models" not in defs["CustomElementDeclaration"]["properties"]
defs["CustomElementDeclaration"]["properties"]["x-holyfloe-models"] = {
    "type": "array",
    "items": {"$ref": "#/definitions/HolyfloeModel"},
}

# 5. The analyzer writes Lit mixins (members with `attribute`/`reflects`, events) without
#    `customElement: true`. CustomElementMixinDeclaration requires it, so untagged serde fell back
#    to MixinDeclaration and dropped events, attributes and `reflects` (fixture fx-mixin's dep).
#    With it optional, every mixin parses as the superset, and MixinDeclaration leaves the union
#    (typify cannot build an enum from two variants where one contains the other).
mixin_required = defs["CustomElementMixinDeclaration"]["required"]
mixin_required.remove("customElement")
assert sorted(mixin_required) == ["kind", "name"], "upstream CustomElementMixinDeclaration changed"
before = len(decls["anyOf"])
decls["anyOf"] = [r for r in decls["anyOf"] if r["$ref"] != "#/definitions/MixinDeclaration"]
assert len(decls["anyOf"]) == before - 1, "upstream declarations union changed"

(here / "schema-2.1.0.patched.json").write_text(json.dumps(schema, indent=2) + "\n")
