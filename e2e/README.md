# e2e: generated wrappers in a real browser

`just e2e-react` regenerates `fixtures/out` with the CLI, then runs Playwright (Chromium) on:

| Project | App | Server |
|---|---|---|
| `react18-dev`, `react19-dev` | `react18/`, `react19/` | `vite` (React dev build, StrictMode double effects, Lit dev mode) |
| `react18-prod`, `react19-prod` | same | `vite build` + `vite preview` |
| `next-dev`, `next-prod` | `next-app/` (Next 16 App Router) | `next dev`; `next build` + `next start` |

Layout:
- `shared/App.tsx`: the page. It renders every fixture through `fixtures/out/react/<name>/index.js`, one block per fixture, each with an `id`.
- `shared/vite.config.ts`: used by both apps. `dedupe` makes React, and `@holyfloe/fixtures` (through its `exports`, so from `fixtures/dist/`), resolve from the app.
- `react19/src/TypesMode.tsx`: `types` mode, plain tags, React 19 only.
- `tests/react.spec.ts`: one `describe` per fixture. Any console error or warning fails a test.
- `next-app/app/page.tsx`: a server component rendering all 7 wrappers. `tests/next.spec.ts` checks the raw server HTML, hydration, and a client event (`app/Clicker.tsx`). The `test` script copies `fixtures/out/react` to `next-app/generated/` first, where a user would generate wrappers. `tsconfig.json` uses `moduleResolution: "nodenext"` because of OQ-017.

Adding a fixture: render it in `shared/App.tsx` with an `id`, and add a `describe` block that looks it up. Assert properties for every prop. Assert attributes only for reflected or renamed props, because React 19 sets known props as properties and writes no attribute (`docs/emit-react.md` rule 5).

Run one test: `pnpm -C e2e test -g "fx-card" --project react19-dev` (the `test` script refreshes `next-app/generated/` first).
