// Must compile: correct props, events, ref and children.
// shortcut: imports expected/ until M1 emits fixtures/out; switch the path then.
import { useRef } from 'react';
import type { FxButton as FxButtonElement } from '@holyfloe/fixtures/fx-button/fx-button.js';
import { FxButton } from '../../expected/react/fx-button/index.js';

export function Consumer() {
  const ref = useRef<FxButtonElement>(null);
  return (
    <FxButton
      ref={ref}
      variant="danger"
      disabled
      className="cta"
      slot="actions"
      onClick={() => {}}
      onFxPress={(e) => {
        const el: FxButtonElement = e.currentTarget;
        const detail: unknown = e.detail;
        void el;
        void detail;
      }}
    >
      Save
    </FxButton>
  );
}
