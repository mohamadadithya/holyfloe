---
title: Configuration
description: Every holyfloe.config.json key.
---

Pass the file with `--config holyfloe.config.json`, or `config` in the [bundler plugin](/holyfloe/guides/bundler-plugin/) and [Node API](/holyfloe/guides/node-api/).

```json title="holyfloe.config.json"
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

Input, output and frameworks are given on the command line or in the plugin options, not in this file.

## `moduleRewrite`

Prefix pairs that map source or manifest paths to published ones, e.g. `{ "src/": "dist/" }` turns `src/button.ts` into `dist/button.js`. The longest matching prefix wins.

A module path becomes an import path in four steps:

1. Strip a leading `/` or `./`.
2. Change `.ts`/`.tsx` to `.js`, `.mts` to `.mjs` and `.cts` to `.cjs`.
3. Apply `moduleRewrite`.
4. Match the result against `package.json` `exports`. String targets, `*` patterns and condition objects (`import` before `default`) are supported, and `null` exclusions are honoured.

No `exports` field means every file is importable as `<pkg>/<path>`. A module that matches nothing is error [`E002`](/holyfloe/reference/errors/).

Libraries that publish manifests with paths relative to `dist/` (Shoelace 2, Web Awesome 3) use `{ "": "dist/" }`.

## `exclude`

Tag globs to leave out, where `*` matches any run of characters. Excluded components can still be base classes of others.

## `bindings`

`"infer"` (default) or `"explicit-only"`. See [two-way binding](/holyfloe/guides/documenting-components/#two-way-binding).

## `react`

| Key | Values | Default |
|---|---|---|
| `mode` | `"wrapper"` (React 18 and 19) or `"types"` (React 19 only) | `"wrapper"` |
| `useClient` | Start each wrapper with `'use client'` | `true` |
| `ref` | `"forwardRef"`, or `"prop"` (React 19 only) | `"forwardRef"` |

Any other `react` key is an error, so typos fail instead of doing nothing. See [React](/holyfloe/frameworks/react/).

## `overrides`

Per-tag settings. An override for a tag, event or prop that does not exist is error `E009`, so a config serves one library.

- **`overrides.<tag>.bindings`**: explicit two-way bindings, `[{ "prop", "event", "value"? }]`. `value` is `detail` (default), `detail.<field>` or `target.<prop>`. An unknown or read-only prop, or an unknown event, is `E015`.
- **`overrides.<tag>.events`**: rename an event's base name, without the framework prefix. `{ "my-change": "MyChange" }` gives React `onMyChange`. This is how you fix `E006`, two events that map to the same name.
- **`overrides.<tag>.excludeProps`**: props to leave out of every wrapper, e.g. a stale JSDoc `@property` the class does not have, or a public internal. Web Awesome 3 needs `"wa-drawer": { "excludeProps": ["modal"] }`.

## Not supported yet

`tagFormatter` and `componentNameFormatter` are reserved: any non-null value is error `E010`.
