// Must compile: correct props, events, ref and children.
// shortcut: imports expected/ until M1 emits fixtures/out; switch the path then.
import { FxDeprecated } from '../../expected/react/fx-deprecated/index.js';

export function Consumer() {
  return (
    <FxDeprecated tone="error" kind="legacy">
      Something failed
    </FxDeprecated>
  );
}
