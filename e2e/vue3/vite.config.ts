import vue from '@vitejs/plugin-vue';
import { defineConfig } from 'vite';

// The generated wrappers live outside the app dir, so `dedupe` resolves their imports from the
// app, as in e2e/shared/vite.config.ts. Raw tags (OQ-003) need `isCustomElement`.
export default defineConfig({
  plugins: [vue({ template: { compilerOptions: { isCustomElement: (tag) => tag.startsWith('fx-') } } })],
  resolve: { dedupe: ['vue', '@holyfloe/fixtures'] },
});
