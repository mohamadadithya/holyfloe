# Releasing holyfloe

A release ships two things from one version tag:

- **Standalone binaries** on a GitHub Release, from `.github/workflows/release.yml` (cargo-dist): macOS (arm64, x64), Linux glibc (x64, arm64), Linux musl (x64, static) and Windows (x64), with checksums.
- **npm packages** from `.github/workflows/napi.yml`: `@holyfloe/core`, `@holyfloe/unplugin` and the 9 platform packages `@holyfloe/core-<platform>` (8 native, plus `wasm32-wasi`). CI only *stages* them; a maintainer approves them with 2FA before they go public (D-096).

The version lives in three files, kept equal: `Cargo.toml` (`[workspace.package]`), `packages/core/package.json` and `packages/unplugin/package.json`. The napi loader `packages/core/index.js` embeds it too.

## Before you start

- You're an owner of the `@holyfloe` npm org, with 2FA on, and logged in (`npm whoami`).
- `gh` (GitHub CLI) is logged in with push access, and `just`, Rust and pnpm are set up as in [CONTRIBUTING.md](../CONTRIBUTING.md).
- `main` is green in CI.

## Release in three commands

```sh
just release 0.2.0
```
This creates branch `release-0.2.0` from `origin/main` and bumps all versions (and `Cargo.lock`). It regenerates the napi loader, drafts a CHANGELOG entry from the commit subjects since the last tag, and opens a PR.

Edit the CHANGELOG entry in the PR so users can read it, wait for CI, then merge.

```sh
just tag
```
This tags the merged `origin/main` as `v0.2.0` after a confirmation, and pushes the tag. It refuses if the versions on `main` disagree or the tag already exists. The push starts both release workflows (about 15 minutes):

- `release.yml` builds the binaries and publishes the GitHub Release.
- `napi.yml` builds the 9 addons, assembles the npm packages, and its `publish` job stages all 11 on npm through trusted publishing (OIDC, no token).

```sh
just approve
```
Run this once the `publish` job is green. It lists the staged packages and checks they are exactly the 11 expected ones at the new version; anything else stops it before it approves anything. It then shows who staged each one, asks for a 2FA code, and approves in order: platform packages first, then `@holyfloe/core`, then `@holyfloe/unplugin`. Run it again if it stops partway; it skips what is already published.

## After the release

New versions can show `version not found` for a few minutes while npm processes them. Then check:

```sh
npm view @holyfloe/core version            # the new version
npm view @holyfloe/unplugin version
cd "$(mktemp -d)" && npm init -y >/dev/null && npm i @holyfloe/core && node -e "console.log(require('@holyfloe/core').version())"
npm audit signatures                       # in that folder: registry signatures and provenance attestations
```

## Versioning

[Semantic versioning](https://semver.org). While the version is 0.x, a minor bump (0.2.0) may contain breaking changes, such as different generated output or renamed options; a patch bump (0.1.2) must not. Generated wrapper output is part of the public contract, so mention any change to it in the CHANGELOG.

## Troubleshooting

**The `publish` job fails with an authentication or OIDC error.** The trusted publisher for a package is missing, or expired: npm drops one that hasn't published within 2 days of being created. Re-create all 11 (with 2FA), then re-run the failed job with `gh run rerun <run-id> --failed`:

```sh
for p in core unplugin core-darwin-arm64 core-darwin-x64 core-linux-x64-gnu core-linux-arm64-gnu \
         core-linux-x64-musl core-linux-arm64-musl core-win32-x64-msvc core-win32-arm64-msvc core-wasm32-wasi; do
  npm trust github "@holyfloe/$p" --file napi.yml --repository mohamadadithya/holyfloe --allow-stage-publish
done
```

**`just approve` reports something unexpected staged.** Don't approve. Reject it on npmjs.com (avatar, **Staged Packages**) or with `npm stage reject <id>`, and find out who staged it. Only `napi.yml` on this repository can stage, so an unexpected item means the repository or its CI needs a look.

**A build job fails.** Fix it on a branch, merge, then delete the tag and tag again:
```sh
git push origin :refs/tags/v0.2.0 && git tag -d v0.2.0
just tag
```
Do this only before any package was approved. npm versions can't be republished, so once one is out, release the next patch version instead.

**The GitHub Release is wrong but npm is fine.** Edit or re-upload assets on the release page; the npm packages are unaffected.

**Publishing by hand** (if CI publishing is unavailable). This is how 0.1.0 shipped. Run from the repository, with the `npm` artifact of the tag's `napi.yml` run:
```sh
cd packages/core
gh run download <napi-run-id> -n npm -D /tmp/holyfloe-release
rm -rf npm && cp -R /tmp/holyfloe-release/npm npm
for d in npm/*/; do (cd "$d" && npm publish --access public); done
pnpm exec napi pre-publish -t npm --skip-optional-publish   # adds the optionalDependencies
pnpm publish --access public --no-git-checks
git checkout package.json && rm -rf npm
cd ../unplugin && pnpm publish --access public --no-git-checks
```

## Changing the release setup

- **`release.yml` is generated.** Edit `dist-workspace.toml`, then run `dist generate` with cargo-dist 0.33.0 (D-044); `dist plan` in CI fails on hand edits. To run every build on a pull request without releasing, set `pr-run-mode = "upload"` temporarily (D-096).
- **`napi.yml` is hand-written.** Its `publish` job runs only on `v*` tags.
- **Background:** the decisions behind this setup are D-044, D-087, D-096, D-097 and D-098 in [decisions.md](decisions.md).
