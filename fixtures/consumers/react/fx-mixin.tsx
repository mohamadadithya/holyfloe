// Must compile: own prop plus the props and event from the dependency mixin.
import { FxMixin } from '../../out/react/fx-mixin/index.js';

export const Ok = (
  <FxMixin prefix="> " focused ringColor="red" onFocusChange={(e) => void e.target.toggleFocus}>
    Label
  </FxMixin>
);
