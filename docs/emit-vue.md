# Vue 3 emitter spec

Reference for type shapes: `@wc-toolkit/vuejs-types`. Reference for v-model design: Stencil's `componentModels` (`elements`, `event`, `targetAttr`, optional `externalEvent`). Port shapes, write original code.

## Output

```
generated/vue/
  index.ts
  MyButton.ts            # defineComponent wrapper (not an SFC)
  global-components.d.ts # GlobalComponents augmentation for raw tags
```

## Component rules

1. Use `defineComponent` with a render function (`h(tag, ...)`). Do not emit `.vue` SFCs (D-023).
2. Side-effect import that defines the element, plus `import type` of the element class.
3. Props: declare each public prop with `PropType<MyButtonElement['x']>` (non-Boolean: `type: null`, no runtime check, because type text can be any union).
   - Boolean: `{ type: Boolean, default: undefined }` so absent stays absent; rendered as `x: props.x ? '' : undefined`. Vue sets `''` as `true` on an upgraded element, and SSR writes `x`.
   - Primitive: pass normally (Vue sets a property when the element has it, else the attribute). Renamed attributes use the attribute name.
   - Complex, and props without an attribute: `'.items': value`. The `.` prefix forces a property even before the element upgrades (Lit keeps it), and Vue SSR skips non-scalar values.
   - The vnode props go through `defined()` (in `vue-utils.ts`), which drops `undefined`: Vue patches every key, and `undefined` on an upgraded element would replace its default (`variant` became `''`, review e2e 2026-10-18). Unsetting a prop later removes the key; Lit then has `null` (Primitive) or `false` (Boolean).
4. Events: listen with a case-exact `'on:<dom_name>'` vnode key, which Vue attaches when it creates the element, so events fired in Lit's first update are not lost. Exception: Vue reads a trailing `Once`, `Passive` or `Capture` in the name as a listener option even after `on:`, so those events are attached in `mounted()` instead (and can miss first-update events). Declare `emits` keyed by `base_name` with typed payloads, e.g. `MyChange: (e: TypedEvent<El, Detail>) => true`. Attach a native listener for `dom_name` to the element and re-emit under `base_name`. The listener prop is `on<base_name>` as in React, and templates write `@my-change`. Exact DOM names are not used: Vue camelizes emit names to find listeners, so `my-change`, `myChange` and `MyChange` would all reach `onMyChange` (D-066).
5. v-model: for each `TwoWayBinding`:
   - prop `value` -> `modelValue` prop + `update:modelValue` emit.
   - any other prop `p` -> prop `p` + `update:p` emit (used as `v-model:p`).
   - Read the new value using `binding.value` (`ValuePath`): `Detail`, `DetailField(name)` or `TargetProp(name)`.
   - Do not mutate props. Emit only; the parent owns state.
6. Slots: default slot -> element children, for every component, documented or not (as React passes children). `render()` destructures named slots into identifier locals (`{ 'help-text': helpText } = this.$slots`, D-081); the slot names themselves are unchanged (`#help-text`). Named Vue slot `x` -> wrapper child with `slot="x"` attribute on a `display: contents` element. Scoped slots are typed as `() => VNode[]` only (D-027).
7. The element and its methods are public instance members, so they are typed for consumers: `setup()` returns `element` (a `shallowRef`, set by `ref: 'element'` in `render()`) and one forwarding function per public method (`show(...args)`). `expose()` is not used, because exposed members are untyped.

## Types

- Per-component `DefineComponent` type exported from each file.
- `global-components.d.ts`: `declare module 'vue' { interface GlobalComponents { 'my-button': ... } }` for raw-tag usage. Props are typed by property name; a renamed attribute (`badge-count`) is not, because Vue templates camelize names for components (limitation, like React types mode). In a file that imports a wrapper, `<my-button>` resolves to the wrapper, not the raw tag.
- Document for consumers: `compilerOptions.isCustomElement: (tag) => tag.includes('-')` is needed only when using raw tags, not the wrappers.

## SSR (Nuxt)

- Wrappers render attributes for Primitive and Boolean props only; `.`-prefixed (Complex, property-only) keys and listeners are skipped by Vue's server renderer. Named slot wrappers are server-rendered.
- The side-effect import that defines the element runs on the server too (Lit's Node build), as in React (D-056). Nuxt 4.6.2 dev and production hydrate with no console warnings (D-079, e2e `nuxt`). Deep Lit SSR (shadow DOM in the server HTML) is not claimed.

## Acceptance (per fixture)

- `vue-tsc --noEmit` passes.
- Consumer `.vue` file using `v-model` compiles and type-checks; wrong payload type fails.
- Playwright vue3 and nuxt apps: props, events, v-model round trip, named slots.

## Raw tags without the wrapper (D-076, e2e `vue3?raw`)

- Raw `v-model` compiles to Vue's text-input directive (`input` event + `el.value`). It follows typing on `fx-input` only because that element fires a composed `input` and updates `value` first; it cannot express `checked`, `detail` paths or custom events, and vue-tsc rejects it (it types v-model as a `modelValue` prop). Document the wrapper's `v-model`.
- Raw Complex props on an element defined after render are lost: Vue sets a property only when `key in el`, so the array becomes the attribute `items="p,q"`. The wrapper's `'.items'` keeps it.
