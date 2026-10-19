# Roadmap with exit commands

Rules: one milestone at a time. Plan mode first. Done means every exit command passes. Time estimates are rough planning figures for one experienced developer.

## M0: Scaffolding (about 1 week)

Scope
- Cargo workspace with all crate stubs from `CLAUDE.md`.
- Resolve and record pinned versions (see `docs/open-questions.md` OQ-001, OQ-002).
- Generate `holyfloe-cem` from the CEM 2.1.0 JSON Schema with typify; check it in.
- Write the fixture corpus and expected outputs (`fixtures/README.md`) BEFORE any emitter.
- `justfile` with the commands in `CLAUDE.md`. CI running `just verify`.
- Spike: confirm minijinja version API with a 20-line render test (closes D-032).

Exit commands
```
cargo build --workspace
cargo test -p holyfloe-cem                 # round-trips every fixtures/src/*/custom-elements.json
cargo clippy --workspace --all-targets -- -D warnings
just gen-fixtures-cem                   # runs the CEM analyzer on fixtures/src, output committed
```

## M1: CEM -> IR -> React (about 3 weeks)

Scope
- `holyfloe-ir`: data model and all six passes from `docs/ir.md`.
- `holyfloe-emit-core`: minijinja env and filters.
- `holyfloe-emit-react`: `wrapper` and `types` modes per `docs/emit-react.md`.
- `holyfloe-cli`: `holyfloe generate --cem <path> --out <dir> --frameworks react`.
- `holyfloe-fs`: write-if-changed.

Libraries: CEM analyzer, typify output, serde, minijinja, insta, clap.

Exit commands
```
cargo test --workspace                  # includes insta snapshots for every fixture x react
just gen-fixtures                       # regenerates fixtures/out/react; git diff must be empty
just typecheck-react                    # tsc --strict against @types/react 18 and 19
just e2e-react                          # Playwright react18 + react19 (vite dev and prod) + next-app: props, events, ref, children, SSR HTML
```

## M2: Vue 3 and Svelte 5 (about 3 weeks)

Scope
- `holyfloe-emit-vue`, `holyfloe-emit-svelte` (templates only). Each framework emitter also renders its raw-tag type augmentation; `holyfloe-emit-types` was removed (D-068).
- `TwoWayBinding` end to end: `@model` JSDoc convention finalized and recorded in decisions (closes D-026), v-model, `$bindable`.
- Snippet-to-slot mapping for Svelte.
- Resolve OQ-003 and OQ-004 with e2e tests, then update the emitter docs.

Exit commands
```
cargo test --workspace                  # snapshots for fixture x {react,vue,svelte}
just gen-fixtures                       # git diff must be empty
just typecheck-vue                      # vue-tsc --noEmit
just typecheck-svelte                   # svelte-check --fail-on-warnings
just e2e-vue                            # includes v-model round trip, named slots
just e2e-svelte                         # includes bind:value round trip, slot assignedNodes()
```

## M3: Distribution and watch (about 2 weeks)

Scope
- CLI `--watch` using notify + debouncer.
- `holyfloe-napi` with napi-rs v3: `generate()`, `watch()`, `version()`. Native targets plus `wasm32-wasip1-threads`.
- `packages/core` npm wrapper, `packages/unplugin` calling core on `buildStart` and `watchChange`, writing through `holyfloe-fs` only when content changes.
- Standalone binaries via the distribution tool pinned in OQ-001.

Exit commands
```
cargo test --workspace
pnpm -C packages/core test              # loads native binary, runs generate() on fixtures
pnpm -C packages/unplugin test          # Vite fixture app regenerates on source edit, HMR fires
just verify
```

## M4: Native Lit parser (about 4 weeks, optional)

Scope
- `holyfloe-lit` using oxc: `@customElement`, `customElements.define`, `@property({type, attribute, reflect})`, `static properties`, skip `@state` and `@query`, JSDoc `@slot @fires @event @csspart @cssprop @cssproperty`, `dispatchEvent(new CustomEvent<T>(...))`, `static formAssociated`, mixins and superclasses via oxc_resolver.
- Types stay verbatim text plus import specifiers (D-013).
- Analyzer remains available through `--input cem`.

Exit commands
```
cargo test -p holyfloe-lit
just parity                             # IR from oxc == IR from analyzer on every fixture (diff test)
just verify
```

## Edge cases: added in the milestone where they first appear

Each edge case gets a fixture and a cross-framework e2e test when first hit. List and owners are in `fixtures/README.md`.

## Definition of done for any task

1. Exit commands for the current milestone pass.
2. New behavior has a fixture or test.
3. Docs updated in the same commit if behavior changed.
4. `docs/decisions.md` updated if a choice was made.
