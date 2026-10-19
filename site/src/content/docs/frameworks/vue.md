---
title: Vue
description: Vue 3 defineComponent wrappers with v-model and named slots.
---

Each element gets a `defineComponent` wrapper (a `.ts` file, not an SFC):

```vue
<script setup lang="ts">
import { ref } from 'vue';
import { FxDialog } from './generated/vue';

const open = ref(false);
</script>

<template>
  <FxDialog v-model:open="open" @open-change="(e) => console.log(e.detail.open)">
    Body
    <template #footer><button @click="open = false">OK</button></template>
  </FxDialog>
</template>
```

- **Props** are typed from the element class. Strings, numbers and booleans render as attributes (and in the server HTML); objects, arrays and props without an attribute are always set as properties, even before the element is defined. Props you don't pass are left out, so the element keeps its defaults.
- **Events** are emitted under their PascalCase base name: listen with `@open-change` or `onOpenChange`. The payload is the typed event.
- **`v-model`**: a binding on `value` is plain `v-model`; any other prop `p` is `v-model:p`. See [two-way binding](/holyfloe/guides/documenting-components/#two-way-binding) for how bindings are found.
- **Slots**: the default slot goes to the element's children, and named slots are Vue slots (`#footer`), with the slot name unchanged (`#help-text`).
- **Element and methods**: the component instance has `element` and one forwarding function per public method, all typed:

```vue
<FxDialog ref="dialog" />
<!-- dialog.value?.show() or dialog.value?.element -->
```

## Raw tags

`global-components.d.ts` types the plain tags (`<fx-dialog>`) for templates. If you use raw tags instead of the wrappers, tell Vue they are custom elements:

```ts
vue({ template: { compilerOptions: { isCustomElement: (tag) => tag.includes('-') } } })
```

Prefer the wrappers: raw `v-model` only works for elements that behave like a text input, and object props on an element defined after render become attributes.

## Server rendering

Nuxt 4 renders the wrappers on the server and hydrates them without warnings. String, number and boolean props are server-rendered attributes; property-only props and listeners are applied on the client. Deep Lit SSR is not supported.
