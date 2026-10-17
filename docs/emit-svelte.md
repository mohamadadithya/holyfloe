# Svelte 5 emitter spec

Reference for type shapes: `@wc-toolkit/svelte-types` (its README still shows `on:` event syntax; update to Svelte 5 attribute events). Port shapes, write original code.

## Output

```
generated/svelte/
  index.ts
  MyButton.svelte
  MyButton.svelte.d.ts   # if needed by svelte-check
  svelte-elements.d.ts   # SvelteHTMLElements augmentation for raw tags
```

## Component rules

1. Real `.svelte` files using runes. `peerDependencies: { "svelte": "^5" }`.
2. Props via `let { variant, items, children, ...rest } = $props();`.
3. Binding to the element: `bind:this={el}`.
4. Props by `PropKind`:
   - Primitive and Boolean: pass in markup as attributes (`<my-button {variant} ...>`).
   - Complex: set in `$effect(() => { if (el) el.items = items; })`. Svelte 5 has no `prop:` or `attr:` forcing syntax (sveltejs/svelte#15455), so this is the supported route.
   - Re-apply after `customElements.whenDefined(tag)`.
5. Events: Svelte 5 event attributes are case-sensitive (`onclick` listens to `click`, `onClick` to `Click`).
   - Generate callback props named from the exact DOM event name when it is a valid identifier (`onmy-change` is not, so use the normalized `onMyChange`).
   - Attach with `on(el, dom_name, handler)` from `svelte/events` inside `$effect`, returning the cleanup.
   - Document the casing rule in the generated file header.
6. Two-way binding: for each `TwoWayBinding` emit `let { value = $bindable() } = $props();` and a listener that writes `value = <read via ValuePath>`. Consumers get `bind:value`.
7. Slots to snippets:
   - Default: `{@render children?.()}` inside the element.
   - Named slot `foo`: snippet prop `foo`, rendered as `<span slot="foo" style="display: contents">{@render foo?.()}</span>`. A snippet cannot carry a `slot` attribute itself, hence the wrapper. Snapshot-test this; it is the subtlest part (D-025).
8. Spread remaining props: `{...rest}` onto the element for global attributes (`class`, `id`, `aria-*`).
9. Expose the element: `export function getElement()` or a bindable `element` prop (choose one in M2 plan and record in decisions).

## Types

- `svelte-elements.d.ts` augments `SvelteHTMLElements` with each tag's props and event attributes.
- Event attribute types use Svelte 5 form (`onmy-change`), not `on:`.

## Acceptance (per fixture)

- `svelte-check --fail-on-warnings` passes on generated output plus consumer files.
- `bind:value` round trip e2e passes.
- Named slot renders in the right shadow slot (e2e asserts `assignedNodes()`).
- Playwright svelte5 and sveltekit apps: props, events, bind, slots.

## Unverified (see open-questions.md)

- OQ-004: exact property-vs-attribute heuristic Svelte 5 uses for lazily defined elements. The e2e suite is the source of truth, not older community write-ups.
