---
title: Svelte
description: Svelte 5 components with runes, bind:, and snippets for slots.
---

Each element gets a real `.svelte` component using runes:

```svelte
<script lang="ts">
  import { FxDialog } from './generated/svelte';
  import type { FxDialog as FxDialogElement } from 'my-lib/fx-dialog.js';

  let open = $state(false);
  let element = $state<FxDialogElement>();
</script>

<FxDialog bind:open bind:element onOpenChange={(e) => console.log(e.detail.open)}>
  Body
  {#snippet footer()}<button onclick={() => element?.show()}>OK</button>{/snippet}
</FxDialog>
```

- **Props** are typed from the element class. Strings, numbers and booleans render as attributes (and in the server HTML); objects, arrays and props without an attribute are set as properties in an effect. Props you don't pass are left out, so the element keeps its defaults.
- **Events** are `on<Name>` callback props (`onOpenChange`), the same names as React. Listeners are attached when the element is created, so early events are not lost.
- **`bind:`**: each binding becomes a `$bindable` prop, so `bind:open` or `bind:value` works. See [two-way binding](/holyfloe/guides/documenting-components/#two-way-binding).
- **Slots** become snippets: children go to the default slot, and a named slot is a snippet prop named after it as an identifier (`help-text` → `helpText`). If that name is already a prop, `Snippet` is appended (`labelSnippet`).
- **`bind:element`** gives you the custom element, to call its methods.
- Other attributes (`class`, `id`, `aria-*`) are spread onto the element.

## Raw tags

`svelte-elements.d.ts` types the plain tags for svelte-check. Svelte 5 event attributes on raw tags use the exact DOM name (`onopen-change`). Prefer the wrappers: `bind:value` on a raw custom element is a compile error in Svelte 5, and functions passed before the element is defined are lost.

## Server rendering

SvelteKit renders the wrappers on the server and hydrates them without warnings. String, number and boolean props are server-rendered attributes; property-only props are set on the client. Deep Lit SSR is not supported.
