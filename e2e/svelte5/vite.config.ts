import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

// The generated wrappers live outside the app dir, so `dedupe` resolves their imports from the
// app, as in e2e/shared/vite.config.ts.
export default defineConfig({
  plugins: [svelte()],
  resolve: { dedupe: ['svelte', '@holyfloe/fixtures'] },
});
