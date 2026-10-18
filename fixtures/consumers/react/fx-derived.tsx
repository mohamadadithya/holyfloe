// Must compile: correct props, events, ref and children.
import { FxDerived } from '../../out/react/fx-derived/index.js';

export function Consumer() {
  return (
    <FxDerived label="Hi" size="lg" loading onFxReady={(e) => void e.currentTarget.label}>
      content
    </FxDerived>
  );
}
