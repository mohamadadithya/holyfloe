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

(here / "schema-2.1.0.patched.json").write_text(json.dumps(schema, indent=2) + "\n")
