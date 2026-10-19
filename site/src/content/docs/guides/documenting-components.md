---
title: Documenting components
description: The JSDoc tags holyfloe reads, and how two-way bindings are found.
---

holyfloe reads the same JSDoc as the [CEM analyzer](https://custom-elements-manifest.open-wc.org/analyzer/getting-started/). Descriptions end up as doc comments on the generated props, events and components.

| Tag | Meaning |
|---|---|
| `@fires {CustomEvent<T>} name - text` / `@event` | An event and its detail type. With `--lit`, `this.dispatchEvent(new CustomEvent<T>('name'))` works too. |
| `@slot [name] - text` | A slot (no name: the default slot). |
| `@csspart` / `@part`, `@cssprop` / `@cssproperty`, `@cssstate` | CSS parts, custom properties and states. |
| `@model prop event [path]` | A two-way binding (`v-model`, `bind:`). |
| `@deprecated` | Deprecation, carried to the wrapper. |
| `@internal`, `@private`, `@protected` | Hidden from wrappers. |
| `@tag name` | The tag, when there is no `@customElement` or `customElements.define`. |
| `@summary` | A short description. |

## Events

Each event gets a PascalCase base name: `open-change` gives `OpenChange`. The React and Svelte prop is `onOpenChange`; the Vue wrapper emits `OpenChange`, which templates listen to with `@open-change`.

The detail type comes from `CustomEvent<T>`. A bare `CustomEvent` or a missing type gives `unknown`. To rename an event's base name, use [`overrides.<tag>.events`](/holyfloe/reference/configuration/#overrides).

## Two-way binding

A binding is **inferred** for a writable prop named `value`, `checked`, `open` or `selected` when the component fires:

- `<prop>-change` or `<prop>Change`. The new value is read from the event detail if it has a typed `CustomEvent<T>`, else from the element's property.
- otherwise, for `value` and `checked` only, `change` or `input`, reading the element's property.

Turn inference off with `"bindings": "explicit-only"` in the [config](/holyfloe/reference/configuration/).

Declare a binding **explicitly** with `@model` on the class:

```ts
/**
 * @model open open-change detail.open
 */
```

The path says where the new value comes from: `detail` (the default), `detail.<field>`, or `target.<prop>` (a property of the element). The same can be set in config with [`overrides.<tag>.bindings`](/holyfloe/reference/configuration/#overrides). Explicit bindings replace inferred ones for the same prop.

React has no binding helper; use the event prop.

## With the CEM analyzer

The analyzer drops tags it does not know, so `@model` needs the holyfloe plugin to survive into the manifest:

```js title="custom-elements-manifest.config.mjs"
import holyfloeModels from '@holyfloe/core/cem-plugin';

export default { litelement: true, plugins: [holyfloeModels()] };
```

With `--lit` no plugin is needed.

## Types are never resolved

holyfloe does not run a TypeScript checker. Props are typed `Element['prop']` from an `import type` of your element class, so they stay exactly as you wrote them. Event detail types are copied as written; types they name are imported only when the manifest lists them in `type.references`.
