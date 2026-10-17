// Each marked line must fail to type-check.
// shortcut: imports expected/ until M1 emits fixtures/out; switch the path then.
import { FxDerived } from '../../expected/react/fx-derived/index.js';

// @ts-expect-error inherited size is a closed union
export const BadSize = <FxDerived size="xl" />;

// @ts-expect-error label is a string
export const BadLabel = <FxDerived label={1} />;
