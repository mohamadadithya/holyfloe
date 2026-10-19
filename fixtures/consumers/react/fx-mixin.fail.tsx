// Each marked line must fail to type-check.
import { FxMixin } from '../../out/react/fx-mixin/index.js';

// @ts-expect-error focused (from the mixin) is a boolean
export const BadFocused = <FxMixin focused="yes" />;

// @ts-expect-error ringColor (from the mixin) is a string
export const BadRing = <FxMixin ringColor={1} />;
