// Each marked line must fail to type-check.
import { FxList } from '../../out/react/fx-list/index.js';

// @ts-expect-error items need id and label strings
export const BadItems = <FxList items={[{ id: 1 }]} />;

// @ts-expect-error dense is boolean
export const BadOptions = <FxList options={{ dense: 'yes' }} />;

// @ts-expect-error renderLabel returns a string
export const BadRenderer = <FxList renderLabel={() => 42} />;
