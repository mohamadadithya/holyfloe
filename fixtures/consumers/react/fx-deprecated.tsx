// Must compile: correct props, events, ref and children.
import { FxDeprecated } from '../../out/react/fx-deprecated/index.js';

export function Consumer() {
  return (
    <FxDeprecated tone="error" kind="legacy">
      Something failed
    </FxDeprecated>
  );
}
