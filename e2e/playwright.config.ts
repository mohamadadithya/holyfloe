import { defineConfig, devices } from '@playwright/test';

// Vite apps (tests/react.spec.ts): each React version under `vite` dev (React dev build,
// StrictMode double effects, Lit dev mode) and under `vite build` + `vite preview`.
const viteApps = [
  { react: 18, mode: 'dev', port: 5181 },
  { react: 18, mode: 'prod', port: 5182 },
  { react: 19, mode: 'dev', port: 5191 },
  { react: 19, mode: 'prod', port: 5192 },
] as const;

// Next 16 App Router (tests/next.spec.ts): `next dev` and `next build` + `next start`. Next 16
// keeps dev output in .next/dev, so both can run at once.
const nextApps = [
  { mode: 'dev', port: 5201 },
  { mode: 'prod', port: 5202 },
] as const;

const host = (port: number) => `--port ${port} --strictPort --host 127.0.0.1`;
const chrome = (port: number) => ({ ...devices['Desktop Chrome'], baseURL: `http://127.0.0.1:${port}` });

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? 'github' : 'list',
  projects: [
    ...viteApps.map(({ react, mode, port }) => ({
      name: `react${react}-${mode}`,
      testMatch: 'react.spec.ts',
      use: chrome(port),
    })),
    ...nextApps.map(({ mode, port }) => ({
      name: `next-${mode}`,
      testMatch: 'next.spec.ts',
      use: chrome(port),
    })),
  ],
  webServer: [
    ...viteApps.map(({ react, mode, port }) => ({
      command:
        mode === 'dev'
          ? `pnpm exec vite react${react} ${host(port)}`
          : `pnpm exec vite build react${react} --logLevel warn && pnpm exec vite preview react${react} ${host(port)}`,
      url: `http://127.0.0.1:${port}`,
      reuseExistingServer: false,
      timeout: 120_000,
    })),
    ...nextApps.map(({ mode, port }) => {
      const next = `pnpm -C next-app exec next`;
      const at = `--port ${port} --hostname 127.0.0.1`;
      return {
        command: mode === 'dev' ? `${next} dev ${at}` : `${next} build && ${next} start ${at}`,
        url: `http://127.0.0.1:${port}`,
        env: { NEXT_TELEMETRY_DISABLED: '1' },
        reuseExistingServer: false,
        timeout: 180_000,
      };
    }),
  ],
});
