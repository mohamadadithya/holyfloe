// Each marked line must fail to type-check.
import { FxInput } from '../../out/react/fx-input/index.js';

// @ts-expect-error value is a string
export const BadValue = <FxInput value={1} />;

// @ts-expect-error the change event target is the element, whose value is a string
export const BadTarget = <FxInput onChange={(e) => e.target.value.toFixed()} />;
