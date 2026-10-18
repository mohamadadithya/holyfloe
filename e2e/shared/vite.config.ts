import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// Shared by e2e/react18 and e2e/react19. The app and the generated wrappers live outside the
// app dir, so `dedupe` resolves their imports from the app: its own React version, and
// @holyfloe/fixtures through that package's `exports` (dist/), as a real consumer would.
export default defineConfig({
  plugins: [react()],
  resolve: { dedupe: ['react', 'react-dom', '@holyfloe/fixtures'] },
});
