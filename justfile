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

# Run the CEM analyzer on each fixture; output is committed.
gen-fixtures-cem:
    cd fixtures && for dir in src/*/; do \
        name=$(basename "$dir"); \
        pnpm exec cem analyze --globs "src/$name/*.ts" --outdir "src/$name" --quiet; \
    done

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

gen-fixtures:
    @echo "not yet: M1" >&2; exit 1

typecheck:
    @echo "not yet: M1" >&2; exit 1

typecheck-react:
    @echo "not yet: M1" >&2; exit 1

typecheck-vue:
    @echo "not yet: M2" >&2; exit 1

typecheck-svelte:
    @echo "not yet: M2" >&2; exit 1

e2e:
    @echo "not yet: M1" >&2; exit 1

e2e-react:
    @echo "not yet: M1" >&2; exit 1

e2e-vue:
    @echo "not yet: M2" >&2; exit 1

e2e-svelte:
    @echo "not yet: M2" >&2; exit 1

parity:
    @echo "not yet: M4" >&2; exit 1

# Everything that exists today, in order. Extend as milestones land.
verify: fmt build clippy test gen-fixtures-cem
    git diff --exit-code -- fixtures/src
