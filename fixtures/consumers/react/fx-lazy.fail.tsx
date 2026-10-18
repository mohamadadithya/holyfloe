// Each marked line must fail to type-check.
import { FxLazy } from '../../out/react/fx-lazy/index.js';

// @ts-expect-error items are strings
export const BadItems = <FxLazy items={[1]} />;
