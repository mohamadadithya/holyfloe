# Open questions (UNVERIFIED: do not build on these)

Each item has an owner step and a command or test that closes it. When closed, move the answer to `docs/decisions.md` and delete the item here.

## Version facts (close in M0)

- OQ-007: TypeScript 7.1 API dates in the brief (RC planned Nov 10, stable planned Nov 24, 2026) came from secondary reporting and are plans, not facts. Irrelevant to M0-M3 because D-013 forbids a checker. Revisit only if D-013 is ever reopened.

## Behavior claims (close with tests)

- OQ-003 (M2): Vue's native `v-model` on a custom element does not give typed or reliable two-way binding without a wrapper. This was an inference; the Vue docs do not cover it. Close with an e2e test using a raw `<fx-input v-model="x">`.

- OQ-004 (M2): Svelte 5 property-vs-attribute behavior for lazily defined elements, and the absence of `prop:`/`attr:` syntax. Evidence is mostly issues and community posts. Close with an e2e test and svelte-check. The e2e result overrides any prose.

- OQ-008 (M2, moved from M1 by D-061): React 19 behavior for properties on elements that are not upgraded at render time. The documented heuristic is "set as property if the property exists on the instance, else attribute". Close with a lazy-define e2e (fixture `fx-lazy`).

- OQ-009 (M2): `@wc-toolkit/svelte-types` README still shows `on:` syntax. Confirm what its output looks like on Svelte 5 and whether a newer release fixes it before porting.

## Design choices still to make (close in the milestone plan)

- OQ-016 (M2): `holyfloe-cem` drops fields the schema does not define. A custom analyzer plugin for `@model` (D-026) would lose its data unless it writes into an existing field, or the types gain a catch-all map. Decide with OQ-010.

- OQ-010 (M2): Final `@model` JSDoc syntax and config key. Proposal: `@model <prop> <event>` with optional `detail.<field>`. Decide in the M2 plan, then record in decisions (D-026).
- OQ-011 (M2): Svelte: expose the element through a bindable `element` prop or a `getElement()` export. Pick one.