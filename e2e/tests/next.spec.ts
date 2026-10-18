// Next 16 App Router: a server component (e2e/next-app/app/page.tsx) renders the 'use client'
// wrappers. Checks the server HTML, then hydration and a client-side event.
import { expect, test } from '@playwright/test';

/** The opening tag of `#id` in the raw server HTML. */
const startTag = (html: string, id: string) => html.match(new RegExp(`<fx-[a-z-]+ id="${id}"[^>]*>`))?.[0];

test('server HTML: primitive and boolean props are attributes, complex and property-only are not', async ({ request }) => {
  const html = await (await request.get('/')).text();
  expect(startTag(html, 'ssr-btn')).toBe('<fx-button id="ssr-btn" variant="danger" disabled="">');
  // Renamed attribute is right in SSR (types mode would write count="3"; emit-react.md).
  expect(startTag(html, 'ssr-badge')).toBe('<fx-badge id="ssr-badge" badge-count="3" tone="success" for="inbox">');
  expect(startTag(html, 'ssr-list')).toBe('<fx-list id="ssr-list">');
  expect(startTag(html, 'ssr-card')).toBe('<fx-card id="ssr-card" style="--fx-card-padding:3px" elevated="">');
  expect(startTag(html, 'ssr-derived')).toBe('<fx-derived id="ssr-derived" label="L" size="lg">');
  expect(html).not.toMatch(/suppresshydrationwarning/i);
});

test('hydrates with no console errors, then sets properties and handles events', async ({ page }) => {
  // #client-btn leaves `variant` unset, so Lit reflects `variant="primary"` before React hydrates;
  // `next dev` would report that without the wrappers' suppressHydrationWarning (D-052).
  const problems: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'warning' && m.text().includes('Lit is in dev mode')) return; // `next dev` only
    if (m.type() === 'error' || m.type() === 'warning') problems.push(`${m.type()}: ${m.text()}`);
  });
  page.on('pageerror', (e) => problems.push(`pageerror: ${e.message}`));
  await page.goto('/');

  // After hydration the client wrapper has set properties the server could not.
  await expect(page.locator('#ssr-list li')).toHaveText(['Alpha']);
  const props = await page.evaluate(() => {
    const el = (id: string) => document.getElementById(id) as any;
    return { disabled: el('ssr-btn').disabled, count: el('ssr-badge').count, label: el('ssr-badge').label };
  });
  expect(props).toEqual({ disabled: true, count: 3, label: '3 unread' });

  await page.locator('#client-btn button').click();
  await expect(page.locator('#presses')).toHaveText('1');
  expect(problems).toEqual([]);
});
