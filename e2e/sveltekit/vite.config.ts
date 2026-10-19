import adapter from '@sveltejs/adapter-node';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// SvelteKit 3 SSR app rendering the Svelte wrappers from fixtures/out/svelte (tests/ssr.spec.ts).
// Kit 3 takes its config here; `svelte.config.js` is no longer read.
export default defineConfig({
  plugins: [sveltekit({ adapter: adapter() })],
  resolve: { dedupe: ['svelte', '@holyfloe/fixtures'] },
});
