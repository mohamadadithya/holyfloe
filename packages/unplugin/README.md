# @holyfloe/unplugin

Keeps typed React, Vue and Svelte wrappers for your Lit web components in sync while you develop. Built on [unplugin](https://github.com/unjs/unplugin) and [`@holyfloe/core`](https://www.npmjs.com/package/@holyfloe/core).

```sh
npm install -D @holyfloe/unplugin
```

```ts
// vite.config.ts
import holyfloe from '@holyfloe/unplugin';

export default {
  plugins: [
    holyfloe.vite({ lit: 'src/components', out: 'src/generated', frameworks: ['react', 'vue', 'svelte'] }),
  ],
};
```

Wrappers are generated when the build starts. In the dev server, editing a component (`lit`) or the manifest (`cem: 'custom-elements.json'`) regenerates only the wrappers that changed, and Vite hot-updates them. Paths are resolved from the Vite root.

Options: `cem` or `lit` (exactly one), `out`, `frameworks`, and optionally `config` and `reactMode`. A failure on the first build stops the build; later failures are logged and the dev server keeps running.

`holyfloe.rollup()`, `holyfloe.webpack()`, `holyfloe.rspack()`, `holyfloe.esbuild()` and `holyfloe.rolldown()` are available too; only Vite is tested so far.

Full documentation: [mohamadadithya.github.io/holyfloe](https://mohamadadithya.github.io/holyfloe/).
