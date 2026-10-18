import { defineConfig, devices } from '@playwright/test';

// One project per app and mode. HOLYFLOE_E2E=react|vue|svelte picks one framework's apps (so
// `just e2e-vue` starts only Vue servers); unset runs them all.
//   react: react18/react19 under `vite` dev and build + preview (tests/react.spec.ts), and
//          next-app under `next dev` and build + start (tests/next.spec.ts).
//   vue:   vue3 (Vite), svelte: svelte5 (Vite), both dev and build + preview
//          (tests/wrappers.spec.ts, shared; plus tests/vue.spec.ts or tests/svelte.spec.ts), and
//          nuxt / sveltekit, dev and production server (tests/ssr.spec.ts, shared).
type App = { framework: string; name: string; testMatch: string[]; port: number; command: string; env?: Record<string, string>; timeout?: number };

const host = (port: number) => `--port ${port} --strictPort --host 127.0.0.1`;
const vite = (dir: string, mode: string, port: number) =>
  mode === 'dev'
    ? `pnpm exec vite ${dir} ${host(port)}`
    : `pnpm exec vite build ${dir} --logLevel warn && pnpm exec vite preview ${dir} ${host(port)}`;
const modes = ['dev', 'prod'] as const;

const apps: App[] = [
  ...[18, 19].flatMap((react, i) =>
    modes.map((mode, j) => {
      const port = 5181 + i * 10 + j;
      return { framework: 'react', name: `react${react}-${mode}`, testMatch: ['react.spec.ts'], port, command: vite(`react${react}`, mode, port) };
    }),
  ),
  // Next 16 keeps dev output in .next/dev, so dev and prod can run at once.
  ...modes.map((mode, j) => {
    const port = 5201 + j;
    const next = `pnpm -C next-app exec next`;
    const at = `--port ${port} --hostname 127.0.0.1`;
    return {
      framework: 'react',
      name: `next-${mode}`,
      testMatch: ['next.spec.ts'],
      port,
      command: mode === 'dev' ? `${next} dev ${at}` : `${next} build && ${next} start ${at}`,
      env: { NEXT_TELEMETRY_DISABLED: '1' },
      timeout: 180_000,
    };
  }),
  ...modes.map((mode, j) => {
    const port = 5301 + j;
    return { framework: 'vue', name: `vue3-${mode}`, testMatch: ['wrappers.spec.ts', 'vue.spec.ts'], port, command: vite('vue3', mode, port) };
  }),
  // Nuxt 4 SSR: `nuxt dev`, and `nuxt build` + the Nitro node server.
  ...modes.map((mode, j) => {
    const port = 5311 + j;
    const nuxt = 'pnpm -C nuxt exec nuxt';
    return {
      framework: 'vue',
      name: `nuxt-${mode}`,
      testMatch: ['ssr.spec.ts'],
      port,
      command: mode === 'dev' ? `${nuxt} dev --port ${port} --host 127.0.0.1` : `${nuxt} build && node nuxt/.output/server/index.mjs`,
      env: { NUXT_TELEMETRY_DISABLED: '1', PORT: String(port), HOST: '127.0.0.1' },
      timeout: 180_000,
    };
  }),
  ...modes.map((mode, j) => {
    const port = 5401 + j;
    return { framework: 'svelte', name: `svelte5-${mode}`, testMatch: ['wrappers.spec.ts', 'svelte.spec.ts'], port, command: vite('svelte5', mode, port) };
  }),
  // SvelteKit 3 SSR: `vite dev`, and `vite build` + adapter-node.
  ...modes.map((mode, j) => {
    const port = 5411 + j;
    return {
      framework: 'svelte',
      name: `sveltekit-${mode}`,
      testMatch: ['ssr.spec.ts'],
      port,
      command:
        mode === 'dev'
          ? `pnpm -C sveltekit exec vite dev ${host(port)}`
          : `pnpm -C sveltekit exec vite build --logLevel warn && node sveltekit/build`,
      env: { PORT: String(port), HOST: '127.0.0.1' },
      timeout: 180_000,
    };
  }),
];

const only = process.env.HOLYFLOE_E2E;
const selected = apps.filter((a) => !only || a.framework === only);

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? 'github' : 'list',
  projects: selected.map((a) => ({
    name: a.name,
    testMatch: a.testMatch,
    use: { ...devices['Desktop Chrome'], baseURL: `http://127.0.0.1:${a.port}` },
  })),
  webServer: selected.map((a) => ({
    command: a.command,
    url: `http://127.0.0.1:${a.port}`,
    env: a.env,
    reuseExistingServer: false,
    timeout: a.timeout ?? 120_000,
  })),
});
