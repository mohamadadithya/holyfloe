# Open questions (UNVERIFIED: do not build on these)

Each item has an owner step and a command or test that closes it. When closed, move the answer to `docs/decisions.md` and delete the item here.

## Version facts (close in M0)

- OQ-007: TypeScript 7.1 API dates in the brief (RC planned Nov 10, stable planned Nov 24, 2026) came from secondary reporting and are plans, not facts. Irrelevant to M0-M3 because D-013 forbids a checker. Revisit only if D-013 is ever reopened.

## Behavior claims (close with tests)

- OQ-003 (M2): Vue's native `v-model` on a custom element does not give typed or reliable two-way binding without a wrapper. This was an inference; the Vue docs do not cover it. Close with an e2e test using a raw `<fx-input v-model="x">`.

- OQ-004 (M2): Svelte 5 property-vs-attribute behavior for lazily defined elements, and the absence of `prop:`/`attr:` syntax. Evidence is mostly issues and community posts. Close with an e2e test and svelte-check. The e2e result overrides any prose.

- OQ-005 (M1): Next.js App Router with Lit deep SSR. The Lit nextjs README says deep SSR does not work inside server components; one older report says Next 13+ was unsupported. Behavior of `@lit-labs/ssr-react` 0.3.x and `@lit-labs/nextjs` 0.2.x on current Next is not confirmed. Close with the `next-app` e2e. Until then, only claim: wrappers carry `'use client'` and render primitive attributes.

- OQ-008 (M1): React 19 behavior for properties on elements that are not upgraded at render time. The documented heuristic is "set as property if the property exists on the instance, else attribute". Close with a lazy-define e2e (fixture `fx-lazy`).

- OQ-009 (M2): `@wc-toolkit/svelte-types` README still shows `on:` syntax. Confirm what its output looks like on Svelte 5 and whether a newer release fixes it before porting.

## Design choices still to make (close in the milestone plan)


- OQ-014 (M1): Module path to import specifier. CEM gives `src/fx-button/fx-button.ts`; expected output imports `@holyfloe/fixtures/fx-button/fx-button.js`, and `fixtures/package.json` `exports` points at `.js` files that do not exist. Decide the mapping (strip source root, `.ts` -> `.js`, match against `exports`) and whether fixtures are built or exported as `.ts`, so e2e bundlers can resolve them.
- OQ-015 (M1): Analyzer output quirks the IR must absorb, each with a test: `superclass.module` is `/src/x/fx-base.js` while `inheritedFrom.module` is `src/x/fx-base.ts` (normalize both); inherited members are already present with `inheritedFrom` (flatten must merge without duplicating); classes with `customElement: true` but no `tagName` (e.g. `FxBase`) are bases, not components; type text can be wrong (`renderLabel` loses parentheses), so it must never be re-emitted. Found in real manifests during step 2 and handled with tests (D-049, D-050): events without `type`; re-export paths relative to their module; definition exports that point at a barrel; one class name under two tags; tags with literal quotes; variables recorded as tags; bundled dependency code that is not schema-valid.
- OQ-016 (M2): `holyfloe-cem` drops fields the schema does not define. A custom analyzer plugin for `@model` (D-026) would lose its data unless it writes into an existing field, or the types gain a catch-all map. Decide with OQ-010.

- OQ-010 (M2): Final `@model` JSDoc syntax and config key. Proposal: `@model <prop> <event>` with optional `detail.<field>`. Decide in the M2 plan, then record in decisions (D-026).
- OQ-011 (M2): Svelte: expose the element through a bindable `element` prop or a `getElement()` export. Pick one.
- OQ-012 (M1): Whether React `ref` handling should be chosen by detecting the consumer's React version or by a config flag. Config flag is simpler; confirm in the M1 plan.
