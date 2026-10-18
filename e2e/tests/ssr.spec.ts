// Nuxt 4 and SvelteKit 3: server HTML of the wrappers, then hydration (e2e/nuxt/app/app.vue,
// e2e/sveltekit/src/routes/+page.svelte; same ids). The counterpart of tests/next.spec.ts.
import { expect, test } from '@playwright/test';

/** Attributes of the opening tag of `#id` in the raw server HTML; `''` for a valueless one. */
function attrs(html: string, id: string): Record<string, string> {
  const tag = html.match(new RegExp(`<fx-[a-z-]+ [^>]*id="${id}"[^>]*>`))?.[0];
  expect(tag, `#${id} in the server HTML`).toBeTruthy();
  const out: Record<string, string> = {};
  for (const [, name, value] of tag!.matchAll(/ ([^\s=>]+)(?:="([^"]*)")?/g)) out[name] = value ?? '';
  delete out.id;
  return out;
}

test('server HTML: primitive and boolean props are attributes, complex and property-only are not', async ({ request }) => {
  const html = await (await request.get('/')).text();
  // Vue writes a boolean attribute bare; Svelte writes `=""` for HTML boolean names and `="true"`
  // otherwise. Lit reads presence either way.
  const on = (v: string | undefined) => v === '' || v === 'true';
  const btn = attrs(html, 'ssr-btn');
  expect(btn.variant).toBe('danger');
  expect(on(btn.disabled)).toBe(true);
  expect(attrs(html, 'client-btn')).toEqual({});
  expect(attrs(html, 'ssr-badge')).toEqual({ 'badge-count': '3', tone: 'success', for: 'inbox' }); // no `label` (property only)
  expect(attrs(html, 'ssr-list')).toEqual({}); // complex props never render
  const card = attrs(html, 'ssr-card');
  expect(on(card.elevated)).toBe(true);
  expect(card.style).toMatch(/--fx-card-padding: ?3px/);
  expect(attrs(html, 'ssr-derived')).toEqual({ label: 'L', size: 'lg' });
  expect(attrs(html, 'ssr-input')).toEqual({ value: 'start', name: 'q' });
  expect(on(attrs(html, 'ssr-dialog').open)).toBe(true);
  const mixin = attrs(html, 'ssr-mixin');
  expect(mixin['ring-color']).toBe('red');
  expect(on(mixin.focused)).toBe(true);
  // The named slot wrapper is server-rendered too (Svelte adds `<!---->` hydration markers).
  expect(html.replace(/<!--.*?-->/g, '')).toMatch(/<span slot="header" style="display: contents"><h2>Head<\/h2><\/span>/);
  // No stringified values, `undefined`, listener keys or forced-property keys on any element.
  const tags = html.match(/<fx-[^>]*>/g)!.join('\n');
  expect(tags).not.toMatch(/\[object |"undefined"|on:|\.items|\.options/);
});

test('hydrates with no console errors or warnings, then sets properties, events and bindings work', async ({ page }) => {
  const problems: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'warning' && m.text().includes('Lit is in dev mode')) return; // dev servers only
    if (m.type() === 'error' || m.type() === 'warning') problems.push(`${m.type()}: ${m.text()}`);
  });
  page.on('pageerror', (e) => problems.push(`pageerror: ${e.message}`));
  await page.goto('/');

  // Complex props exist only on the client, so this waits for hydration.
  await expect(page.locator('#ssr-list li')).toHaveText(['Alpha']);
  const props = await page.evaluate(() => {
    const el = (id: string) => document.getElementById(id) as any;
    return { disabled: el('ssr-btn').disabled, count: el('ssr-badge').count, label: el('ssr-badge').label, open: el('ssr-dialog').open };
  });
  expect(props).toEqual({ disabled: true, count: 3, label: '3 unread', open: true });

  await page.locator('#client-btn button').click();
  await expect(page.locator('#presses')).toHaveText('1');

  const native = page.locator('#ssr-input input');
  await native.fill('hydrated');
  await native.press('Enter');
  await expect(page.locator('#ssr-input-state')).toHaveText('hydrated');

  await page.locator('#ssr-dialog [part=close]').click();
  await expect(page.locator('#ssr-dialog-state')).toHaveText('false');
  expect(problems).toEqual([]);
});
