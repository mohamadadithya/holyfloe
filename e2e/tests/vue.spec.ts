// Vue without the wrappers (e2e/vue3/src/Raw.vue, `?raw`): closes OQ-003 (D-076).
import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/?raw');
  await page.waitForFunction(() => customElements.get('fx-input'));
});

test('OQ-003: raw v-model follows typing, because fx-input fires a composed `input` and updates `value` first', async ({ page }) => {
  // Vue compiles v-model on a custom element to its text-input directive: `input` + `el.value`.
  // An element that updates `value` later, or signals through another event or detail, would not
  // work; and vue-tsc rejects raw v-model (it types it as a `modelValue` prop). Use the wrapper.
  await page.locator('#raw-input input').fill('typed');
  await expect(page.locator('#raw-input-state')).toHaveText('typed');
});

test('OQ-003: raw tags lose a complex prop set before the element is defined', async ({ page }) => {
  // Vue sets a property only when the element has it (`key in el`), so before the upgrade the
  // array becomes the attribute "p,q", and after it the property is Lit's default.
  await expect(page.locator('#raw-lazy')).toHaveAttribute('items', 'p,q');
  await page.click('#define-lazy');
  await expect(page.locator('#raw-lazy [part=label]')).toHaveText('Raw');
  expect(await page.evaluate(() => (document.getElementById('raw-lazy') as any).items)).toEqual([]);
});
