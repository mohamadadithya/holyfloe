# Open questions (UNVERIFIED: do not build on these)

Each item has an owner step and a command or test that closes it. When closed, move the answer to `docs/decisions.md` and delete the item here.

## Version facts (close in M0)

- OQ-007: TypeScript 7.1 API dates in the brief (RC planned Nov 10, stable planned Nov 24, 2026) came from secondary reporting and are plans, not facts. Irrelevant to M0-M3 because D-013 forbids a checker. Revisit only if D-013 is ever reopened.

## Behavior claims (close with tests)

- OQ-009 (M2): `@wc-toolkit/svelte-types` README still shows `on:` syntax. Confirm what its output looks like on Svelte 5 and whether a newer release fixes it before porting.
