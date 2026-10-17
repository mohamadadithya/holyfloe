# Open questions (UNVERIFIED: do not build on these)

Each item has an owner step and a command or test that closes it. When closed, move the answer to `docs/decisions.md` and delete the item here.

## Version facts (close in M0)

- OQ-001: Resolve and pin versions. Sources conflicted or were secondary.
  ```
  npm view @custom-elements-manifest/analyzer version license
  npm view @wc-toolkit/react-wrappers version license
  npm view @wc-toolkit/vuejs-types version
  npm view @wc-toolkit/svelte-types version
  npm view @lit-labs/gen-wrapper-react version      # briefs said 0.3.4 vs 0.3.5
  npm view unplugin version engines
  npm view @napi-rs/cli version
  cargo search typify
  cargo search minijinja                             # brief said 3.0.0, license unconfirmed
  cargo search oxc_parser
  cargo search oxc_resolver                          # brief said 11.24.2 vs 11.24.3
  cargo search insta
  cargo search clap
  cargo search notify
  cargo search notify-debouncer-full
  ```
  Record results in the "Pinned versions" block of `CLAUDE.md`.

- OQ-013: Check that the project names are free before the first publish. Do this in M0, before creating crates.
  ```
  cargo search holyfloe                              # look for holyfloe, holyfloe-cli, holyfloe-ir, etc.
  npm view @holyfloe/core                            # 404 means free; also check the scope can be created
  npm view holyfloe
  ```
  If a name is taken, update the Naming table in `CLAUDE.md` first, then search-and-replace the docs.

- OQ-002: Confirm licenses and tool name for standalone binary distribution. The brief says `cargo-dist` was renamed to `dist` (v0.24.0 era) and reported v0.33.0 (2026-09-10); license unconfirmed. Check the project's README and LICENSE.

- OQ-006: Confirm the CEM schema 2.1.0 JSON file location and that typify can compile it without manual patching. Spike in M0. If typify fails, fall back to hand-written serde structs (about 300 lines).

- OQ-007: TypeScript 7.1 API dates in the brief (RC planned Nov 10, stable planned Nov 24, 2026) came from secondary reporting and are plans, not facts. Irrelevant to M0-M3 because D-013 forbids a checker. Revisit only if D-013 is ever reopened.

## Behavior claims (close with tests)

- OQ-003 (M2): Vue's native `v-model` on a custom element does not give typed or reliable two-way binding without a wrapper. This was an inference; the Vue docs do not cover it. Close with an e2e test using a raw `<fx-input v-model="x">`.

- OQ-004 (M2): Svelte 5 property-vs-attribute behavior for lazily defined elements, and the absence of `prop:`/`attr:` syntax. Evidence is mostly issues and community posts. Close with an e2e test and svelte-check. The e2e result overrides any prose.

- OQ-005 (M1): Next.js App Router with Lit deep SSR. The Lit nextjs README says deep SSR does not work inside server components; one older report says Next 13+ was unsupported. Behavior of `@lit-labs/ssr-react` 0.3.x and `@lit-labs/nextjs` 0.2.x on current Next is not confirmed. Close with the `next-app` e2e. Until then, only claim: wrappers carry `'use client'` and render primitive attributes.

- OQ-008 (M1): React 19 behavior for properties on elements that are not upgraded at render time. The documented heuristic is "set as property if the property exists on the instance, else attribute". Close with a lazy-define e2e (fixture `fx-lazy`).

- OQ-009 (M2): `@wc-toolkit/svelte-types` README still shows `on:` syntax. Confirm what its output looks like on Svelte 5 and whether a newer release fixes it before porting.

## Design choices still to make (close in the milestone plan)

- OQ-010 (M2): Final `@model` JSDoc syntax and config key. Proposal: `@model <prop> <event>` with optional `detail.<field>`. Decide in the M2 plan, then record in decisions (D-026).
- OQ-011 (M2): Svelte: expose the element through a bindable `element` prop or a `getElement()` export. Pick one.
- OQ-012 (M1): Whether React `ref` handling should be chosen by detecting the consumer's React version or by a config flag. Config flag is simpler; confirm in the M1 plan.
