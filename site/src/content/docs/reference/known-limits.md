---
title: Known limits
description: What holyfloe does not do (yet).
---

- **Next.js** needs `"moduleResolution": "nodenext"` in `tsconfig.json`, because the wrappers import each other with `.js` paths.
- **Types are never resolved.** A field without a type annotation or a literal default is typed from the element class only.
- **React types mode** can lose object props on elements defined after the first render; wrapper mode handles them. See [React](/holyfloe/frameworks/react/#types-mode-react-19-only).
- **Deep Lit SSR** (shadow DOM in the server HTML) is not supported. Wrappers server-render attributes and hydrate on the client.
- **Library typings.** Some published libraries' own `.d.ts` files fail current DOM typings (Shoelace and Web Awesome on `autocorrect`). Use `skipLibCheck`.
- **Bundlers.** Only Vite is tested; the Rollup, webpack, Rspack, esbuild and Rolldown plugins exist but are untested.
- **Stale output.** holyfloe never deletes files, so wrappers for removed components stay until you delete them.

Tested against Shoelace 2 and Web Awesome 3: every component compiles in all three frameworks.
