// Each marked line must fail to type-check.
import { FxCard } from '../../out/react/fx-card/index.js';

// @ts-expect-error CSS custom properties are strings
export const BadVar = <FxCard style={{ '--fx-card-padding': 4 }} />;

// @ts-expect-error undeclared custom properties are rejected
export const UnknownVar = <FxCard style={{ '--fx-card-shadow': '1px' }} />;

// @ts-expect-error elevated is boolean
export const BadElevated = <FxCard elevated="yes" />;
