---
title: Bundler plugin
description: Keep wrappers in sync during development with @holyfloe/unplugin.
---

`@holyfloe/unplugin` generates the wrappers when the build starts and, in the dev server, regenerates them when a component changes.

```sh
npm install -D @holyfloe/unplugin
```

```ts title="vite.config.ts"
import holyfloe from '@holyfloe/unplugin';

export default {
  plugins: [
    holyfloe.vite({ lit: 'src/components', out: 'src/generated', frameworks: ['react', 'vue', 'svelte'] }),
  ],
};
```

Editing a component (`lit`) or the manifest (`cem: 'custom-elements.json'`) regenerates only the wrappers that changed, and Vite hot-updates them. Paths are resolved from the Vite root.

## Options

| Option | Meaning |
|---|---|
| `lit` | Directory of Lit sources. |
| `cem` | Path to a Custom Elements Manifest. Exactly one of `lit` and `cem`. |
| `out` | Output directory. |
| `frameworks` | Any of `'react'`, `'vue'`, `'svelte'`. |
| `config` | Path to a [`holyfloe.config.json`](/holyfloe/reference/configuration/). Optional. |
| `reactMode` | `'wrapper'` or `'types'`. Optional. |

A failure on the first build stops the build. Later failures are logged and the dev server keeps running.

## Other bundlers

`holyfloe.rollup()`, `holyfloe.webpack()`, `holyfloe.rspack()`, `holyfloe.esbuild()` and `holyfloe.rolldown()` exist too, but only Vite is tested so far.
