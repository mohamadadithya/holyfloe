// Vue and Svelte wrapper behavior per fixture, dev and prod (see playwright.config.ts). The pages
// are e2e/vue3/src/App.vue and e2e/svelte5/src/App.svelte; element ids here match the ids there.
import { expect, test, type Page } from '@playwright/test';

const isSvelte = () => test.info().project.name.startsWith('svelte');

// Any console error or warning fails the test (Vue and Svelte report bad props and binding
// misuse as warnings in dev).
let problems: string[];
test.beforeEach(async ({ page }) => {
  problems = [];
  page.on('console', (m) => {
    if (m.type() === 'warning' && m.text().includes('Lit is in dev mode')) return; // `vite` dev only
    if (m.type() === 'error' || m.type() === 'warning') problems.push(`${m.type()}: ${m.text()}`);
  });
  page.on('pageerror', (e) => problems.push(`pageerror: ${e.message}`));
  await page.goto('/');
  await page.waitForFunction(() => customElements.get('fx-derived') && window.refs?.button);
});
test.afterEach(() => expect(problems).toEqual([]));

const el = (page: Page, id: string) => page.locator(`#${id}`);
const prop = (page: Page, id: string, name: string) =>
  page.evaluate(([id, name]) => (document.getElementById(id) as any)[name], [id, name]);
const logged = async (page: Page, prefix: RegExp) =>
  (await page.evaluate(() => window.log)).filter((line) => prefix.test(line));

test.describe('fx-button', () => {
  test('primitive and boolean props reach attribute and property, and stay in sync', async ({ page }) => {
    await expect(el(page, 'btn')).toHaveAttribute('variant', 'danger');
    expect(await prop(page, 'btn', 'disabled')).toBe(true);
    await expect(el(page, 'btn')).toHaveAttribute('disabled', '');

    await page.click('#toggle');
    expect(await prop(page, 'btn', 'disabled')).toBe(false); // false, not null/undefined (D-074)
    await expect(el(page, 'btn')).not.toHaveAttribute('disabled');

    await page.click('#toggle');
    expect(await prop(page, 'btn', 'disabled')).toBe(true);
    await expect(el(page, 'btn').locator('button')).toBeDisabled();

    // Unsetting removes the attribute; Lit then holds null (Vue) or undefined (Svelte), not the default.
    await page.click('#unset-variant');
    await expect(el(page, 'btn')).not.toHaveAttribute('variant');
    expect(await prop(page, 'btn', 'variant')).toBe(isSvelte() ? undefined : null);
  });

  test('unset props keep the element defaults; attributes and native listeners fall through', async ({ page }) => {
    expect(await prop(page, 'plain', 'variant')).toBe('primary');
    expect(await prop(page, 'plain', 'disabled')).toBe(false);
    await expect(el(page, 'plain')).toHaveClass('extra');
    await expect(el(page, 'plain')).toHaveAttribute('data-x', '1');
    await el(page, 'plain').locator('button').click();
    expect(await logged(page, /^native/)).toEqual(['native click']);
  });

  test('an inline handler sees the latest closure and fires once per event', async ({ page }) => {
    await el(page, 'btn2').locator('button').click();
    await page.click('#rerender');
    await page.click('#rerender');
    await el(page, 'btn2').locator('button').click();
    expect(await logged(page, /^press:/)).toEqual(['press:0:FX-BUTTON', 'press:2:FX-BUTTON']);
  });

  test('element access is the custom element', async ({ page }) => {
    expect(await page.evaluate(() => window.refs.button === document.getElementById('btn'))).toBe(true);
    expect(await page.evaluate(() => window.refs.button instanceof customElements.get('fx-button')!)).toBe(true);
  });
});

test.describe('fx-badge', () => {
  test('renamed attribute, property-only prop, reserved-word prop, and updates', async ({ page }) => {
    const badge = el(page, 'badge');
    await expect(badge).toHaveAttribute('badge-count', '3');
    await expect(badge).not.toHaveAttribute('count');
    expect(await prop(page, 'badge', 'count')).toBe(3);
    await expect(badge).toHaveAttribute('tone', 'success');
    expect(await prop(page, 'badge', 'label')).toBe('3 unread');
    await expect(badge).not.toHaveAttribute('label'); // Lit `attribute: false`
    expect(await prop(page, 'badge', 'for')).toBe('inbox');

    await page.click('#inc');
    await expect(badge).toHaveAttribute('badge-count', '4');
    expect(await prop(page, 'badge', 'count')).toBe(4);
    await expect(badge.locator('span')).toHaveText('4');
  });
});

test.describe('fx-list', () => {
  test('complex props are properties, never attributes', async ({ page }) => {
    await expect(el(page, 'list').locator('li')).toHaveText(['ALPHA', 'BETA']); // renderLabel + maxItems
    expect(await prop(page, 'list', 'options')).toEqual({ dense: true, maxItems: 2 });
    for (const name of ['items', 'options', 'renderlabel']) {
      await expect(el(page, 'list')).not.toHaveAttribute(name);
    }
  });
});

test.describe('fx-events', () => {
  test('all four casings reach their handler once, with typed detail', async ({ page }) => {
    await el(page, 'events').locator('button').click();
    expect(await logged(page, /^(my-change|myChange|MyChange|MYEVENT):/)).toEqual([
      'my-change:a',
      'myChange:1',
      'MyChange:true',
      'MYEVENT:4',
    ]);
  });
});

