# Contributing to holyfloe

Thanks for helping. This guide covers setting up the repository, how changes are checked, and what a pull request needs.

## Reporting bugs and asking for features

Open a [GitHub issue](https://github.com/mohamadadithya/holyfloe/issues). For a bug, include:

- the holyfloe version (`holyfloe --version`, or `version()` from `@holyfloe/core`);
- how you ran it: CLI, Node API or the bundler plugin, with `--lit` or `--cem`, and the frameworks;
- the smallest input that shows the problem, either a Lit component or a `custom-elements.json` excerpt, plus your `holyfloe.config.json`;
- what you expected and what you got, including the error code (`E0xx`) if there is one.

A component library that fails is very useful to report, because real libraries find most bugs.

**Security issues:** please don't open a public issue. Use [Report a vulnerability](https://github.com/mohamadadithya/holyfloe/security/advisories/new) on the Security tab.

## Setting up

You need:

- **Rust:** `rust-toolchain.toml` pins the version, and rustup installs it on the first `cargo` command.
- **Node.js 24** (CI uses 24) and **pnpm 12.9.1**: `corepack enable` picks up the version from `package.json`.
- **[just](https://github.com/casey/just)**, which runs the project's commands.
- Optional: `cargo install cargo-insta` to review snapshots, and the GitHub CLI `gh`.

Then, once:

```sh
pnpm install
pnpm -C e2e exec playwright install chromium
rustup target add wasm32-wasip1-threads
```

## How the project fits together

Lit source (read by `holyfloe-lit` with oxc) or a Custom Elements Manifest goes into `holyfloe-ir`, which builds a framework-neutral IR. The emitters `holyfloe-emit-react`, `-vue` and `-svelte` render it with minijinja templates, and `holyfloe-fs` writes only files whose content changed. The CLI, the napi addon (`@holyfloe/core`) and the bundler plugin (`@holyfloe/unplugin`) all wrap that one pipeline.

| Read | For |
|---|---|
| [`docs/ir.md`](docs/ir.md) | the IR, its passes, configuration keys, error codes |
| [`docs/emit-react.md`](docs/emit-react.md), [`emit-vue.md`](docs/emit-vue.md), [`emit-svelte.md`](docs/emit-svelte.md) | what each framework's output looks like and why |
| [`docs/decisions.md`](docs/decisions.md) | every settled design decision (D-001 onward) |
| [`fixtures/README.md`](fixtures/README.md) | the 12 fixture components and what each covers |
| [`CLAUDE.md`](CLAUDE.md) | the full contributor reference: layout, commands, pinned versions, hard rules (also read by AI coding assistants) |

## Making a change

**The fixtures are the spec.** `fixtures/src/<name>/` holds a Lit component and its manifest, and `fixtures/expected/<framework>/<name>/` holds the hand-written output the emitters must match byte for byte.

- **A change to generated output** means updating `fixtures/expected/`, regenerating with `just gen-fixtures`, and reviewing the insta snapshots with `cargo insta review`. Say in the PR why the output changed; never accept a snapshot change without a reason.
- **New behaviour needs a fixture or a test.** A bug fix should come with a test that fails without the fix.
- **A new edge case** gets a fixture plus coverage in the type-check consumers and e2e tests, as `fixtures/README.md` describes.

**Rules the code follows** (details in CLAUDE.md):

- **The IR and the emitters are pure:** no filesystem, network or clock access outside `holyfloe-fs`, `holyfloe-cli` and `holyfloe-napi`.
- **Types are never resolved.** Type text is kept verbatim, and wrappers type props as `Element['prop']`. Don't add a TypeScript checker.
- **Framework files come from minijinja templates,** not AST printers.
- **No `unwrap`/`expect` in library crates;** return typed errors (`thiserror`).
- **Dependencies are pinned to exact versions.** An upgrade gets its own PR and updates the version table in CLAUDE.md.
- **Generated TypeScript must type-check** under `strict`.

**Settled decisions:** `docs/decisions.md` records why things are the way they are. If a change contradicts one, raise it in an issue first instead of changing it silently. New decisions are appended with the next number, and existing ones are never renumbered.

## Checking your change

Run the focused checks while you work:

```sh
cargo test -p holyfloe-ir                 # one crate
cargo insta test -p holyfloe-emit-react   # one emitter's snapshots, then: cargo insta review
just parity                               # Lit-source IR == analyzer IR on every fixture
just typecheck-react                      # or typecheck-vue / typecheck-svelte
just e2e-vue                              # Playwright for one framework
```

Before opening the PR, run everything CI runs:

```sh
just verify
```

It runs format, build, clippy, tests, fixture regeneration (which must leave no diff), type checks, the Playwright matrix (React 18/19, Next, Vue, Nuxt, Svelte, SvelteKit), the napi addon on native and wasm, and the Vite plugin test.

## Pull requests

- **Keep each PR to one logical change,** with docs updated in the same PR when behaviour changes: docs are the source of truth.
- **Commit messages:** `<area>: <what changed>`, lowercase, e.g. `ir: infer bindings for the indeterminate prop` or `emit-svelte: keep slot order`. Common areas are `ir`, `lit`, `emit-react`, `emit-vue`, `emit-svelte`, `cli`, `napi`, `unplugin`, `fixtures`, `docs`, `ci` and `release`.
- **Say what you verified** in the PR description: which commands ran, and anything you couldn't run.
- **CI must pass:** `just verify` on Ubuntu and macOS.

Releases are done by maintainers; see [`docs/releasing.md`](docs/releasing.md).

## License

holyfloe is dual-licensed under MIT or Apache-2.0, at your option. Unless you state otherwise, any contribution you submit is licensed under the same terms, without additional conditions.
