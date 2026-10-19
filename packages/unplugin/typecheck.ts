// Type test for the published declarations of @holyfloe/unplugin and @holyfloe/core.
import { generate, watch, type FrameworkReport } from '@holyfloe/core';
import holyfloe from '@holyfloe/unplugin';

holyfloe.vite({ cem: 'a.json', out: 'out', frameworks: ['react', 'vue'], reactMode: 'types' });
holyfloe.vite({ lit: 'src', out: 'out', frameworks: ['svelte'] });
// @ts-expect-error unknown framework
holyfloe.vite({ cem: 'a.json', out: 'out', frameworks: ['angular'] });

const reports: FrameworkReport[] = generate({ cem: 'a.json', out: 'out', frameworks: ['svelte'] });
const w = watch({ cem: 'a.json', out: 'out', frameworks: ['react'] }, (err, runs) => {
  const message: string | undefined = err?.message;
  const written: string[] = runs.flatMap((r) => r.written);
  void [message, written, reports];
});
w.close();
