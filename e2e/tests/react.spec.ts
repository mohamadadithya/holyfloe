// Wrapper-mode behavior per fixture, on React 18 and 19, dev and prod (see playwright.config.ts).
// The page is e2e/shared/App.tsx; element ids here match the ids there.
import { expect, test, type Page } from '@playwright/test';

const isReact18 = () => test.info().project.name.startsWith('react18');

// Any console error or warning fails the test: React reports bad props and double
// registrations as console errors.
let problems: string[];
test.beforeEach(async ({ page }) => {
  problems = [];
  page.on('console', (m) => {
    if (m.type() === 'warning' && m.text().includes('Lit is in dev mode')) return; // `vite` dev only
    if (m.type() === 'error' || m.type() === 'warning') problems.push(`${m.type()}: ${m.text()}`);
  });
  page.on('pageerror', (e) => problems.push(`pageerror: ${e.message}`));
  await page.goto('/');
  await page.waitForFunction(() => customElements.get('fx-derived') && window.refs);
});
test.afterEach(() => expect(problems).toEqual([]));

const el = (page: Page, id: string) => page.locator(`#${id}`);
const prop = (page: Page, id: string, name: string) =>
  page.evaluate(([id, name]) => (document.getElementById(id) as any)[name], [id, name]);
const logged = async (page: Page, prefix: RegExp) =>
  (await page.evaluate(() => window.log)).filter((line) => prefix.test(line));

test.describe('fx-button', () => {
  test('primitive and boolean props go out as attribute and property, and stay in sync', async ({ page }) => {
    await expect(el(page, 'btn')).toHaveAttribute('variant', 'danger');
    expect(await prop(page, 'btn', 'variant')).toBe('danger');
    await expect(el(page, 'btn')).toHaveAttribute('disabled', '');
    expect(await prop(page, 'btn', 'disabled')).toBe(true);

    await page.click('#toggle');
    await expect(el(page, 'btn')).not.toHaveAttribute('disabled');
    expect(await prop(page, 'btn', 'disabled')).toBe(false);

    // Back on: property true, attribute present, and Lit rendered it (step-4 review finding 5).
    await page.click('#toggle');
    await expect(el(page, 'btn')).toHaveAttribute('disabled', '');
    expect(await prop(page, 'btn', 'disabled')).toBe(true);
    await expect(el(page, 'btn').locator('button')).toBeDisabled();
  });

  test('an inline handler sees the latest closure and fires once per event', async ({ page }) => {
    await el(page, 'btn2').locator('button').click();
    await page.click('#rerender');
    await page.click('#rerender');
    await el(page, 'btn2').locator('button').click();
    expect(await logged(page, /^press:/)).toEqual(['press:0:FX-BUTTON', 'press:2:FX-BUTTON']);
  });

  test('ref is the custom element', async ({ page }) => {
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
    expect(await prop(page, 'badge', 'for')).toBe('inbox'); // destructured as `for: for_`
    await expect(badge).toHaveAttribute('for', 'inbox');

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
  test('default and named slots', async ({ page }) => {
    const assigned = await page.evaluate(() => {
      const root = document.getElementById('card')!.shadowRoot!;
      const tags = (selector: string) =>
        (root.querySelector(selector) as HTMLSlotElement)
          .assignedNodes({ flatten: true })
          .filter((n): n is Element => n.nodeType === Node.ELEMENT_NODE)
          .map((n) => n.tagName);
      return { header: tags('slot[name=header]'), body: tags('slot:not([name])'), footer: tags('slot[name=footer]') };
    });
    expect(assigned).toEqual({ header: ['H2'], body: ['P'], footer: ['SPAN'] });
  });

  test('style takes CSS custom properties next to normal ones', async ({ page }) => {
    await expect(el(page, 'card')).toHaveAttribute('elevated', '');
    await expect(el(page, 'card').locator('[part=base]')).toHaveCSS('padding-top', '3px');
    await expect(el(page, 'card')).toHaveCSS('color', 'rgb(255, 0, 0)');
  });

  test('ref works on a default-exported class', async ({ page }) => {
    expect(await page.evaluate(() => window.refs.card instanceof customElements.get('fx-card')!)).toBe(true);
  });
});

test.describe('fx-derived', () => {
  test("an event fired in Lit's first update is not lost, and fires once", async ({ page }) => {
    // The wrapper listens in a layout effect; a passive effect would miss this event.
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
    await expect(el(page, 'dep')).toHaveAttribute('tone', 'error'); // reflected
    expect(await prop(page, 'dep', 'kind')).toBe('x');
    // React 19 sets props the element already has as properties on the client, so a
    // non-reflected prop gets no attribute; React 18 writes the attribute (emit-react.md rule 5).
    if (isReact18()) await expect(el(page, 'dep')).toHaveAttribute('kind', 'x');
    else await expect(el(page, 'dep')).not.toHaveAttribute('kind');
  });
});

test.describe('types mode (React 19 only)', () => {
  test.beforeEach(() => test.skip(isReact18(), 'types mode needs React 19'));

  test('`on` + exact DOM event name attaches a listener for every casing', async ({ page }) => {
    await el(page, 'native-events').locator('button').click();
    expect(await logged(page, /^native /)).toEqual([
      'native my-change:a',
      'native myChange:1',
      'native MyChange:true',
      'native MYEVENT:4',
    ]);
  });

  test('props become properties', async ({ page }) => {
    expect(await prop(page, 'native-badge', 'count')).toBe(5);
    await expect(el(page, 'native-badge').locator('span')).toHaveText('5');
  });
});
