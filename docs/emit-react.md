# React emitter spec

Reference output shape: `@wc-toolkit/react-wrappers` 1.2.x (port the shape, write original code). Types-only mode follows `@wc-toolkit/jsx-types`.

## Modes

Config: `react: { mode: "wrapper" | "types" }`. Default `wrapper`.

### `wrapper` mode (React 18 and 19)

Output, one file per component (tree-shakeable) plus shared utils:

```
generated/react/
  index.ts                 # re-exports
  react-utils.ts           # shared helpers
  MyButton.tsx
  MyButton.d.ts            # only if emitting .js
```

Component rules:
1. First line `'use client';` when `react.useClient` is true (default).
2. Side-effect import that defines the element: `import '<pkg>/<module>';`
3. `import type { MyButton as MyButtonElement } from '<pkg>/<module>';`
4. Element props typed from the class: `MyButtonElement['variant']`. Never inline resolved types.
5. Props by `PropKind`:
   - **Boolean:** pass `true` as an empty attribute, omit when `false`. Set the property on the element in a layout effect after mount so updates work post-upgrade.
   - **Primitive:** pass as attribute in JSX (SSR-correct), and also assign the property in the layout effect when the element is upgraded.
   - **Complex:** never an attribute. Assign as a property in `useLayoutEffect`. Re-apply after `customElements.whenDefined(tag)` resolves.
6. Events: for each event emit an `on<PascalName>` prop. Attach with `addEventListener(dom_name, handler)` in an effect; clean up on unmount and when the handler changes. Handler type: `(e: TypedEvent<MyButtonElement, Detail>) => void` where `TypedEvent` makes `e.target` the element class.
7. Ref: forward to the custom element. For React 19 peer ranges, `ref` may be a normal prop; for 18, use `forwardRef`. Choose by `peerDependencies`, do not emit both.
8. Children: pass through unchanged. They become the default slot.
9. Named slots: consumers put `slot="name"` on child elements. Optionally emit `<MyButton.Slot name="icon">` helper (config `react.slotHelpers`, default off).
10. Style: accept `style` with CSS custom property keys typed from `css_props` (`'--my-color'?: string`).
11. Attribute renames: `class` -> `className`, `for` -> `htmlFor` (config `attributeMapping`).

Typing: `export type MyButtonProps = Partial<Pick<MyButtonElement, PublicPropNames>> & EventProps & Omit<React.HTMLAttributes<MyButtonElement>, ConflictingKeys>`.

### `types` mode (React 19 only)

Emit only an `IntrinsicElements` augmentation. No runtime code.

```ts
declare module 'react' {
  namespace JSX {
    interface IntrinsicElements {
      'my-button': React.DetailedHTMLProps<React.HTMLAttributes<MyButtonElement>, MyButtonElement> & MyButtonProps;
    }
  }
}
```

Event props must match React 19 convention: `on` + exact DOM event name, so event `my-change` is typed as `onmy-change`. Add a doc comment noting that camelCase event names are friendlier.

## SSR

- Default: no Lit SSR integration. Wrappers render attributes for Primitive and Boolean props only.
- `ssr: "lit"` (opt-in, experimental): document `@lit-labs/ssr-react` and `@lit-labs/nextjs`. Deep Lit SSR does not work inside React Server Components; keep `'use client'`.
- Do not claim Next.js App Router deep SSR works. See `docs/open-questions.md` OQ-005.

## Acceptance (per fixture)

- `tsc --noEmit --strict` passes against `@types/react` 18 and 19.
- Consumer test file compiles; a `@ts-expect-error` file with a wrong detail type fails to compile.
- Playwright react18 and react19 apps: props, events, ref, children tests pass.
