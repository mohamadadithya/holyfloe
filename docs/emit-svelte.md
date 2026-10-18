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
   - Primitive: in markup (`{variant}`); renamed attributes use the attribute name. Svelte skips an attribute that starts `undefined`, so the element keeps its default.
   - Boolean: in markup as `x={x || undefined}` (SSR and pre-upgrade: an attribute can only be present or absent), and also set as a property in `$effect`, so `false` reaches the element as `false`, not `undefined` (review e2e 2026-10-18).
   - Complex, and props without an attribute: set in the same `$effect`, skipping `undefined` (`if (element && items !== undefined) element.items = items`). Svelte 5 has no `prop:` or `attr:` forcing syntax (sveltejs/svelte#15455).
   - No `whenDefined` re-apply: Lit keeps properties set before the upgrade (to be proven by fx-lazy).
5. Events: callback props are `on<base_name>` (`onMyChange`, `onFxPress`), the same names as React and Vue (D-066). A mixed rule (exact DOM name when it is an identifier) would give `onmyChange` next to `onMyChange`.
   - Attach with an event attribute on the host, `onmy-change={(e) => onMyChange?.(e)}`. Svelte attaches it when it creates the element, so events fired in Lit's first update are not lost (a `$effect` would run later).
   - Names Svelte delegates to the root (`change`, `click`, `input` and the rest of its list; Svelte 5.57.2 `DELEGATED_EVENTS`) only arrive if the event bubbles, and `new Event('change')` does not. Those, and names that end in `capture` (which Svelte would read as a capture flag), use the capture form `onchangecapture`: a capture listener on the host fires at the target either way (D-075, fixture `fx-input`).
   - The props type omits both `on<base_name>` and the exact `on<dom_name>` key, so `onchange` cannot be passed through `...rest` next to `onChange`.
   - Raw tags differ: Svelte 5 event attributes are case-sensitive and exact (`onclick` listens to `click`), so `svelte-elements.d.ts` types `onmy-change`.
6. Two-way binding: for each `TwoWayBinding` emit `let { value = $bindable() } = $props();` and a listener that writes `value = <read via ValuePath>`. Consumers get `bind:value`.
7. Slots to snippets:
   - Default: `{@render children?.()}` inside the element, for every component, documented or not (React passes children through too; dropping them silently would surprise).
   - Named slot `foo`: snippet prop `foo`, rendered as `<span slot="foo" style="display: contents">{@render foo?.()}</span>`. A snippet cannot carry a `slot` attribute itself, hence the wrapper. Snapshot-test this; it is the subtlest part (D-025).
8. Spread remaining props: `{...rest}` onto the element for global attributes (`class`, `id`, `aria-*`).
9. Expose the element through a bindable `element` prop: `let { element = $bindable() } = $props()` and `bind:this={element}`; consumers write `bind:element={el}` (D-064).

## Types

- `svelte-elements.d.ts` augments `SvelteHTMLElements` with each tag's props and event attributes.
- Event attribute types use Svelte 5 form (`onmy-change`), not `on:`.
- A renamed attribute is typed under its attribute name too (`'badge-count'`): for a raw tag, the attribute is what works before the element upgrades and in SSR. The wrappers' own markup is checked against these types when both are in one program.

## Acceptance (per fixture)

- `svelte-check --fail-on-warnings` passes on generated output plus consumer files.
- `bind:value` round trip e2e passes.
- Named slot renders in the right shadow slot (e2e asserts `assignedNodes()`).
- Playwright svelte5 and sveltekit apps: props, events, bind, slots.

## Raw tags without the wrapper (D-077, e2e `svelte5?raw`)

- Before the element is defined, Svelte sets objects and arrays as properties (they survive the upgrade) and everything else, functions included, as attributes (a function is lost). After definition it uses the element's setters. There is no `prop:`/`attr:` syntax: svelte-check reads `prop:items` as an attribute with an illegal colon (a warning).
- `bind:value` on a custom element is a compile error in Svelte 5.57.2, so raw two-way binding is impossible; the wrappers' `$bindable` props are the way.
