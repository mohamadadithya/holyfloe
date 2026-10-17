// Each marked line must fail to type-check.
// shortcut: imports expected/ until M1 emits fixtures/out; switch the path then.
import { FxList } from '../../expected/react/fx-list/index.js';

// @ts-expect-error items need id and label strings
export const BadItems = <FxList items={[{ id: 1 }]} />;

// @ts-expect-error dense is boolean
export const BadOptions = <FxList options={{ dense: 'yes' }} />;

// @ts-expect-error renderLabel returns a string
export const BadRenderer = <FxList renderLabel={() => 42} />;
