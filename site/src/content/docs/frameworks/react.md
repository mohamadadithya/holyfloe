---
title: React
description: React 18 and 19 wrappers, or React 19 types for the plain tags.
---

holyfloe has two React modes, chosen with `react.mode` in the [config](/holyfloe/reference/configuration/#react) or `--react-mode`.

## Wrapper mode (React 18 and 19)

The default. Each element gets a component:

```tsx
import { FxDialog } from './generated/react';

<FxDialog ref={ref} open={open} onOpenChange={(e) => setOpen(e.detail.open)}>
  Body
  <button slot="footer">OK</button>
</FxDialog>
```

- **Props** are typed from the element class (`FxDialogElement['open']`). Strings, numbers and booleans render as attributes, so they are in the server HTML; objects, arrays and functions are set as properties after mount, and set again once the element is defined.
- **Events** are `on<Name>` props (`open-change` gives `onOpenChange`), typed `(e: TypedEvent<FxDialogElement, Detail>) => void`, where `e.target` is the element. Listeners are attached before Lit's first update, so early events are not lost, and changing the handler does not re-subscribe.
- **Refs** point at the custom element, so you can call its methods (`ref.current?.show()`). `react.ref: "forwardRef"` (default) uses `forwardRef`; `"prop"` (React 19 only) takes `ref` as a plain prop.
- **Children** go to the default slot. Named slots use `slot="name"` on a child.
- **`style`** accepts the component's CSS custom properties, typed (`'--fx-color'?: string`).
- Read-only props (getters without setters) are left out, since assigning them throws.

Every file starts with `'use client'`; turn it off with `react.useClient: false`.

## Types mode (React 19 only)

No runtime code: holyfloe only adds the tags to `JSX.IntrinsicElements`, so you use the elements directly and React 19 handles properties.

```tsx
<fx-dialog open={open} onopen-change={(e) => setOpen(e.detail.open)} />
```

Event props follow React 19's convention, `on` + the exact DOM event name, so `open-change` is typed as `onopen-change`.

Limits of types mode, which wrapper mode does not have:

- An element defined after React renders it loses object and array props: before the upgrade React writes them as attributes.
- A prop whose attribute is renamed (`count` → `badge-count`) is rendered on the server as `count="3"`, which Lit ignores until hydration.

## Server rendering

Wrappers render in Next 16 App Router server components. The server HTML has the string, number and boolean attributes; the wrappers then hydrate and set properties and listeners. Deep Lit SSR (shadow DOM in the server HTML) is not supported.

Next.js needs `"moduleResolution": "nodenext"` (and `"module": "nodenext"`) in `tsconfig.json`, because the wrappers import each other with `.js` paths.

Unsetting a prop (back to `undefined`) removes the attribute, and Lit then sets the property to `null` or `false`, not back to its default.
