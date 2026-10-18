// Each marked line must fail to type-check.
import { FxDerived } from '../../out/react/fx-derived/index.js';

// @ts-expect-error inherited size is a closed union
export const BadSize = <FxDerived size="xl" />;

// @ts-expect-error label is a string
export const BadLabel = <FxDerived label={1} />;
