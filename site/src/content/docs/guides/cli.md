---
title: Command line
description: The holyfloe generate command and its flags.
---

```sh
holyfloe generate (--lit <dir> | --cem <custom-elements.json>) --out <dir> --frameworks react,vue,svelte [options]
```

| Flag | Meaning |
|---|---|
| `--lit <dir>` | Read Lit sources from this directory. Scanned recursively, skipping tests, stories, `node_modules`, dot-directories and `--out`. |
| `--cem <file>` | Read a Custom Elements Manifest instead. Exactly one of `--lit` and `--cem`. |
| `--out <dir>` | Output directory. Wrappers go to `<out>/<framework>/`. |
| `--frameworks <list>` | Comma-separated: `react`, `vue`, `svelte`. |
| `--config <file>` | A [`holyfloe.config.json`](/holyfloe/reference/configuration/). |
| `--react-mode <mode>` | `wrapper` or `types`; overrides `react.mode` from the config. |
| `--watch` | Keep running and regenerate when the input (manifest or Lit sources) or the config changes. |

## Examples

```sh
# From Lit source
holyfloe generate --lit src --out generated --frameworks react,vue,svelte

# From a manifest
holyfloe generate --cem custom-elements.json --out generated --frameworks react,vue,svelte

# Keep running and regenerate on changes
holyfloe generate --lit src --out generated --frameworks react --watch
```

## Output

| Framework | Files |
|---|---|
| React | `FxButton.tsx` per element, `index.ts`, `react-utils.ts` |
| Vue | `FxButton.ts` per element, `index.ts`, `vue-utils.ts`, `global-components.d.ts` |
| Svelte | `FxButton.svelte` per element, `index.ts`, `svelte-utils.ts`, `svelte-elements.d.ts` |

Component names are the PascalCase tag: `fx-button` gives `FxButton`. Only files whose content changed are written; nothing is deleted, so remove the output directory yourself when you rename or drop a component.

## Errors

Errors print as `error[E002]: message (module)` with a `hint:` line, and the command exits with status 1. See [error codes](/holyfloe/reference/errors/).
