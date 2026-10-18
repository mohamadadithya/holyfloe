// Each marked line must fail to type-check.
import { FxDeprecated } from '../../out/react/fx-deprecated/index.js';

// @ts-expect-error tone is a closed union
export const BadTone = <FxDeprecated tone="warn" />;

// @ts-expect-error protected members are not props
export const Protected = <FxDeprecated busy />;

// @ts-expect-error static members are not props
export const Static = <FxDeprecated defaultTone="info" />;
