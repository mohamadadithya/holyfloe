# holyfloe task runner. Recipes for later milestones fail loudly until they exist.

set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

build:
    cargo build --workspace

test:
    cargo test --workspace

clippy:
    cargo clippy --workspace --all-targets -- -D warnings

fmt:
    cargo fmt --all -- --check

# Compile fixtures/src to fixtures/dist (.js + .d.ts, git-ignored). Generated wrappers import from here.
build-fixtures:
    pnpm -C fixtures build

# Run the CEM analyzer on each fixture, and on the src/_dep package fx-mixin uses; output is committed.
gen-fixtures-cem:
    cd fixtures && for dir in src/[!_]*/; do \
        name=$(basename "$dir"); \
        pnpm exec cem analyze --globs "src/$name/*.ts" --outdir "src/$name" --quiet; \
    done
    cd fixtures/src/_dep && ../../node_modules/.bin/cem analyze --config ../../custom-elements-manifest.config.mjs --globs "src/*.ts" --quiet

# Regenerate typify output from the patched CEM schema (D-046). Not part of the build.
gen-cem-types:
    cd crates/holyfloe-cem && python3 schema/patch.py \
        && cargo typify --no-builder schema/schema-2.1.0.patched.json -o src/generated.rs \
        && { printf '%s\n' \
            '// Generated once by typify, checked in (D-031, D-046). Do not edit by hand.' \
            '// Regenerate with `just gen-cem-types`: schema/patch.py, then cargo-typify 0.8.0' \
            '// on schema-2.1.0.patched.json (upstream: npm custom-elements-manifest@2.1.0).' ''; \
            cat src/generated.rs; } > src/generated.rs.tmp && mv src/generated.rs.tmp src/generated.rs
    cargo fmt -p holyfloe-cem

# Generate every fixture x React mode into fixtures/out (committed), then diff against expected/.
# A fixture uses src/<name>/holyfloe.config.json when it has one, else fixtures/holyfloe.config.json.
# fx-button also goes out with `react.ref: "prop"` (React 19 only) into react-ref-prop/.
gen-fixtures: build-fixtures
    rm -rf fixtures/out/react fixtures/out/react-types fixtures/out/react-ref-prop
    stage=$(mktemp -d); trap 'rm -rf "$stage"' EXIT; \
    for dir in fixtures/src/[!_]*/; do \
        name=$(basename "$dir"); \
        cfg=fixtures/holyfloe.config.json; \
        if [ -f "$dir/holyfloe.config.json" ]; then cfg="$dir/holyfloe.config.json"; fi; \
        for mode in wrapper types; do \
            sub=react; if [ "$mode" = types ]; then sub=react-types; fi; \
            cargo run -q -p holyfloe-cli -- generate --cem "$dir/custom-elements.json" --out "$stage" \
                --frameworks react --config "$cfg" --react-mode "$mode" > /dev/null; \
            mkdir -p "fixtures/out/$sub"; mv "$stage/react" "fixtures/out/$sub/$name"; \
        done; \
    done; \
    cargo run -q -p holyfloe-cli -- generate --cem fixtures/src/fx-button/custom-elements.json \
        --out "$stage" --frameworks react --config fixtures/holyfloe.ref-prop.config.json > /dev/null; \
    mkdir -p fixtures/out/react-ref-prop; mv "$stage/react" fixtures/out/react-ref-prop/fx-button
    diff -r fixtures/expected/react fixtures/out/react
    diff -r fixtures/expected/react-types fixtures/out/react-types
    diff -r fixtures/expected/react-ref-prop fixtures/out/react-ref-prop

typecheck: typecheck-react

# tsc --strict on fresh fixtures/out + consumer tests: wrapper mode on @types/react 18 and 19,
# types mode and `react.ref: "prop"` on 19 only.
typecheck-react: gen-fixtures
    pnpm -C fixtures/typecheck/react18 typecheck
    pnpm -C fixtures/typecheck/react19 typecheck

typecheck-vue:
    @echo "not yet: M2" >&2; exit 1

typecheck-svelte:
    @echo "not yet: M2" >&2; exit 1

e2e: e2e-react

# Playwright (chromium): e2e/react18 and e2e/react19 under vite dev and build + preview, and
# e2e/next-app under next dev and build + start.
# Runs on fresh fixtures/out. First time: pnpm -C e2e exec playwright install chromium
e2e-react: gen-fixtures
    pnpm -C e2e test:react

e2e-vue:
    @echo "not yet: M2" >&2; exit 1

e2e-svelte:
    @echo "not yet: M2" >&2; exit 1

parity:
    @echo "not yet: M4" >&2; exit 1

# Everything that exists today, in order. Extend as milestones land.
# Committed analyzer output and generated wrappers must match what the tools produce now.
verify: fmt build clippy test gen-fixtures-cem gen-fixtures typecheck-react e2e-react
    git diff --exit-code -- fixtures/src fixtures/out
