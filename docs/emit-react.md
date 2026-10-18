# React emitter spec

Reference output shape: `@wc-toolkit/react-wrappers` 1.2.x (port the shape, write original code). Types-only mode follows `@wc-toolkit/jsx-types`.

## Modes

Config: `react: { mode: "wrapper" | "types", useClient: boolean, ref: "forwardRef" | "prop" }`. Defaults `wrapper`, `true`, `forwardRef`. Any other `react` key is a config error, so typos and keys that are not supported yet (`slotHelpers`, `attributeMapping`) fail instead of doing nothing.

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
   - Imports between generated files use `.js` specifiers (D-051). Next 16 with Turbopack needs tsconfig `"module": "nodenext"` and `"moduleResolution": "nodenext"` to resolve them; under `"bundler"`, the create-next-app default, the build fails.
2. Side-effect import that defines the element: `import '<definition.specifier>';`. This is the module holding the `custom-element-definition` export, which can be a separate `define` file (Lion).
3. `import type { <export_name> as MyButtonElement } from '<module.specifier>';`. This is the declaring module, or an exported module that re-exports the class. `export_name` is usually the class name, and `default` for `export default class` (Shoelace, Web Awesome; fixture `fx-card`).
4. Element props typed from the class: `MyButtonElement['variant']`. Never inline resolved types.
5. Props by `PropKind`:
   - **Boolean:** pass `true` as an empty attribute, omit when `false`. Set the property on the element in a layout effect after mount so updates work post-upgrade.
   - **Primitive:** pass as attribute in JSX (SSR-correct), and also assign the property in the layout effect when the element is upgraded.
   - On the React 19 client, React itself assigns any prop the element already has (`'kind' in el`) as a property instead of writing the attribute. So after a client render, a non-reflected Primitive (`fx-deprecated` `kind`, `fx-derived` `label`) has the right property but no attribute, while React 18 writes both. Reflected props show the attribute on both, because Lit reflects them. Renamed attributes (`'badge-count'`) are not element properties, so React 19 writes them as attributes too. Verified in Chromium on 2026-10-18. Server HTML from wrapper mode has the attribute for every set Primitive and Boolean prop, non-reflected ones included (`fx-derived` `label="L"`, e2e `next-app`).
   - Unsetting a Primitive or Boolean prop (value -> `undefined`) removes the attribute, and Lit then sets the property to `null` (Primitive) or `false` (Boolean), not back to its default. This is documented behavior, not reset by the wrapper.
   - **Complex:** never an attribute. Assign as a property in `useLayoutEffect`. Re-apply after `customElements.whenDefined(tag)` resolves.
   - **No attribute** (`attribute: None`, Lit `attribute: false`): property only, whatever the kind. Never render it as an attribute (fixture `fx-badge` `label`).
   - **Read-only** (`readonly: true`, a getter without a setter): not a prop at all. Leave it out of `Pick`, destructuring and `useProperties`, because assigning it throws a `TypeError` (fixture `fx-badge` `overflow`).
   - **Every public field is a prop.** The manifest decides; the emitter does not guess at internals. Use `overrides.<tag>.excludeProps` to drop one, e.g. a stale JSDoc `@property` the class does not have (the `Pick` would fail) or a public internal (Web Awesome `didSSR`).
   - **Local names:** props are destructured into locals of the same name, unless the name is a reserved word or a local the wrapper binds itself (`ref`, `rest`, `forwardedRef`, the React and util imports). Those bind `<name>_`: `{ for: for_ }`, then `for: for_` in `useProperties` and `createElement` (fixture `fx-badge` `for`; Web Awesome `for`, `case`).
6. Events: for each event emit an `on<base_name>` prop (`base_name` already has config overrides applied). Attach one stable listener per event with `addEventListener(dom_name, …)` in a layout effect (not `useEffect`: Lit's first update runs before passive effects, so events fired there would be lost). The listener calls the latest handler from a ref, so changing the handler never re-subscribes. A non-read-only prop with the same name as an event prop (an `onFxReady` callback next to event `fx-ready`) would be destructured twice, so it is an `EmitError::PropEventClash` (`E013`); rename the event with `overrides.<tag>.events` or drop the prop with `excludeProps`. Clean up on unmount. Handler type: `(e: TypedEvent<MyButtonElement, Detail>) => void` where `TypedEvent` makes `e.target`/`e.currentTarget` the element class. `Detail` is `T` only when the CEM type is `CustomEvent<T>` (or a `@fires {CustomEvent<T>}` tag); a bare `CustomEvent` or a missing type gives `unknown`, never `null`. The detail type text is the one place CEM type text is re-emitted (there is no class member to index into); if it names other types, they compile only when the CEM lists them in `type.references`, which become `import type` lines. Because the text is verbatim, an imported name cannot be renamed: one that is imported from two modules, or that the generated file already uses (`Ref`, `TypedEvent`, `<Name>Element`, ...), is an `EmitError::DetailImportClash` (`E012`) instead of output that does not compile.
7. Ref: forward to the custom element. Config `react.ref`: `"forwardRef"` (default, React 18 and 19) wraps the component in `forwardRef`; `"prop"` (React 19 only) emits a plain function taking `ref?: Ref<El>` as a prop. One style per run, never both.
7a. Hydration: the host element gets `suppressHydrationWarning: true` before `...rest` (D-052). Lit reflects defaults when the element upgrades, which happens before React hydrates, so the server HTML lacks attributes the client DOM has. Only the host's own attributes are exempt; children are still checked.
8. Children: pass through unchanged. They become the default slot.
9. Named slots: consumers put `slot="name"` on child elements. Optionally emit `<MyButton.Slot name="icon">` helper (config `react.slotHelpers`, not supported yet: the key is rejected).
10. Style: accept `style` with CSS custom property keys typed from `css_props` (`'--my-color'?: string`).
11. Attribute renames: `class` -> `className`, `for` -> `htmlFor` (config `attributeMapping`, not supported yet: the key is rejected).

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
- Next 16 App Router (D-056, e2e `next-app`): server component pages can render the wrappers. The server HTML has Primitive and Boolean attributes, then the wrappers hydrate and set properties and listeners. Needs tsconfig `nodenext` (D-051).
- `ssr: "lit"` (opt-in, experimental, not supported yet, D-061): document `@lit-labs/ssr-react` and `@lit-labs/nextjs`. Deep Lit SSR does not work inside React Server Components; keep `'use client'`.
- Do not claim Next.js App Router deep SSR (shadow DOM in the server HTML) works; it is not tested.

## Acceptance (per fixture)

- `tsc --noEmit --strict` passes against `@types/react` 18 and 19.
- Consumer test file compiles; a `@ts-expect-error` file with a wrong detail type fails to compile.
- Playwright react18 and react19 apps: props, events, ref, children tests pass.
