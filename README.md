# holyfloe

Typed React, Vue and Svelte wrappers for Lit web components, generated from your Lit source or from a [Custom Elements Manifest](https://github.com/webcomponents/custom-elements-manifest).

Write a component once in Lit:

```ts
/**
 * @model open open-change detail.open
 * @fires {CustomEvent<{ open: boolean }>} open-change - Fired when the dialog opens or closes.
 * @slot - Dialog content.
 * @slot footer - Dialog actions.
 */
@customElement('fx-dialog')
export class FxDialog extends LitElement {
  @property({ type: Boolean, reflect: true }) open = false;
  show() { /* ... */ }
}
```

and use it as a native component in each framework, fully typed:

```tsx
// React 18 and 19
<FxDialog ref={ref} open={open} onOpenChange={(e) => setOpen(e.detail.open)}>
  Body
  <button slot="footer">OK</button>
</FxDialog>
```

```vue
<!-- Vue 3 -->
<FxDialog v-model:open="open">
  Body
  <template #footer><button @click="open = false">OK</button></template>
</FxDialog>
```

```svelte
<!-- Svelte 5 -->
<FxDialog bind:open bind:element>
  Body
  {#snippet footer()}<button onclick={() => element?.show()}>OK</button>{/snippet}
</FxDialog>
```

## What you get

- **Real components, not just types.** React `forwardRef` components (or React 19 type augmentations), Vue `defineComponent` wrappers and Svelte 5 components with runes.
- **Exact types without a type checker.** Props are typed from the element class (`El['prop']`), events from `CustomEvent<T>`, and slots become React `slot` children, Vue named slots and Svelte snippets.
- **Two-way binding.** `v-model` in Vue and `bind:` in Svelte, inferred for `value`/`checked`/`open`/`selected` or declared with `@model`.
- **SSR-safe.** Primitive props render as attributes on the server; objects and arrays are set as properties on the client. Tested on Next 16, Nuxt 4 and SvelteKit 3.
- **Fast and incremental.** A native Rust engine, a `--watch` mode, and a Vite plugin with HMR. Only changed files are rewritten.

## Install

```sh
npm install -D @holyfloe/core            # Node API (generate, watch) and the analyzer plugin
npm install -D @holyfloe/unplugin        # Vite / Rollup / webpack / esbuild plugin
```

The standalone `holyfloe` binary for macOS, Linux and Windows is attached to each [GitHub release](https://github.com/mohamadadithya/holyfloe/releases).

## Use

### Command line

```sh
# From Lit source (no analyzer needed)
holyfloe generate --lit src --out generated --frameworks react,vue,svelte

# From a Custom Elements Manifest (e.g. written by @custom-elements-manifest/analyzer)
holyfloe generate --cem custom-elements.json --out generated --frameworks react,vue,svelte

# Keep running and regenerate on changes
holyfloe generate --lit src --out generated --frameworks react --watch
```

Wrappers go to `<out>/<framework>/`. The `package.json` nearest to the input names your package, and its `exports` decide the import paths in the wrappers.

### Vite (and other bundlers)

```ts
// vite.config.ts
import holyfloe from '@holyfloe/unplugin';

export default {
  plugins: [
    holyfloe.vite({ lit: 'src/components', out: 'src/generated', frameworks: ['react'] }),
  ],
};
```

Editing a component regenerates only its wrapper, and Vite hot-updates it. Use `cem` instead of `lit` to read a manifest. `holyfloe.rollup()`, `holyfloe.webpack()`, `holyfloe.esbuild()` and `holyfloe.rspack()` exist too, but only Vite is tested so far.

### Node API

```js
import { generate, watch } from '@holyfloe/core';

const reports = generate({ lit: 'src', out: 'generated', frameworks: ['vue'] });
const watcher = watch({ cem: 'custom-elements.json', out: 'generated', frameworks: ['svelte'] }, (err, reports) => {});
watcher.close();
```

Errors carry a code (`error.code === 'E002'`) and a fix hint in the message.

## Configuration

Pass `--config holyfloe.config.json` (or `config` in the plugin and Node API):

```json
{
  "moduleRewrite": { "src/": "dist/" },
  "exclude": ["internal-*"],
  "bindings": "infer",
  "react": { "mode": "wrapper", "useClient": true, "ref": "forwardRef" },
  "overrides": {
    "my-input": {
      "bindings": [{ "prop": "value", "event": "my-change", "value": "detail.value" }],
      "events": { "my-change": "MyChange" },
      "excludeProps": ["internalState"]
    }
  }
}
```

- `moduleRewrite` maps source or manifest paths to published ones, e.g. `src/x.ts` to `dist/x.js`. Libraries that publish manifests with paths relative to `dist/` use `{ "": "dist/" }`.
- `react.mode`: `wrapper` (React 18 and 19, the default) or `types` (React 19 only: types for the plain tags, no wrapper components).
- `react.ref`: `forwardRef` (the default) or `prop` (React 19 only).
- `bindings`: `infer` (the default) or `explicit-only`.

## Documenting components

holyfloe reads the same JSDoc as the CEM analyzer:

| Tag | Meaning |
|---|---|
| `@fires {CustomEvent<T>} name - text` / `@event` | An event and its detail type. `this.dispatchEvent(new CustomEvent<T>('name'))` works too. |
| `@slot [name] - text` | A slot (no name: the default slot). |
| `@csspart`, `@cssprop`, `@cssstate` | CSS parts, custom properties and states. |
| `@model prop event [detail.field]` | A two-way binding (`v-model`, `bind:`). |
| `@deprecated`, `@internal`, `@private` | Deprecation; hidden from wrappers. |
| `@tag name` | The tag, when there is no `@customElement` or `customElements.define`. |

When you use the CEM analyzer, add the holyfloe plugin so `@model` survives:

```js
// custom-elements-manifest.config.mjs
import holyfloeModels from '@holyfloe/core/cem-plugin';
export default { litelement: true, plugins: [holyfloeModels()] };
```

## Known limits

- Next.js needs `"moduleResolution": "nodenext"` in `tsconfig.json`, because the wrappers import each other with `.js` paths.
- Types are never resolved: a field without a type annotation or a literal default is typed from the element class only.
- React `types` mode can lose object props on elements defined after the first render; wrapper mode handles them.
- Some published libraries' own `.d.ts` files fail current DOM typings (Shoelace and Web Awesome on `autocorrect`). Use `skipLibCheck`.
- Only Vite is tested among the bundlers.

## Status

Early releases (0.1.x). Tested against Shoelace 2 and Web Awesome 3: every component compiles in all three frameworks. Design decisions are logged in [`docs/decisions.md`](docs/decisions.md). AI coding assistants can read [`llms.txt`](llms.txt), a summary with links to every doc.

## License

MIT or Apache-2.0, at your option.
