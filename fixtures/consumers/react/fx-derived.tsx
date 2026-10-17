// Must compile: correct props, events, ref and children.
// shortcut: imports expected/ until M1 emits fixtures/out; switch the path then.
import { FxDerived } from '../../expected/react/fx-derived/index.js';

export function Consumer() {
  return (
    <FxDerived label="Hi" size="lg" loading onFxReady={(e) => void e.currentTarget.label}>
      content
    </FxDerived>
  );
}
