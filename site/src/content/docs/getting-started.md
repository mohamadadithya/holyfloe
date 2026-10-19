---
title: Getting started
description: Install holyfloe and generate your first wrappers.
---

holyfloe reads your Lit components, either the source directly or a [Custom Elements Manifest](https://github.com/webcomponents/custom-elements-manifest), and writes one wrapper per element for React 18 and 19, Vue 3 and Svelte 5.

## Install

Pick the entry point that fits your build:

```sh
npm install -D @holyfloe/unplugin   # Vite / Rollup / webpack / esbuild plugin
npm install -D @holyfloe/core       # Node API (generate, watch) and the CEM analyzer plugin
```

The standalone `holyfloe` binary for macOS, Linux and Windows is attached to each [GitHub release](https://github.com/mohamadadithya/holyfloe/releases).

## Generate wrappers

From Lit source, no analyzer needed:

```sh
holyfloe generate --lit src --out generated --frameworks react,vue,svelte
```

or from a manifest, e.g. one written by `@custom-elements-manifest/analyzer`:

```sh
holyfloe generate --cem custom-elements.json --out generated --frameworks react,vue,svelte
```

Wrappers go to `generated/react/`, `generated/vue/` and `generated/svelte/`. Each folder has one file per element, an `index.ts` that re-exports them, and a shared utils file. Only files whose content changed are written, and nothing is deleted.

## Package paths

The wrappers import your elements by package name, e.g. `import '@my/lib/button.js'`. holyfloe finds the `package.json` nearest to the input: its `name` is the package, and its `exports` decide which paths are importable.

If your source paths differ from the published ones (`src/button.ts` is published as `dist/button.js`), map them with [`moduleRewrite`](/holyfloe/reference/configuration/#modulerewrite):

```json title="holyfloe.config.json"
{ "moduleRewrite": { "src/": "dist/" } }
```

```sh
holyfloe generate --lit src --out generated --frameworks react --config holyfloe.config.json
```

## Next steps

- Keep wrappers in sync while you develop with the [bundler plugin](/holyfloe/guides/bundler-plugin/) or `--watch`.
- Learn which [JSDoc tags](/holyfloe/guides/documenting-components/) holyfloe reads.
- See what each framework gets: [React](/holyfloe/frameworks/react/), [Vue](/holyfloe/frameworks/vue/), [Svelte](/holyfloe/frameworks/svelte/).
