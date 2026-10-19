// `just approve`: approve the npm packages CI staged for the current release (D-096), in the
// order users need them: platform packages, then @holyfloe/core, then @holyfloe/unplugin.
// Approves nothing unless exactly the expected 11 packages are staged at the release version.
import { execFileSync, spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { createInterface } from 'node:readline';

// napi platform names for packages/core `napi.targets`, in that order.
const PLATFORMS = [
  'darwin-arm64', 'darwin-x64', 'linux-x64-gnu', 'linux-arm64-gnu', 'linux-x64-musl',
  'linux-arm64-musl', 'win32-x64-msvc', 'win32-arm64-msvc', 'wasm32-wasi',
];

const core = JSON.parse(readFileSync(new URL('../packages/core/package.json', import.meta.url), 'utf8'));
if (core.napi.targets.length !== PLATFORMS.length) {
  throw new Error(`napi.targets has ${core.napi.targets.length} entries, PLATFORMS ${PLATFORMS.length}: update approve-staged.mjs`);
}
const version = core.version;
const order = [...PLATFORMS.map((p) => `@holyfloe/core-${p}`), '@holyfloe/core', '@holyfloe/unplugin'];

const staged = JSON.parse(execFileSync('npm', ['stage', 'list', '--json'], { encoding: 'utf8' }))
  .filter((item) => item.packageName?.startsWith('@holyfloe/'));

const problems = [];
const byName = new Map();
for (const item of staged) {
  if (item.version !== version) problems.push(`${item.packageName}@${item.version} is staged, but this release is ${version}`);
  else if (!order.includes(item.packageName)) problems.push(`unexpected package ${item.packageName}`);
  else if (byName.has(item.packageName)) problems.push(`${item.packageName}@${version} is staged twice`);
  else byName.set(item.packageName, item);
}
// Not staged: fine if a previous run already approved it, i.e. it is on the registry.
const published = (name) => {
  try {
    return execFileSync('npm', ['view', `${name}@${version}`, 'version'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim() === version;
  } catch {
    return false;
  }
};
const done = new Set();
for (const name of order) {
  if (byName.has(name) || problems.length) continue;
  if (published(name)) done.add(name);
  else problems.push(`${name}@${version} is not staged (yet?)`);
}
if (problems.length) {
  console.error(`Nothing approved:\n  ${problems.join('\n  ')}`);
  console.error('Reject unexpected items on npmjs.com (Staged Packages) or with `npm stage reject <id>`.');
  process.exit(1);
}

const todo = order.filter((name) => !done.has(name));
if (!todo.length) {
  console.log(`All ${order.length} packages are already published at ${version}.`);
  process.exit(0);
}
console.log(`Staged for ${version}${done.size ? ` (${done.size} already published)` : ''}:`);
for (const name of todo) {
  const { id, actor, actorType } = byName.get(name);
  console.log(`  ${name.padEnd(34)} ${id}  staged by ${actorType ? `${actor} (${actorType})` : actor}`);
}
// One line stream: readline's question() drops lines that arrive before it asks (piped input).
const rl = createInterface({ input: process.stdin });
const lines = rl[Symbol.asyncIterator]();
const ask = async (prompt) => {
  process.stdout.write(prompt);
  const { value = '' } = await lines.next();
  return value.trim();
};
if ((await ask(`Approve these ${todo.length}? [y/N] `)).toLowerCase() !== 'y') process.exit(1);
// One 2FA code for all approvals; if npm rejects a reused code, npm prompts for a fresh one.
let otp = await ask('2FA code (Enter to let npm prompt instead): ');
rl.close();

for (const name of todo) {
  const args = ['stage', 'approve', byName.get(name).id, ...(otp ? ['--otp', otp] : [])];
  let res = spawnSync('npm', args, { stdio: ['inherit', 'inherit', 'pipe'], encoding: 'utf8' });
  if (res.status !== 0 && otp && /EOTP|one-time password/i.test(res.stderr)) {
    otp = '';
    res = spawnSync('npm', ['stage', 'approve', byName.get(name).id], { stdio: 'inherit' });
  } else if (res.stderr) {
    process.stderr.write(res.stderr);
  }
  if (res.status !== 0) {
    console.error(`Stopped at ${name}; run \`just approve\` again after fixing the error (approved packages are skipped).`);
    process.exit(1);
  }
}
console.log(`Approved ${todo.length} packages. They can take a few minutes to appear on the registry.`);
