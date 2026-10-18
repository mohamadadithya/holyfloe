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
3. Props: declare each public prop with `PropType<MyButtonElement['x']>`.
   - Boolean: `{ type: Boolean, default: undefined }` so absent stays absent.
   - Complex: pass with the `.prop` modifier form in `h()` (`{ '.items': value }` pattern, or assign in `onMounted`/`watchEffect` via a template ref) so lazy definition cannot demote it to an attribute.
   - Primitive: pass normally.
4. Events: declare `emits` keyed by `base_name` with typed payloads, e.g. `MyChange: (e: TypedEvent<El, Detail>) => true`. Attach a native listener for `dom_name` to the element and re-emit under `base_name`. The listener prop is `on<base_name>` as in React, and templates write `@my-change`. Exact DOM names are not used: Vue camelizes emit names to find listeners, so `my-change`, `myChange` and `MyChange` would all reach `onMyChange` (D-066).
5. v-model: for each `TwoWayBinding`:
   - prop `value` -> `modelValue` prop + `update:modelValue` emit.
   - any other prop `p` -> prop `p` + `update:p` emit (used as `v-model:p`).
   - Read the new value using `binding.value` (`ValuePath`): `Detail`, `DetailField(name)` or `TargetProp(name)`.
   - Do not mutate props. Emit only; the parent owns state.
6. Slots: default slot -> element children. Named Vue slot `x` -> wrapper child with `slot="x"` attribute on a `display: contents` element. Scoped slots are typed as `() => VNode[]` only (D-027).
7. Expose element methods via `expose()` as a typed proxy to the underlying element.
8. Expose the element via a `elementRef` exposed property.

## Types

- Per-component `DefineComponent` type exported from each file.
- `global-components.d.ts`: `declare module 'vue' { interface GlobalComponents { 'my-button': ... } }` for raw-tag usage.
- Document for consumers: `compilerOptions.isCustomElement: (tag) => tag.includes('-')` is needed only when using raw tags, not the wrappers.

## SSR (Nuxt)

- Wrappers render attributes for Primitive and Boolean props only.
- Element definition is deferred to the client with a client-only import unless a Lit SSR integration is configured. Do not claim more.

## Acceptance (per fixture)

- `vue-tsc --noEmit` passes.
- Consumer `.vue` file using `v-model` compiles and type-checks; wrong payload type fails.
- Playwright vue3 and nuxt apps: props, events, v-model round trip, named slots.

## Unverified (see open-questions.md)

- OQ-003: whether raw `v-model` on a custom element works without the wrapper. Design assumes it does not give typed behavior. Verify with an e2e test in M2 before documenting.