test.describe('fx-card', () => {
  test('named slots go through a display: contents wrapper carrying the slot attribute', async ({ page }) => {
    const assigned = await page.evaluate(() => {
      const root = document.getElementById('card')!.shadowRoot!;
      const nodes = (selector: string) =>
        (root.querySelector(selector) as HTMLSlotElement)
          .assignedElements()
          .map((n) => `${n.tagName}>${[...n.children].map((c) => c.tagName).join(',')}:${getComputedStyle(n).display}`);
      return { header: nodes('slot[name=header]'), body: nodes('slot:not([name])'), footer: nodes('slot[name=footer]') };
    });
    expect(assigned).toEqual({
      header: ['SPAN>H2:contents'],
      body: ['P>:block'],
      footer: ['SPAN>SPAN:contents'],
    });
  });

  test('an unused named slot gets no wrapper', async ({ page }) => {
    await expect(el(page, 'card-plain').locator('[slot]')).toHaveCount(0);
  });

  test('style passes through, CSS custom properties included', async ({ page }) => {
    await expect(el(page, 'card')).toHaveAttribute('elevated', '');
    await expect(el(page, 'card').locator('[part=base]')).toHaveCSS('padding-top', '3px');
    await expect(el(page, 'card')).toHaveCSS('color', 'rgb(255, 0, 0)');
  });

  test('element access works on a default-exported class', async ({ page }) => {
    expect(await page.evaluate(() => window.refs.card instanceof customElements.get('fx-card')!)).toBe(true);
  });
});

test.describe('fx-derived', () => {
  test("an event fired in Lit's first update is not lost, and fires once", async ({ page }) => {
    expect(await logged(page, /^ready:/)).toEqual(['ready:FX-DERIVED']);
  });

  test('inherited props and children', async ({ page }) => {
    await expect(el(page, 'derived')).toHaveAttribute('size', 'lg');
    expect(await prop(page, 'derived', 'loading')).toBe(true);
    expect(await prop(page, 'derived', 'label')).toBe('L');
    await expect(el(page, 'derived')).toContainText('kid');
  });
});

test.describe('fx-deprecated', () => {
  test('a deprecated component still renders its props', async ({ page }) => {
    await expect(el(page, 'dep')).toHaveAttribute('tone', 'error');
    expect(await prop(page, 'dep', 'kind')).toBe('x');
  });
});

test.describe('fx-input', () => {
  test('two-way binding round trip, through a change event that does not bubble', async ({ page }) => {
    expect(await prop(page, 'input', 'value')).toBe('start');
    const native = el(page, 'input').locator('input');
    await native.fill('hello');
    await native.press('Enter');
    await expect(el(page, 'input-state')).toHaveText('hello');
    expect(await prop(page, 'input', 'value')).toBe('hello');
    await expect(el(page, 'input')).toHaveAttribute('name', 'q');
  });
});

test.describe('fx-dialog', () => {
  test('two-way open round trip (from detail.open), and show() through the instance or element', async ({ page }) => {
    await expect(el(page, 'dialog')).not.toHaveAttribute('open');
    await page.click('#open-dialog');
    await expect(el(page, 'dialog')).toHaveAttribute('open', '');
    await el(page, 'dialog').locator('[part=close]').click();
    await expect(el(page, 'dialog-state')).toHaveText('false');
    await expect(el(page, 'dialog')).not.toHaveAttribute('open');
    await page.click('#show-dialog');
    await expect(el(page, 'dialog-state')).toHaveText('true');
    await expect(el(page, 'dialog')).toHaveAttribute('open', '');
  });
});

test.describe('fx-select', () => {
  test('two-way binding from detail.value', async ({ page }) => {
    await expect(el(page, 'select')).toHaveAttribute('value', 'b');
    await el(page, 'select').locator('[part=option]', { hasText: 'c' }).click();
    await expect(el(page, 'select-state')).toHaveText('c');
    await expect(el(page, 'select')).toHaveAttribute('value', 'c');
  });
});

test.describe('fx-mixin', () => {
  test('props, method and event from the dependency mixin', async ({ page }) => {
    await expect(el(page, 'mixin')).toHaveAttribute('focused', '');
    await expect(el(page, 'mixin')).toHaveAttribute('ring-color', 'red');
    expect(await prop(page, 'mixin', 'prefix')).toBe('> ');
    await page.click('#toggle-focus');
    expect(await logged(page, /^focus:/)).toEqual(['focus:false']);
  });
});

test.describe('fx-lazy', () => {
  test('props set before the element is defined arrive after it upgrades (no whenDefined re-apply)', async ({ page }) => {
    expect(await page.evaluate(() => customElements.get('fx-lazy'))).toBeUndefined();
    await expect(el(page, 'lazy')).toHaveAttribute('label', 'Lazy');
    await expect(el(page, 'lazy')).not.toHaveAttribute('items');
    await page.click('#define-lazy');
    await expect(el(page, 'lazy').locator('li')).toHaveText(['x', 'y']);
    expect(await prop(page, 'lazy', 'items')).toEqual(['x', 'y']);
    expect(await prop(page, 'lazy', 'label')).toBe('Lazy');
    expect(await prop(page, 'lazy', 'active')).toBe(true);
  });
});
