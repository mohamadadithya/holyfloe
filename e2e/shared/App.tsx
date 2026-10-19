// Renders every fixture through the wrappers `just gen-fixtures` wrote to fixtures/out/react.
// Each block below has an `id` that tests/react.spec.ts looks up; keep the two in step.
import { StrictMode, useEffect, useRef, useState, type ReactNode } from 'react';
import { createRoot } from 'react-dom/client';

import { FxBadge } from '../../fixtures/out/react/fx-badge/index.js';
import { FxButton } from '../../fixtures/out/react/fx-button/index.js';
import { FxCard } from '../../fixtures/out/react/fx-card/index.js';
import { FxDeprecated } from '../../fixtures/out/react/fx-deprecated/index.js';
import { FxDerived } from '../../fixtures/out/react/fx-derived/index.js';
import { FxDialog } from '../../fixtures/out/react/fx-dialog/index.js';
import { FxEvents } from '../../fixtures/out/react/fx-events/index.js';
import { FxInput } from '../../fixtures/out/react/fx-input/index.js';
import { FxLazy } from '../../fixtures/out/react/fx-lazy/index.js';
import { FxList } from '../../fixtures/out/react/fx-list/index.js';
import { FxMixin } from '../../fixtures/out/react/fx-mixin/index.js';
import { FxSelect } from '../../fixtures/out/react/fx-select/index.js';
import { defineFxLazy } from '@holyfloe/fixtures/fx-lazy/fx-lazy.js';
import { log } from './log.ts';

function App({ extra }: { extra?: ReactNode }) {
  const [renders, setRenders] = useState(0);
  const [disabled, setDisabled] = useState(true);
  const [count, setCount] = useState(3);
  const [text, setText] = useState('start');
  const [open, setOpen] = useState(false);
  const [choice, setChoice] = useState('b');
  const buttonRef = useRef<HTMLElement>(null);
  const cardRef = useRef<HTMLElement>(null);
  const dialogRef = useRef<HTMLElementTagNameMap['fx-dialog']>(null);
  useEffect(() => {
    window.refs = { button: buttonRef.current, card: cardRef.current, dialog: dialogRef.current };
  });

  return (
    <>
      {/* Controls the tests click. */}
      <button id="rerender" onClick={() => setRenders((n) => n + 1)}>rerender {renders}</button>
      <button id="toggle" onClick={() => setDisabled((d) => !d)}>toggle disabled</button>
      <button id="inc" onClick={() => setCount((c) => c + 1)}>increment count</button>
      <button id="define-lazy" onClick={defineFxLazy}>define fx-lazy</button>
      <button id="open-dialog" onClick={() => setOpen(true)}>open dialog</button>
      <button id="show-dialog" onClick={() => dialogRef.current?.show()}>show() via ref</button>

      {/* fx-button: primitive + boolean props, ref, and an inline handler that closes over `renders`. */}
      <FxButton id="btn" ref={buttonRef as never} variant="danger" disabled={disabled}>Go</FxButton>
      <FxButton id="btn2" onFxPress={(e) => log(`press:${renders}:${e.currentTarget.tagName}`)}>Click</FxButton>

      {/* fx-badge: renamed attribute (count -> badge-count), property-only `label`, reserved word `for`. */}
      <FxBadge id="badge" count={count} tone="success" label="3 unread" for="inbox">new</FxBadge>

      {/* fx-list: complex props (array, object, function) set as properties. */}
      <FxList
        id="list"
        items={[{ id: 'a', label: 'Alpha' }, { id: 'b', label: 'Beta' }, { id: 'c', label: 'Gamma' }]}
        options={{ dense: true, maxItems: 2 }}
        renderLabel={(item) => item.label.toUpperCase()}
      />

      {/* fx-events: four event casings; myChange and MyChange are renamed by its holyfloe.config.json. */}
      <FxEvents
        id="events"
        onMyChange={(e) => log(`my-change:${e.detail.value}`)}
        onMyChangeCamel={(e) => log(`myChange:${e.detail.value}`)}
        onMyChangePascal={(e) => log(`MyChange:${e.detail.checked}`)}
        onMYEVENT={(e) => log(`MYEVENT:${e.detail.count}`)}
      />

      {/* fx-card: default-exported class, default + named slots, CSS custom property in `style`. */}
      <FxCard id="card" ref={cardRef as never} elevated style={{ '--fx-card-padding': '3px', color: 'red' }}>
        <h2 slot="header">Head</h2>
        <p>Body</p>
        <span slot="footer">Foot</span>
      </FxCard>

      {/* fx-derived: inherited props, and `fx-ready` fired during Lit's first update. */}
      <FxDerived id="derived" label="L" size="lg" loading onFxReady={(e) => log(`ready:${e.currentTarget.tagName}`)}>
        kid
      </FxDerived>

      {/* fx-deprecated: a deprecated component still works. */}
      <FxDeprecated id="dep" tone="error" kind="x" />

      {/* fx-input: value + change wired by hand; React has no two-way helper (D-067). */}
      <FxInput id="input" value={text} name="q" onChange={(e) => setText(e.target.value)} />
      <output id="input-state">{text}</output>

      {/* fx-dialog: open + open-change, and show() through the ref. */}
      <FxDialog id="dialog" ref={dialogRef as never} open={open} onOpenChange={(e) => setOpen(e.detail.open)}>
        Dialog body
        <button slot="footer">OK</button>
      </FxDialog>
      <output id="dialog-state">{String(open)}</output>

      {/* fx-select: the new value arrives in detail.value. */}
      <FxSelect id="select" value={choice} options={['a', 'b', 'c']} onFxChange={(e) => setChoice(e.detail.value)} />
      <output id="select-state">{choice}</output>

      {/* fx-mixin: props and event from a mixin in another package. */}
      <FxMixin id="mixin" prefix="> " focused ringColor="red" onFocusChange={(e) => log(`focus:${e.currentTarget.focused}`)}>
        Mixed
      </FxMixin>

      {/* fx-lazy: rendered before `fx-lazy` is defined (#define-lazy). */}
      <FxLazy id="lazy" items={['x', 'y']} label="Lazy" active />

      {extra}
    </>
  );
}

/** StrictMode on purpose: in dev it double-runs effects, which must not double listeners. */
export function mount(extra?: ReactNode) {
  createRoot(document.getElementById('root')!).render(
    <StrictMode>
      <App extra={extra} />
    </StrictMode>,
  );
}
