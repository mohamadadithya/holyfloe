// Each marked line must fail to type-check.
// shortcut: imports expected/ until M1 emits fixtures/out; switch the path then.
import { FxCard } from '../../expected/react/fx-card/index.js';

// @ts-expect-error CSS custom properties are strings
export const BadVar = <FxCard style={{ '--fx-card-padding': 4 }} />;

// @ts-expect-error undeclared custom properties are rejected
export const UnknownVar = <FxCard style={{ '--fx-card-shadow': '1px' }} />;

// @ts-expect-error elevated is boolean
export const BadElevated = <FxCard elevated="yes" />;
