"""Patch the upstream CEM 2.1.0 schema so typify output is lossless (D-046).

schema-2.1.0.json is the untouched upstream file. Run from crates/holyfloe-cem:
    python3 schema/patch.py && cargo typify --no-builder schema/schema-2.1.0.patched.json -o src/generated.rs && cargo fmt
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
    for ref in defs[name]["properties"]["members"]["items"]["anyOf"]:
        if ref["$ref"] == "#/definitions/ClassField":
            ref["$ref"] = "#/definitions/CustomElementField"

(here / "schema-2.1.0.patched.json").write_text(json.dumps(schema, indent=2) + "\n")
