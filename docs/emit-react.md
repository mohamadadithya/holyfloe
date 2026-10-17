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
2. Side-effect import that defines the element: `import '<definition.specifier>';`. This is the module holding the `custom-element-definition` export, which can be a separate `define` file (Lion).
3. `import type { MyButton as MyButtonElement } from '<module.specifier>';`. This is the declaring module, or an exported module that re-exports the class.
4. Element props typed from the class: `MyButtonElement['variant']`. Never inline resolved types.
5. Props by `PropKind`:
   - **Boolean:** pass `true` as an empty attribute, omit when `false`. Set the property on the element in a layout effect after mount so updates work post-upgrade.
   - **Primitive:** pass as attribute in JSX (SSR-correct), and also assign the property in the layout effect when the element is upgraded.
   - Unsetting a Primitive or Boolean prop (value -> `undefined`) removes the attribute, and Lit then sets the property to `null` (Primitive) or `false` (Boolean), not back to its default. This is documented behavior, not reset by the wrapper.
   - **Complex:** never an attribute. Assign as a property in `useLayoutEffect`. Re-apply after `customElements.whenDefined(tag)` resolves.
   - **No attribute** (`attribute: None`, Lit `attribute: false`): property only, whatever the kind. Never render it as an attribute (fixture `fx-badge` `label`).
   - **Read-only** (`readonly: true`, a getter without a setter): not a prop at all. Leave it out of `Pick`, destructuring and `useProperties`, because assigning it throws a `TypeError` (fixture `fx-badge` `overflow`).
6. Events: for each event emit an `on<base_name>` prop (`base_name` already has config overrides applied). Attach one stable listener per event with `addEventListener(dom_name, …)` in a layout effect (not `useEffect`: Lit's first update runs before passive effects, so events fired there would be lost). The listener calls the latest handler from a ref, so changing the handler never re-subscribes. Clean up on unmount. Handler type: `(e: TypedEvent<MyButtonElement, Detail>) => void` where `TypedEvent` makes `e.target`/`e.currentTarget` the element class. `Detail` is `T` only when the CEM type is `CustomEvent<T>` (or a `@fires {CustomEvent<T>}` tag); a bare `CustomEvent` or a missing type gives `unknown`, never `null`. The detail type text is the one place CEM type text is re-emitted (there is no class member to index into); if it names other types, they compile only when the CEM lists them in `type.references`, which become `import type` lines.
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

Base props: `Omit<DetailedHTMLProps<HTMLAttributes<El>, El>, OwnKeys> & Props`, where `OwnKeys` are all prop, event and `style` keys the component declares (same rule as wrapper mode: the emitter cannot know which built-in attributes clash, so it omits all of its own).

Limitation: React 19 renders attributes from prop names, so a prop whose attribute is renamed (`count` -> `badge-count`) is rendered on the server as `count="3"`, which Lit ignores until hydration sets the property. Use wrapper mode when renamed attributes must be correct in SSR HTML.

## SSR

- Default: no Lit SSR integration. Wrappers render attributes for Primitive and Boolean props only.
- `ssr: "lit"` (opt-in, experimental): document `@lit-labs/ssr-react` and `@lit-labs/nextjs`. Deep Lit SSR does not work inside React Server Components; keep `'use client'`.
- Do not claim Next.js App Router deep SSR works. See `docs/open-questions.md` OQ-005.

## Acceptance (per fixture)

- `tsc --noEmit --strict` passes against `@types/react` 18 and 19.
- Consumer test file compiles; a `@ts-expect-error` file with a wrong detail type fails to compile.
- Playwright react18 and react19 apps: props, events, ref, children tests pass.
