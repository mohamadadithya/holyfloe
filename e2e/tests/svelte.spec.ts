// Svelte without the wrappers (e2e/svelte5/src/Raw.svelte, `?raw`): closes OQ-004 (D-077).
import { expect, test } from '@playwright/test';

const items = (page: import('@playwright/test').Page, id: string) =>
  page.evaluate((id) => {
    const value = (document.getElementById(id) as any).items;
    return typeof value === 'function' ? 'function' : value;
  }, id);

test.beforeEach(async ({ page }) => {
  await page.goto('/?raw');
});

test('OQ-004: before definition Svelte sets objects as properties, which survive the upgrade', async ({ page }) => {
  expect(await page.evaluate(() => customElements.get('fx-lazy'))).toBeUndefined();
  // Not a registered element yet: Svelte's heuristic is "an object, then a property".
  await expect(page.locator('#raw-lazy')).not.toHaveAttribute('items');
  expect(await items(page, 'raw-lazy')).toEqual(['p', 'q']);
  await page.click('#define-lazy');
  await expect(page.locator('#raw-lazy li')).toHaveText(['p', 'q']); // Lit kept the instance property
});

test('OQ-004: before definition a function becomes an attribute string and is lost', async ({ page }) => {
  await expect(page.locator('#raw-lazy-fn')).toHaveAttribute('items', /=>/);
  await page.click('#define-lazy');
  await expect(page.locator('#raw-lazy-fn [part=label]')).toHaveText('Fn');
  expect(await items(page, 'raw-lazy-fn')).toEqual([]);
});
