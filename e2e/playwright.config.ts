import { defineConfig, devices } from '@playwright/test';

// Each app runs twice: `vite` dev (React dev build, StrictMode double effects, Lit dev mode) and
// `vite build` + `vite preview` (production bundles). One project and one server per pair.
const apps = [
  { react: 18, mode: 'dev', port: 5181 },
  { react: 18, mode: 'prod', port: 5182 },
  { react: 19, mode: 'dev', port: 5191 },
  { react: 19, mode: 'prod', port: 5192 },
] as const;

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? 'github' : 'list',
  projects: apps.map(({ react, mode, port }) => ({
    name: `react${react}-${mode}`,
    use: { ...devices['Desktop Chrome'], baseURL: `http://127.0.0.1:${port}` },
  })),
  webServer: apps.map(({ react, mode, port }) => {
    const serve = `--port ${port} --strictPort --host 127.0.0.1`;
    return {
      command:
        mode === 'dev'
          ? `pnpm exec vite react${react} ${serve}`
          : `pnpm exec vite build react${react} --logLevel warn && pnpm exec vite preview react${react} ${serve}`,
      url: `http://127.0.0.1:${port}`,
      reuseExistingServer: false,
      timeout: 120_000,
    };
  }),
});
