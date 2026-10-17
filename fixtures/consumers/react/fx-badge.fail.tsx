// Each marked line must fail to type-check.
// shortcut: imports expected/ until M1 emits fixtures/out; switch the path then.
import { FxBadge } from '../../expected/react/fx-badge/index.js';

// @ts-expect-error count is a number
export const BadCount = <FxBadge count="3" />;

// @ts-expect-error tone is a closed union
export const BadTone = <FxBadge tone="loud" />;

// @ts-expect-error the attribute name is not a prop
export const AttrName = <FxBadge badgeCount={3} />;

// @ts-expect-error overflow is a read-only getter, so it is not a prop
export const ReadOnly = <FxBadge overflow />;
