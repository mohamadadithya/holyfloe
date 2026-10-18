// Must compile on React 19: `react.ref: "prop"` takes `ref` as a plain prop, no forwardRef.
import { useRef } from 'react';
import type { FxButton as FxButtonElement } from '@holyfloe/fixtures/fx-button/fx-button.js';
import { FxButton } from '../../out/react-ref-prop/fx-button/index.js';

export function Consumer() {
  const ref = useRef<FxButtonElement>(null);
  const callbackRef = (el: FxButtonElement | null) => void el;
  return (
    <>
      <FxButton ref={ref} variant="danger" disabled onFxPress={(e) => void e.currentTarget.variant}>
        Save
      </FxButton>
      <FxButton ref={callbackRef}>Cancel</FxButton>
    </>
  );
}
