# Changelog

## 0.1.1

- Packages are published from CI through npm trusted publishing, with provenance: npm shows that each version was built from this repository by GitHub Actions. No code changes.

## 0.1.0

First release.

- `holyfloe generate` turns Lit components into React (18 and 19), Vue 3 and Svelte 5 wrappers, from Lit source (`--lit`) or a Custom Elements Manifest (`--cem`), with `--watch`.
- Exact types from the element class, `CustomEvent<T>` event details, slots, CSS parts and properties, two-way bindings (`@model`, inferred for `value`/`checked`/`open`/`selected`), and SSR-safe prop handling.
- `@holyfloe/core`: `generate()`, `watch()`, `version()` and the CEM analyzer plugin. Native binaries for macOS, Linux (glibc and musl) and Windows, and a WebAssembly fallback.
- `@holyfloe/unplugin`: regenerates on edits in the Vite dev server, with HMR.
- Standalone `holyfloe` binaries on GitHub releases.
